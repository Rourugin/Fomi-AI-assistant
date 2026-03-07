use serde_json::Value;
use uuid::Uuid;


pub trait FomiTool: Send + Sync {
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> Value;
    fn execute(&self, args: Value) -> Result<String, String>;
}