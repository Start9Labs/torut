use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::onion::v3::{TorPublicKeyV3, TorSecretKeyV3};

impl Serialize for TorSecretKeyV3 {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(&self.as_bytes()[..]))
    }
}

impl Serialize for TorPublicKeyV3 {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(&self.0[..]))
    }
}

impl<'de> Deserialize<'de> for TorSecretKeyV3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TorSecretKeyV3;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a raw onion address (without the .onion)")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let raw = base64::decode(&v[..]).map_err(serde::de::Error::custom)?;
                if raw.len() != 64 {
                    return Err(serde::de::Error::custom("Invalid secret key length"));
                }
                let mut buf = [0u8; 64];
                buf.clone_from_slice(&raw[..]);
                Ok(Self::Value::from(buf))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl<'de> Deserialize<'de> for TorPublicKeyV3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TorPublicKeyV3;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a raw onion address (without the .onion)")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let raw = base64::decode(&v[..]).map_err(serde::de::Error::custom)?;
                if raw.len() != 32 {
                    return Err(serde::de::Error::custom("Invalid secret key length"));
                }
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = raw[i];
                }
                Ok(TorPublicKeyV3(buf))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_serialize_and_deserialize_secret_key() {
        let sk = TorSecretKeyV3::generate();
        let data = serde_json::to_vec(&sk).unwrap();
        let rsk: TorSecretKeyV3 = serde_json::from_slice(&data).unwrap();

        assert_eq!(sk, rsk);
    }

    #[test]
    fn test_can_serialize_and_deserialize_public_key() {
        let pk = TorSecretKeyV3::generate().public();
        let data = serde_json::to_vec(&pk).unwrap();
        let rpk: TorPublicKeyV3 = serde_json::from_slice(&data).unwrap();

        assert_eq!(pk, rpk);
    }

    #[test]
    fn test_can_serialize_and_deserialize_secret_key_with_no_borrowing() {
        let sk = TorSecretKeyV3::generate();
        let data = serde_json::to_vec(&sk).unwrap();

        let mut c = Cursor::new(&data);
        let rsk: TorSecretKeyV3 = serde_json::from_reader(&mut c).unwrap();

        assert_eq!(sk, rsk);
    }

    #[test]
    fn test_can_serialize_and_deserialize_public_key_with_no_borrowing() {
        let pk = TorSecretKeyV3::generate().public();
        let data = serde_json::to_vec(&pk).unwrap();

        let mut c = Cursor::new(&data);
        let rpk: TorPublicKeyV3 = serde_json::from_reader(&mut c).unwrap();

        assert_eq!(pk, rpk);
    }
}
