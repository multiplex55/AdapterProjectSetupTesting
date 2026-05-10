//! core skeleton crate.

pub mod algorithms;
pub mod domain;
pub mod services;
pub mod state;

pub fn crate_name() -> &'static str {
    "core"
}

#[cfg(test)]
mod tests {
    use crate::domain::{CoreError, InputFrame};
    use crate::services::CoreService;
    use messages::common::MessageType;
    use ports::{AlgorithmProvider, ProviderError};

    struct DeterministicProvider;

    impl AlgorithmProvider for DeterministicProvider {
        type Input = InputFrame;
        type Output = Vec<u8>;

        fn compute(&self, input: Self::Input) -> Result<Self::Output, ProviderError> {
            let mut out = input.payload;
            out.reverse();
            Ok(out)
        }
    }

    fn fixed_input(sequence: u64) -> InputFrame {
        InputFrame {
            protocol_version: 1,
            sequence,
            message_type: MessageType::Target5Status,
            payload: vec![1, 2, 3, 4],
        }
    }

    #[test]
    fn deterministic_outputs_from_fixed_inputs() {
        let mut service = CoreService::new(DeterministicProvider);
        let output = service.process(fixed_input(10)).expect("processing works");
        assert_eq!(output.payload, vec![4, 3, 2, 1]);
        assert_eq!(output.sequence, 10);
        assert_eq!(service.state().processed_count, 1);
    }

    #[test]
    fn state_transition_invariant_enforces_monotonic_sequence() {
        let mut service = CoreService::new(DeterministicProvider);

        let _ = service.process(fixed_input(20)).expect("first frame ok");
        let err = service
            .process(fixed_input(20))
            .expect_err("same sequence must fail");

        assert_eq!(
            err,
            CoreError::InvalidStateTransition("sequence must strictly increase")
        );
    }

    #[test]
    fn unsupported_message_version_is_mapped_from_messages_helper() {
        let mut service = CoreService::new(DeterministicProvider);
        let mut input = fixed_input(1);
        input.protocol_version = 99;

        let err = service.process(input).expect_err("unsupported version");
        assert_eq!(
            err,
            CoreError::UnsupportedMessageVersion {
                found: 99,
                min: 1,
                max: 1,
            }
        );
    }
}
