#[derive(Clone)]
pub struct SsmHead;

impl SsmHead {
    pub fn new() -> Self {
        Self
    }

    pub fn step(&self, prompt: &str) -> String {
        format!("ssm(step) '{}' -> linear_state", prompt)
    }
}
