use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Visitor, value::CowStrDeserializer},
};
use std::{borrow::Cow, collections::HashMap, fmt, hash::Hash};

pub fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct OptVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> Visitor<'de> for OptVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Option<T>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("null, empty string, or a valid value")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let cow = Cow::<str>::deserialize(d)?;

            if cow.is_empty() {
                return Ok(None);
            }

            T::deserialize(CowStrDeserializer::<D::Error>::new(cow)).map(Some)
        }
    }

    deserializer.deserialize_option(OptVisitor(std::marker::PhantomData))
}

#[cfg(test)]
mod tests {
    use crate::serde::deserialize_json;

    use super::*;

    #[test]
    fn deserialize_with_empty_string_as_none() {
        #[derive(PartialEq, Deserialize, Debug)]
        enum MassageType {
            Type,
        }
        #[derive(PartialEq, Deserialize, Debug)]
        struct Massage {
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub enum1: Option<MassageType>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub enum2: Option<MassageType>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub enum3: Option<MassageType>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub enum4: Option<MassageType>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub str1: Option<String>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub str2: Option<String>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub str3: Option<String>,
            #[serde(default, deserialize_with = "empty_string_as_none")]
            pub str4: Option<String>,
        }
        let json = r#"{
            "enum2": null,
            "enum3": "",
            "enum4": "Type",
            "str2": null,
            "str3": "",
            "str4": "string"
        }"#;
        let expected = Massage {
            enum1: None,                        // missing field
            enum2: None,                        // null value
            enum3: None,                        // empty string
            enum4: Some(MassageType::Type),     // correct enum
            str1: None,                         // missing field
            str2: None,                         // null value
            str3: None,                         // empty string
            str4: Some(String::from("string")), // correct string
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }
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
