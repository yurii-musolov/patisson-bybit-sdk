pub fn invalid_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let res = serde::Deserialize::deserialize(deserializer).unwrap_or(None);
    Ok(res)
}

pub fn deserialize_str<'de, T>(
    json: &'de str,
) -> Result<T, serde_path_to_error::Error<serde_json::Error>>
where
    T: serde::de::Deserialize<'de>,
{
    let deserializer = &mut serde_json::Deserializer::from_str(json);

    serde_path_to_error::deserialize(deserializer)
}
