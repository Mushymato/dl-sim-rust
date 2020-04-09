extern crate rusqlite;
use rusqlite::{Connection, Result};

mod data;
use data::PlayerActionHitAttribute;

fn main() -> Result<()> {
    let conn = Connection::open(data::DB_FILE)?;

    let res = PlayerActionHitAttribute::populate(conn, "CAN_CHR_05_DRAIN_LV02".to_string());
    match res {
        Ok(v) => println!("PlayerActionHitAttribute: {:?}", v),
        Err(e) => println!("error: {:?}", e),
    }
    Ok(())
}
