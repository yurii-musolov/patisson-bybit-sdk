use serde::Deserialize;

pub fn invalid_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let res = Deserialize::deserialize(deserializer).unwrap_or(None);
    Ok(res)
}
