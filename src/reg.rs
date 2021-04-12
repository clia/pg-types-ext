use postgres_types::{Type};
// use postgres_types::accepts;
use tokio_postgres::types::accepts;
use postgres_protocol::types::{self};
use std::error::Error;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegType(pub String);

// impl<'a> postgres_types::FromSql<'a> for RegType
// {
//     fn from_sql(_ty: &Type, raw: &[u8]) -> Result<RegType, Box<dyn Error + Sync + Send>> {
//         // for c in raw {
//         //     print!("{}", String::from(*c as char));
//         // }
//         types::text_from_sql(raw).map(|s| RegType(s.to_string()))
//     }

//     accepts!(REGTYPE);
// }

impl<'a> tokio_postgres::types::FromSql<'_> for RegType
{
    fn from_sql(_ty: &Type, raw: &[u8]) -> Result<RegType, Box<dyn Error + Sync + Send>> {
        for c in raw {
            print!("{}", String::from(*c as char));
        }
        types::text_from_sql(raw).map(|s| RegType(s.to_string()))
    }

    accepts!(REGTYPE);
}
