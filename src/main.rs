extern crate rusqlite;
use rusqlite::{Connection, Result};

mod data;
use data::PlayerActionHitAttribute;

fn main() -> Result<()> {
    let conn = Connection::open(data::DB_FILE)?;

    let res = PlayerActionHitAttribute::populate(&conn, "BUF_180_DMGLINK_LV01".to_string())?;
    println!("{:?}", res);
    let act = res.link_action_condition(&conn)?;
    println!("{:?}", act);
    Ok(())
}
