use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};
use sqlx::{Decode, Encode, Postgres, postgres::PgTypeInfo};
pub use ulid::Ulid as PrimitiveUlid;

/// Ulid wrapper that works with serde and sqlx postgres
#[derive(PartialEq, Eq, Debug)]
pub struct Ulid(pub PrimitiveUlid);

struct UlidVisitor;
impl<'de> Visitor<'de> for UlidVisitor {
    type Value = Ulid;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Cockford's base32 26 character string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Ulid(
            PrimitiveUlid::from_string(v).map_err(|_| de::Error::custom("invalid ULID string"))?,
        ))
    }
}

impl Serialize for Ulid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Ulid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UlidVisitor)
    }
}

impl<'r> Decode<'r, Postgres> for Ulid {
    fn decode(
        value: <Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let buf = <[u8; 16] as Decode<Postgres>>::decode(value)?;
        Ok(Ulid(PrimitiveUlid::from_bytes(buf)))
    }
}

impl<'r> Encode<'r, Postgres> for Ulid {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let u8buf = self.0.to_bytes();
        buf.reserve_exact(u8buf.len());
        for b in u8buf {
            buf.push(b);
        }
        Ok(sqlx::encode::IsNull::No)
    }
}

impl sqlx::Type<Postgres> for Ulid {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        PgTypeInfo::with_name("bytea")
    }
}

impl From<PrimitiveUlid> for Ulid {
    fn from(value: PrimitiveUlid) -> Self {
        Self(value)
    }
}

impl From<Ulid> for PrimitiveUlid {
    fn from(value: Ulid) -> Self {
        value.0
    }
}
