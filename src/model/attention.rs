#[derive(Clone)]
pub struct AttentionHead {
    window: usize,
}

impl AttentionHead {
    pub fn new(window: usize) -> Self {
        Self { window }
    }

    pub fn attend(&self, prompt: &str) -> String {
        format!("attn(window={}) snapshot='{}'", self.window, prompt)
    }
}
