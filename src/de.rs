use serde::Deserialize;

/// Deserialize a `Option<String>` and map an empty string to `None`
pub(crate) fn empty_string_as_none_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    let s = s.and_then(|s| if s.is_empty() { None } else { Some(s) });
    Ok(s)
}

/// Deserialize a `String` and map an empty string to `None`
pub(crate) fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = if s.is_empty() { None } else { Some(s) };
    Ok(s)
}
