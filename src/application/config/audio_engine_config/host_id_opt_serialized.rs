use crate::application::config::{self, StringSerializedTrait as _};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct HostIdOptSerialized {
    host_name_opt: Option<String>,
}

impl From<Option<cpal::HostId>> for HostIdOptSerialized {
    fn from(value: Option<cpal::HostId>) -> Self {
        HostIdOptSerialized::serialize(value)
    }
}

impl TryFrom<HostIdOptSerialized> for Option<cpal::HostId> {
    type Error = HostIdDeserializationError;
    fn try_from(value: HostIdOptSerialized) -> Result<Self, Self::Error> {
        value.deserialize(())
    }
}

impl config::StringSerializedTrait<Option<cpal::HostId>, ()>
    for HostIdOptSerialized
{
    type DeserializationError = HostIdDeserializationError;

    fn serialize(value: Option<cpal::HostId>) -> Self {
        Self {
            host_name_opt: value.map(|x| x.name().to_string()),
        }
    }

    fn deserialize(
        self,
        _: (),
    ) -> Result<Option<cpal::HostId>, Self::DeserializationError> {
        match &self.host_name_opt {
            Some(host_name) => {
                let available_hosts = cpal::available_hosts();
                for host in &available_hosts {
                    if host.name() == host_name {
                        return Ok(Some(*host));
                    }
                }
                Err(HostIdDeserializationError {
                    host_id_serialized: self,
                })
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug)]
pub struct HostIdDeserializationError {
    host_id_serialized: HostIdOptSerialized,
}

impl std::fmt::Display for HostIdDeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "unable to deserialize HostId from '{:?}'. such host does not exist or is unavailable",
                self.host_id_serialized
                    .host_name_opt
            )
            .as_str(),
        )
    }
}

impl std::error::Error for HostIdDeserializationError {}
