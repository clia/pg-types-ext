use postgres_types::{FromSql, Type};
use postgres_types::accepts;
use postgres_protocol::types::{self};
use std::error::Error;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegType(pub String);

impl<'a> FromSql<'a> for RegType
{
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<RegType, Box<dyn Error + Sync + Send>> {
        // for c in raw {
        //     print!("{}", String::from(*c as char));
        // }
        types::text_from_sql(raw).map(|s| RegType(s.to_string()))
    }

    accepts!(REGTYPE);
}
