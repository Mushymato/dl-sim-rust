use std::collections::HashMap;

extern crate rusqlite;
use rusqlite::{Connection, Result, NO_PARAMS};

db_data_struct! {
    pub struct TextLabel {
        _Id: String,
        _Text: String
    }
}
