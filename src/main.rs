extern crate rusqlite;
use rusqlite::{Connection, Result};

mod data;
use data::CharaUniqueCombo;

fn main() -> Result<()> {
    let conn = Connection::open(data::DB_FILE)?;

    let res = CharaUniqueCombo::populate(&conn, &1)?;
    println!("{:?}", res);
    let combo = res.link_combo_actions(&conn)?;
    for c in combo {
        println!("\n{:?}", c);
    }

    // let res = PlayerAction::populate(&conn, &799000)?;
    // println!("{:?}", res);
    // let parts = res.link_action_parts(&conn)?;
    // for p in parts {
    //     if p.commandType == CommandType::HIT {
    //         println!("SEQ {}\n{:?}", p._seq, p);
    //         let attrs = p.link_hit_label(&conn)?;
    //         for a in attrs {
    //             println!("\n{:?}", a);
    //         }
    //     } else if p.commandType == CommandType::BULLET {
    //         println!("SEQ {}\n{:?}", p._seq, p);
    //         let attrs = p.link_hit_attr_label(&conn)?;
    //         for a in attrs {
    //             println!("\n{:?}", a);
    //         }
    //     }
    // }

    Ok(())
}
