use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::path::PathBuf;


pub struct FomiEmbedder {
    model: TextEmbedding,
}

impl FomiEmbedder {
    pub fn new(model_path: PathBuf) -> Result<FomiEmbedder, Box<dyn std::error::Error>> {
        let options = InitOptions::new(EmbeddingModel::AllMiniLML6V2)
            .with_cache_dir(model_path)
            .with_show_download_progress(false);

        let model = TextEmbedding::try_new(options)?;
        Ok(FomiEmbedder {
            model
        })
    }

    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let embeddings = self.model.embed(vec![text], None)?;

        Ok(embeddings[0].clone())
    }
}