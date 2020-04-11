extern crate rusqlite;
use rusqlite::{Connection, Result};

mod data;
use data::CommandType;
use data::PlayerAction;

fn main() -> Result<()> {
    let conn = Connection::open(data::DB_FILE)?;

    // let res = PlayerActionHitAttribute::populate(&conn, &"KAT_112_04_H01_LV03".to_string())?;
    // println!("{:?}", res);
    // res.link_action_condition(&conn, |r| {
    //     println!("{:?}", r);
    // });

    let res = PlayerAction::populate(&conn, &799000)?;
    println!("{:?}", res);
    let parts = res.link_action_parts(&conn)?;
    for p in parts {
        if p.commandType == CommandType::HIT {
            println!("SEQ {}\n{:?}", p._seq, p);
            let attrs = p.link_hit_label(&conn)?;
            for a in attrs {
                println!("\n{:?}", a);
            }
        } else if p.commandType == CommandType::BULLET {
            println!("SEQ {}\n{:?}", p._seq, p);
            let attrs = p.link_hit_attr_label(&conn)?;
            for a in attrs {
                println!("\n{:?}", a);
            }
        }
    }

    Ok(())
}
