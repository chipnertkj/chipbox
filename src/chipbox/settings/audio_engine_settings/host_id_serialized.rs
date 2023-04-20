#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct HostIdSerialized {
    host_name_opt: Option<String>,
}

impl HostIdSerialized {
    pub fn new(host_id_opt: Option<cpal::HostId>) -> Self {
        Self::serialize(host_id_opt)
    }

    pub fn serialize(host_id_opt: Option<cpal::HostId>) -> Self {
        host_id_opt.into()
    }

    pub fn deserialize(
        self,
    ) -> Result<Option<cpal::HostId>, HostIdDeserializationError> {
        self.try_into()
    }
}

impl From<cpal::HostId> for HostIdSerialized {
    fn from(value: cpal::HostId) -> Self {
        Self {
            host_name_opt: Some(value.name().to_string()),
        }
    }
}

impl From<Option<cpal::HostId>> for HostIdSerialized {
    fn from(value: Option<cpal::HostId>) -> Self {
        Self {
            host_name_opt: value.map(|x| x.name().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct HostIdDeserializationError {
    host_id_serialized: HostIdSerialized,
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

impl TryFrom<HostIdSerialized> for Option<cpal::HostId> {
    type Error = HostIdDeserializationError;
    fn try_from(value: HostIdSerialized) -> Result<Self, Self::Error> {
        match &value.host_name_opt {
            Some(host_name) => {
                let available_hosts = cpal::available_hosts();
                for host in &available_hosts {
                    if host.name() == host_name {
                        return Ok(Some(*host));
                    }
                }
                Err(HostIdDeserializationError {
                    host_id_serialized: value,
                })
            }
            None => Ok(None),
        }
    }
}
