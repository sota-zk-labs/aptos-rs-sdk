use serde_json::value::RawValue;

pub(crate) type Bigint = u64;
pub(crate) type Jsonb = Box<RawValue>;