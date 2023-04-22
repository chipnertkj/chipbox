use crate::config;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct HostIdSerialized {
    host_name: String,
}

impl config::StringSerializedTrait<cpal::HostId, ()> for HostIdSerialized {
    type DeserializationError = HostIdDeserializationError;

    fn serialize(value: cpal::HostId) -> Self {
        Self {
            host_name: value.name().to_string(),
        }
    }

    fn deserialize(
        &self,
        _: (),
    ) -> Result<cpal::HostId, Self::DeserializationError> {
        let available_hosts = cpal::available_hosts();
        for host in &available_hosts {
            if host.name() == self.host_name {
                return Ok(*host);
            }
        }
        Err(HostIdDeserializationError {
            host_name: self.host_name.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct HostIdDeserializationError {
    host_name: String,
}

impl std::fmt::Display for HostIdDeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "unable to deserialize HostId from '{}'. such host does not exist or is unavailable",
                self.host_name
            )
            .as_str(),
        )
    }
}

impl std::error::Error for HostIdDeserializationError {}
