use crate::common::{
    MAX_SUPPORTED_PROTOCOL_VERSION, MIN_SUPPORTED_PROTOCOL_VERSION, MessageType,
};
use crate::ethernet::EthernetEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersioningError {
    UnsupportedProtocolVersion { found: u16, min: u16, max: u16 },
    UnknownMessageType(u16),
}

pub fn is_protocol_version_supported(version: u16) -> bool {
    (MIN_SUPPORTED_PROTOCOL_VERSION..=MAX_SUPPORTED_PROTOCOL_VERSION).contains(&version)
}

pub fn validate_protocol_version(version: u16) -> Result<(), VersioningError> {
    if is_protocol_version_supported(version) {
        Ok(())
    } else {
        Err(VersioningError::UnsupportedProtocolVersion {
            found: version,
            min: MIN_SUPPORTED_PROTOCOL_VERSION,
            max: MAX_SUPPORTED_PROTOCOL_VERSION,
        })
    }
}

pub fn validate_message_type(raw_type: u16) -> Result<MessageType, VersioningError> {
    MessageType::from_code(raw_type).ok_or(VersioningError::UnknownMessageType(raw_type))
}

pub fn validate_envelope_compatibility(envelope: &EthernetEnvelope) -> Result<(), VersioningError> {
    validate_protocol_version(envelope.protocol_version)?;
    let _ = envelope.message_type();
    Ok(())
}

pub fn serialize_envelope_deterministic(
    envelope: &EthernetEnvelope,
) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(envelope)
}

pub fn deserialize_envelope_deterministic(
    bytes: &[u8],
) -> Result<EthernetEnvelope, serde_json::Error> {
    serde_json::from_slice(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::CURRENT_PROTOCOL_VERSION;
    use crate::ethernet::EthernetPayload;
    use pretty_assertions::assert_eq;

    fn sample_status_envelope() -> EthernetEnvelope {
        EthernetEnvelope {
            protocol_version: CURRENT_PROTOCOL_VERSION,
            payload: EthernetPayload::Target5Status(crate::Target5Status {
                device_id: 42,
                online: true,
                sequence: 11,
            }),
        }
    }

    #[test]
    fn round_trip_serde_is_stable() {
        let envelope = sample_status_envelope();

        let first = serialize_envelope_deterministic(&envelope).expect("serialize first");
        let second = serialize_envelope_deterministic(&envelope).expect("serialize second");
        assert_eq!(first, second);

        let decoded = deserialize_envelope_deterministic(&first).expect("deserialize");
        assert_eq!(decoded, envelope);
    }

    #[test]
    fn reject_unsupported_versions() {
        let err_low = validate_protocol_version(0).expect_err("version 0 must fail");
        let err_high = validate_protocol_version(2).expect_err("version 2 must fail");

        assert_eq!(
            err_low,
            VersioningError::UnsupportedProtocolVersion {
                found: 0,
                min: MIN_SUPPORTED_PROTOCOL_VERSION,
                max: MAX_SUPPORTED_PROTOCOL_VERSION,
            }
        );
        assert_eq!(
            err_high,
            VersioningError::UnsupportedProtocolVersion {
                found: 2,
                min: MIN_SUPPORTED_PROTOCOL_VERSION,
                max: MAX_SUPPORTED_PROTOCOL_VERSION,
            }
        );
    }

    #[test]
    fn unknown_message_type_is_rejected() {
        let err = validate_message_type(0xDEAD).expect_err("unknown type must fail");
        assert_eq!(err, VersioningError::UnknownMessageType(0xDEAD));
    }

    #[test]
    fn known_message_type_is_accepted() {
        let ty = validate_message_type(0x000A).expect("known type accepted");
        assert_eq!(ty, MessageType::Target10Command);
    }
}
