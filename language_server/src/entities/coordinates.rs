#[derive(Debug, Clone, Copy)]
pub struct Column(usize);

impl Column {
    pub fn column(&self) -> usize {
        self.0
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Column {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<usize> for Column {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Row(usize);

impl Row {
    pub fn row(&self) -> usize {
        self.0
    }
}

impl From<usize> for Row {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
