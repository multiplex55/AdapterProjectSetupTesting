#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinkState {
    pub linked_pairs: u64,
}

impl LinkState {
    pub fn record_linked_pair(&mut self) {
        self.linked_pairs += 1;
    }
}
