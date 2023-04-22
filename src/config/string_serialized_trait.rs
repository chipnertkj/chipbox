pub trait StringSerializedTrait<T, D>:
    serde::Serialize + serde::de::DeserializeOwned
{
    type DeserializationError;
    fn serialize(value: T) -> Self;
    fn deserialize(&self, data: D) -> Result<T, Self::DeserializationError>;
}
