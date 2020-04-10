extern crate rusqlite;
use rusqlite::{Connection, Result};

db_data_struct! {
    pub struct TextLabel {
        _Id: String,
        pub _Text: String
    }
}
