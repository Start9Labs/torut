use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::onion::OnionAddressV3;

impl Serialize for OnionAddressV3 {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let res = self.get_address_without_dot_onion();
        serializer.serialize_str(&res)
    }
}

impl<'de> Deserialize<'de> for OnionAddressV3 {
    //noinspection SpellCheckingInspection
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OnionAddressV3;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a raw onion address (without the .onion)")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::from_str(v).map_err(de::Error::custom)?)
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

// TODO(teawithsand): testing for these
