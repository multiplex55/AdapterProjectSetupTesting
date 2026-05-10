use std::cell::RefCell;

use ports::{DataSource, DataSourceError};

use crate::{manual_input::ManualInput, replay::DeterministicReplay, replay::ReplayEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimAdapterError {
    Timeout,
    TransportDisconnected,
    Exhausted,
}

#[derive(Debug)]
pub enum InputMode {
    Replay(DeterministicReplay),
    Manual(ManualInput),
    LocalNetworkBind,
}

pub struct SimDataSource {
    mode: RefCell<InputMode>,
}

impl SimDataSource {
    pub fn replay(replay: DeterministicReplay) -> Self {
        Self {
            mode: RefCell::new(InputMode::Replay(replay)),
        }
    }

    pub fn manual(manual: ManualInput) -> Self {
        Self {
            mode: RefCell::new(InputMode::Manual(manual)),
        }
    }

    pub fn local_network_bind() -> Self {
        Self {
            mode: RefCell::new(InputMode::LocalNetworkBind),
        }
    }

    pub fn read_event(&self) -> Result<ReplayEvent, SimAdapterError> {
        match &mut *self.mode.borrow_mut() {
            InputMode::Replay(replay) => replay.next().ok_or(SimAdapterError::Exhausted),
            InputMode::Manual(manual) => manual.next_event().ok_or(SimAdapterError::Timeout),
            InputMode::LocalNetworkBind => Err(SimAdapterError::TransportDisconnected),
        }
    }
}

impl DataSource for SimDataSource {
    type Item = ReplayEvent;

    fn read(&self) -> Result<Self::Item, DataSourceError> {
        self.read_event().map_err(|err| match err {
            SimAdapterError::Timeout => DataSourceError::Unavailable,
            SimAdapterError::TransportDisconnected => DataSourceError::Unavailable,
            SimAdapterError::Exhausted => DataSourceError::NotFound,
        })
    }
}
