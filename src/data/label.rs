extern crate rusqlite;
use crate::data::mappings::*;
use rusqlite::{Connection, Result};

db_data_struct! {
    pub struct TextLabel {
        _Id: String,
        _Text: String
    }
}
