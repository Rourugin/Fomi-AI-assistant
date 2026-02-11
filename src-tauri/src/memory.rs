use std::path::PathBuf;
use uuid::Uuid;
pub mod vector_db;
pub mod embedder;


pub struct MemorySystem {
    embedder: embedder::FomiEmbedder,
    store: vector_db::FomiVectorStore,
}

impl MemorySystem {
    pub async  fn new(model_path: PathBuf, db_path: PathBuf) -> Result<MemorySystem, Box<dyn std::error::Error>> {
        let embedder = embedder::FomiEmbedder::new(model_path)?;
        let db = vector_db::FomiVectorStore::new(db_path).await?;

        Ok(MemorySystem { 
            embedder, 
            store: db, 
        })
    }

    pub async fn ingest(&self, text: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let vector = self.embedder.embed(text)?;
        self.store.add(id, text, vector, source).await?;

        Ok(())
    }

    pub async fn retrieve(&self, text: &str, limit: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let vector = self.embedder.embed(text)?;
        let results = self.store.search(vector, limit).await?;
        let texts: Vec<String> = results.into_iter().map(|(_id, text)| text).collect();

        Ok(texts)
    }
}