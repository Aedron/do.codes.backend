
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};



pub fn get_timestamp() -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp()
}
