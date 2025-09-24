#[derive(Debug)]
pub struct SUIDBinary {
    pub path: String,
}

impl SUIDBinary {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}
