use serde_json::Value;


pub trait FomiTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> Value;
    fn execute(&self, args: Value) -> Result<String, String>;
}