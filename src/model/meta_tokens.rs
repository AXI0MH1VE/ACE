pub struct MetaTokenInjector {
    count: usize,
}

impl MetaTokenInjector {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn inject(&self, prompt: &str, mode: &str) -> String {
        format!(
            "[meta:{} tokens injected for {}] {}",
            self.count, mode, prompt
        )
    }
}
