//! core skeleton crate.

pub fn crate_name() -> &'static str {
    "core"
}

#[cfg(test)]
mod tests {
    use ports::{
        AlgorithmProvider, Clock, ClockError, DataSource, DataSourceError, MessagePublisher,
        MessageSubscriber, ProviderError, TelemetryError, TelemetrySink, TransportError,
    };

    struct Mock;

    impl DataSource for Mock {
        type Item = u8;

        fn read(&self) -> Result<Self::Item, DataSourceError> {
            Ok(1)
        }
    }

    impl MessagePublisher for Mock {
        type Message = u8;

        fn publish(&self, _message: Self::Message) -> Result<(), TransportError> {
            Ok(())
        }
    }

    impl MessageSubscriber for Mock {
        type Message = u8;

        fn receive(&self) -> Result<Self::Message, TransportError> {
            Ok(2)
        }
    }

    impl Clock for Mock {
        type Instant = u64;

        fn now(&self) -> Result<Self::Instant, ClockError> {
            Ok(42)
        }
    }

    impl TelemetrySink for Mock {
        type Event = &'static str;

        fn emit(&self, _event: Self::Event) -> Result<(), TelemetryError> {
            Ok(())
        }
    }

    impl AlgorithmProvider for Mock {
        type Input = u8;
        type Output = u16;

        fn compute(&self, input: Self::Input) -> Result<Self::Output, ProviderError> {
            Ok(input as u16)
        }
    }

    #[test]
    fn trait_objects_are_usable_from_core() {
        let mock = Mock;

        let source: &dyn DataSource<Item = u8> = &mock;
        let publisher: &dyn MessagePublisher<Message = u8> = &mock;
        let subscriber: &dyn MessageSubscriber<Message = u8> = &mock;
        let clock: &dyn Clock<Instant = u64> = &mock;
        let telemetry: &dyn TelemetrySink<Event = &'static str> = &mock;
        let provider: &dyn AlgorithmProvider<Input = u8, Output = u16> = &mock;

        assert_eq!(source.read().ok(), Some(1));
        assert!(publisher.publish(9).is_ok());
        assert_eq!(subscriber.receive().ok(), Some(2));
        assert_eq!(clock.now().ok(), Some(42));
        assert!(telemetry.emit("tick").is_ok());
        assert_eq!(provider.compute(7).ok(), Some(7));
    }
}
