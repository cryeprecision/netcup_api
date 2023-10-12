/// Serialize a value with its `ToString` representation
pub(crate) fn with_to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
