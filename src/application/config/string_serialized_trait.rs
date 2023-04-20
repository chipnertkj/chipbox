pub trait StringSerializedTrait<T>:
    serde::Serialize + serde::de::DeserializeOwned + From<T>
where
    T: TryFrom<Self>,
{
    type DeserializationError;
    fn serialize(value: T) -> Self;
    fn deserialize(self) -> Result<T, Self::DeserializationError>;
}
