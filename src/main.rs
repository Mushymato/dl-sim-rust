extern crate rusqlite;
use rusqlite::{Connection, Result};

mod entities;
mod simulation;
use simulation::Player;

fn main() -> Result<()> {
    let conn = Connection::open(entities::DB_FILE)?;
    let chara_id = 10540502;
    // let dragon_id = 20050509;
    let dragon_id = 20050507;
    let weapon_id = 30560501;
    let amulet_ids = [40050026, 40050098];
    let player = Player::new(&conn, chara_id, dragon_id, weapon_id, amulet_ids);
    println!("{:?}", player);
    Ok(())
}
