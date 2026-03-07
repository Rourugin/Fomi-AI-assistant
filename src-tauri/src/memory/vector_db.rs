use arrow_array::{FixedSizeListArray, Float32Array, Int64Array, RecordBatch, StringArray, RecordBatchIterator};
use lancedb::{Table, query::{ExecutableQuery, QueryBase}};
use arrow_schema::{DataType, Field, Schema};
use std::{path::PathBuf, sync::Arc};
use futures::TryStreamExt;
use chrono::Utc;
use uuid::Uuid;
use sha256;


const EMBEDDING_DIM: i32 = 384;


#[derive(Debug, Clone)]
pub struct MemoryRecord {
    pub id: String,
    pub text: String,
    pub vector: Vec<f32>,
    pub created_at: i64,
    pub source: String,
}


pub struct FomiVectorStore {
    table: Table,
}

impl FomiVectorStore {
    pub async fn new(store_path: PathBuf) -> Result<FomiVectorStore, Box<dyn std::error::Error>> {
        let db = lancedb::connect(store_path.to_str().ok_or("Invalid path")?).execute().await?;
        let table_name = "memories";

        let table = if !db.table_names().execute().await?.contains(&table_name.to_string()) {
            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Utf8, false),
                Field::new("text", DataType::Utf8, false),
                Field::new("vector", DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    EMBEDDING_DIM
                ), false),
                Field::new("hash", DataType::Utf8, false),
                Field::new("created_at", DataType::Int64, false),
                Field::new("source", DataType::Utf8, false),
            ]));
            db.create_empty_table(table_name, schema).execute().await?
        } else {
            db.open_table(table_name).execute().await?
        };

        Ok(FomiVectorStore {
            table: table
        })
    }

    pub async fn add(&self, id: Uuid, text: &str, vector:Vec<f32>, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let created_at = Utc::now().timestamp();
        let current_hash = sha256::digest(text.to_string());

        let count_result: Vec<RecordBatch> = self.table
            .query()
            .only_if(format!("hash = '{}'", current_hash))
            .limit(1)
            .execute()
            .await?
            .try_collect::<Vec<RecordBatch>>()
            .await?;

        if !count_result.is_empty() && count_result[0].num_rows() > 0 {
            println!("Skipping duplicate: {}", text.chars().take(20).collect::<String>());
            return Ok(());
        }

        let id_array = StringArray::from(vec![id.to_string()]);
        let text_array = StringArray::from(vec![text]);
        let source_array = StringArray::from(vec![source]);
        let hash_array = StringArray::from(vec![current_hash]);
        let created_at_array = Int64Array::from(vec![created_at]);
        let value_array = Float32Array::from(vector);
        let list_field = Arc::new(Field::new("item", DataType::Float32, true));
        let vector_array = FixedSizeListArray::new(
            list_field, 
            EMBEDDING_DIM, 
            Arc::new(value_array), 
            None
        );

        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new("vector", DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                EMBEDDING_DIM
            ), false),
            Field::new("hash", DataType::Utf8, false),
            Field::new("created_at", DataType::Int64, false),
            Field::new("source", DataType::Utf8, false),
        ]));

        let batch = RecordBatch::try_new(schema.clone(),
            vec![
                Arc::new(id_array),
                Arc::new(text_array),
                Arc::new(vector_array),
                Arc::new(hash_array),
                Arc::new(created_at_array),
                Arc::new(source_array),
            ]
        )?;

        let batch_iter = RecordBatchIterator::new(vec![Ok(batch)], schema.clone());
        self.table.add(Box::new(batch_iter)).execute().await?;
        Ok(())
    }

    pub async fn search(&self, query_vec: Vec<f32>, limit: usize) -> Result<Vec<(Uuid, String, f32)>, Box<dyn std::error::Error>> {
        let results_stream = self.table
            .query()
            .nearest_to(query_vec)?
            .limit(limit)
            .execute()
            .await?;

        let batches: Vec<RecordBatch> = results_stream.try_collect().await?;
        let mut found_memories = Vec::new();

        for batch in batches {
            let ids = batch.column_by_name("id")
                .ok_or("No id column")?
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or("Type mismatch in DB column")?;

            let texts = batch.column_by_name("text")
                .ok_or("No text column")?
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or("Type mismatch in DB column")?;

            let distances = batch.column_by_name("_distance")
                .ok_or("No distance column")?
                .as_any()
                .downcast_ref::<Float32Array>()
                .unwrap();

            for i in 0..batch.num_rows() {
                let id_str = ids.value(i);
                let text_str = texts.value(i);
                let distance = distances.value(i);

                if let Ok(uuid) = Uuid::parse_str(id_str) {
                    found_memories.push((uuid, text_str.to_string(), distance));
                }
            }
        }

        Ok(found_memories)
    }

    pub async fn wipe(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.table.delete("true").await?;
        Ok(())
    }
}