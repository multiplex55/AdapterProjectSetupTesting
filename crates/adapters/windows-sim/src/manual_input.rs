use std::collections::VecDeque;

use crate::replay::ReplayEvent;

#[derive(Debug, Clone)]
pub struct ManualInput {
    queue: VecDeque<ReplayEvent>,
}

impl ManualInput {
    pub fn new(initial_events: Vec<ReplayEvent>) -> Self {
        Self {
            queue: initial_events.into(),
        }
    }

    pub fn feed(&mut self, event: ReplayEvent) {
        self.queue.push_back(event);
    }

    pub fn next_event(&mut self) -> Option<ReplayEvent> {
        self.queue.pop_front()
    }
}
