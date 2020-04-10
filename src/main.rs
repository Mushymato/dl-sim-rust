extern crate rusqlite;
use rusqlite::{Connection, Result};

mod data;
use data::PlayerActionHitAttribute;

fn main() -> Result<()> {
    let conn = Connection::open(data::DB_FILE)?;

    let res = PlayerActionHitAttribute::populate(&conn, &"KAT_112_04_H01_LV03".to_string())?;
    println!("{:?}", res);
    res.link_action_condition(&conn, |r| {
        println!("{:?}", r);
    });
    Ok(())
}
