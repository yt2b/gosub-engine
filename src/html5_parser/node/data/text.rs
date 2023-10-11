#[derive(Debug, PartialEq, Clone)]
pub struct TextData {
    pub(crate) value: String,
}

impl Default for TextData {
    fn default() -> Self {
        Self::new()
    }
}

impl TextData {
    pub(crate) fn new() -> Self {
        Self {
            value: "".to_string(),
        }
    }

    pub(crate) fn with_value(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}