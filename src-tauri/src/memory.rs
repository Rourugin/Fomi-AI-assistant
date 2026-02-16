use std::{path::PathBuf, sync::Mutex};
use uuid::Uuid;
pub mod vector_db;
pub mod embedder;


pub struct MemorySystem {
    embedder: Mutex<embedder::FomiEmbedder>,
    store: vector_db::FomiVectorStore,
}

impl MemorySystem {
    pub async  fn new(model_path: PathBuf, db_path: PathBuf) -> Result<MemorySystem, Box<dyn std::error::Error>> {
        let embedder = embedder::FomiEmbedder::new(model_path)?;
        let db = vector_db::FomiVectorStore::new(db_path).await?;

        Ok(MemorySystem { 
            embedder: Mutex::new(embedder), 
            store: db, 
        })
    }

    fn split_text(&self, text: &str) -> Vec<String> {
        text.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    pub async fn ingest(&self, text: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let vector = {
            let mut embedder_guard = self.embedder.lock().map_err(|_| "Mutex poison error")?;
            embedder_guard.embed(text)?
        };
        let chunks = self.split_text(text);

        for chunk in chunks {
            self.store.add(id, &chunk, vector.clone(), source).await?;
        }

        Ok(())
    }

    pub async fn retrieve(&self, text: &str, limit: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let vector = {
            let mut embedder_guard = self.embedder.lock().map_err(|_| "Mutex poison error")?;
            embedder_guard.embed(text)?
        };
        let results = self.store.search(vector, limit).await?;
        let texts: Vec<String> = results.into_iter().map(|(_id, text)| text).collect();

        Ok(texts)
    }

    pub async fn wipe_memory(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.store.wipe().await
    }
}