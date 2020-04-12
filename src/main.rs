extern crate rusqlite;
use rusqlite::{Connection, Result};

mod data;
use data::*;

fn main() -> Result<()> {
    let conn = Connection::open(data::DB_FILE)?;

    let all_chara = CharaData::populate_all(&conn)?;
    for (_, chara) in &all_chara {
        println!(
            "{} HP {} ATK {}",
            chara.link_name(&conn),
            chara.max_hp(),
            chara.max_atk()
        );
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
