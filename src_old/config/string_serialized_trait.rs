pub trait SerializedItemTrait<V, D>:
    serde::Serialize + serde::de::DeserializeOwned
{
    type SerializationError;
    type DeserializationError;
    fn serialize(value: V) -> Result<Self, Self::SerializationError>;
    fn deserialize(&self, data: D) -> Result<V, Self::DeserializationError>;
}
