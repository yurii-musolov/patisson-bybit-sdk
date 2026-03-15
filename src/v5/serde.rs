use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Deserializer, Serialize};

pub fn invalid_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let res = Deserialize::deserialize(deserializer).unwrap_or(None);
    Ok(res)
}

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

pub fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let n = i64::deserialize(deserializer)?;
    match n {
        1 => Ok(true),
        0 => Ok(false),
        other => Err(serde::de::Error::custom(format!(
            "invalid boolean integer: {other}"
        ))),
    }
}

pub fn string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(serde::de::Error::custom(format!(
            "invalid boolean string: {other}"
        ))),
    }
}

pub fn string_to_option_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        Some("1") => Ok(Some(true)),
        Some("0") => Ok(Some(false)),
        Some("") | None => Ok(None),
        Some(other) => Err(serde::de::Error::custom(format!(
            "invalid boolean string: {other}"
        ))),
    }
}

#[inline]
pub fn deserialize_json<'de, T>(
    json: &'de str,
) -> Result<T, serde_path_to_error::Error<serde_json::Error>>
where
    T: Deserialize<'de>,
{
    let deserializer = &mut serde_json::Deserializer::from_str(json);

    serde_path_to_error::deserialize(deserializer)
}

#[inline]
pub fn serialize_json<T>(msg: &T) -> serde_json::Result<String>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(msg)
}

#[inline]
pub fn serialize_query<T>(msg: &T) -> Result<String, serde_urlencoded::ser::Error>
where
    T: ?Sized + Serialize,
{
    serde_urlencoded::to_string(msg)
}

pub trait Unique<Q>
where
    Q: Hash + Eq,
{
    fn unique_key(&self) -> Q;
}

pub fn hash_map<'de, D, Q, T>(deserializer: D) -> Result<HashMap<Q, T>, D::Error>
where
    D: Deserializer<'de>,
    Q: Hash + Eq,
    T: Deserialize<'de> + Unique<Q>,
{
    let mut map = HashMap::new();
    for item in Vec::<T>::deserialize(deserializer)? {
        map.insert(item.unique_key(), item);
    }
    Ok(map)
}
