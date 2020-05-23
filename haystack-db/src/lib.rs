use sled;
use chrono::{self,offset::FixedOffset};

pub struct HaystackDB {
    db: sled::Db
}

pub enum HaystackType {
    Str(String),
    Date(chrono::NaiveDate),
    DateTime(chrono::DateTime<FixedOffset>)
}

pub struct Rec<'a> {
    id: u64,
    modified: chrono::NaiveDateTime,
    tags: &'a[(&'a str,HaystackType)]

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
