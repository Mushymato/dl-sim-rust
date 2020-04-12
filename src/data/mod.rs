pub const DB_FILE: &str = "dl.sqlite";

/* define */

macro_rules! db_data_struct {
    ($namevis:vis struct $name:ident { $pkvis:vis $pkname:ident : $pktype:ty, $($fvis:vis $fname:ident : $ftype:ty),* }) => {
        #[derive(Debug, Default)]
        #[allow(non_snake_case)]
        $namevis struct $name {
            $pkvis $pkname : $pktype,
            $($fvis $fname : $ftype),*
        }

        #[allow(dead_code)]
        impl $name {
            // pub fn field_names() -> &'static [&'static str] {
            //     static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
            //     NAMES
            // }

            pub fn populate(conn: &Connection, pk: &$pktype) -> Result<$name>{
                let mut x = 0;
                let mut counter = || {x+=1; return x};
                return conn.query_row(
                    stringify!(SELECT $pkname, $($fname),* FROM $name WHERE $pkname=?;),
                    &[&pk],
                    |r| {
                    Ok($name {
                        $pkname : r.get(0)?,
                        $($fname: match r.get(counter()) {
                            Ok(d) => d,
                            Err(_e) => <$ftype>::default(),
                        }),*
                    })
                });
            }

            pub fn populate_multiple(conn: &Connection, val: &$pktype) -> Result<Vec<$name>>{
                let mut stmt = conn.prepare(stringify!(SELECT $pkname, $($fname),* FROM $name WHERE $pkname=?;))?;
                let rows = stmt.query_map(&[&val], |r| {
                    let mut x = 0;
                    let mut counter = || {x+=1; return x};
                    Ok($name {
                        $pkname : r.get(0)?,
                        $($fname: match r.get(counter()) {
                            Ok(d) => d,
                            Err(_e) => <$ftype>::default(),
                        }),*
                    })
                })?;
                let mut res = Vec::new();
                for q_res in rows {
                    res.push(q_res?);
                }
                return Ok(res);
            }
            pub fn populate_conditionally(conn: &Connection, conditions: &str, values: &[&$pktype]) -> Result<Vec<$name>>{
                let mut stmt = conn.prepare(&format!(stringify!(SELECT $pkname, $($fname),* FROM $name WHERE {}), conditions))?;
                let rows = stmt.query_map(values, |r| {
                    let mut x = 0;
                    let mut counter = || {x+=1; return x};
                    Ok($name {
                        $pkname : r.get(0)?,
                        $($fname: match r.get(counter()) {
                            Ok(d) => d,
                            Err(_e) => <$ftype>::default(),
                        }),*
                    })
                })?;
                let mut res = Vec::new();
                for q_res in rows {
                    res.push(q_res?);
                }
                return Ok(res);
            }
        }

        impl PartialEq for $name{
            fn eq(&self, other: &Self) -> bool {
                return self.$pkname == other.$pkname;
            }
        }
    }
}

macro_rules! link_data_struct {
    ($name:ty {$($func:ident : $dkey:ident -> $dname:ident),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<$dname> {
                return $dname::populate(&conn, &self.$dkey);
            })*
        }
    };
}

macro_rules! link_data_struct_multi {
    ($name:ty {$($func:ident : $dkey:ident -> $dname:ident),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<Vec<$dname>> {
                return $dname::populate_multiple(&conn, &self.$dkey);
            })*
        }
    };
}

macro_rules! link_label {
    ($name:ty {$($func:ident : $dkey:ident -> TextLabel),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<String> {
                let label = TextLabel::populate(&conn, &self.$dkey)?;
                return Ok(label._Text);
            })*
        }
    };
}

macro_rules! link_hit_attr_levels {
    ($name:ty {$($func:ident : $dkey:ident -> PlayerActionHitAttribute),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<Vec<PlayerActionHitAttribute>> {
                let len = self.$dkey.len();
                if len > 4 && &self.$dkey[(len-4)..(len-2)] == "LV"{
                    let skey = &self.$dkey[0..(len-2)].to_string();
                    return PlayerActionHitAttribute::populate_conditionally(&conn, "PlayerActionHitAttribute._Id LIKE ? || \'%\'", &[&skey]);
                }else{
                    return PlayerActionHitAttribute::populate_multiple(&conn, &self.$dkey);
                }
            })*
        }
    };
}

macro_rules! link_burst_attack_actions {
    ($name:ty {$($func:ident : $dkey:ident -> PlayerAction),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<Vec<PlayerAction>> {
                return PlayerAction::populate_conditionally(&conn, "PlayerAction._Id >= ? AND PlayerAction._Id < ?", &[&self.$dkey, &(&self.$dkey+5)]);
            })*
        }
    };
}

macro_rules! link_combo_chain_actions {
    ($name:ty {$($func:ident : $dkey:ident, $cmaxkey:ident -> PlayerAction),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<Vec<PlayerAction>> {
                return PlayerAction::populate_conditionally(&conn, "PlayerAction._Id >= ? AND PlayerAction._Id < ?", &[&self.$dkey, &(self.$dkey+self.$cmaxkey)]);
            })*
        }
    };
}

macro_rules! from_sql_enum(
    (pub enum $name:ident {$($ename:ident = $evalue:tt),*}) => {
        #[derive(Debug, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            UNKNOWN = -1,
            NONE = 0,
            $($ename = $evalue),*
        }
        impl FromSql for $name {
            fn column_result(value: ValueRef) -> FromSqlResult<Self> {
                i64::column_result(value).and_then(|i| match i {
                    $($evalue => Ok($name::$ename)),*,
                    0 => Ok($name::NONE),
                    _ => Ok($name::UNKNOWN)
                })
            }
        }
        impl Default for $name {
            fn default() -> Self { $name::NONE }
        }
    }
);

pub mod mappings {
    extern crate rusqlite;
    use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

    from_sql_enum! {
        pub enum SkillIndex {
            S1 = 1,
            S2 = 2,
            S3 = 3
        }
    }

    from_sql_enum! {
        pub enum Affliction {
            // 1: Poison, 2: Burn, 3: Freeze, 4: Paralysis, 5: Blind, 6: Stun, 7: Curse, 8: UNKNOWN08, 9: Bog, 10: Sleep, 11: Frostbite, 103: Def down, 198: Buffed, 201: Break
            POISON = 1,
            BURN = 2,
            FREEZE = 3,
            PARALYSIS = 4,
            BLIND = 5,
            STUN = 6,
            CURSE = 7,
            UNKNOWN08 = 8,
            BOG = 9,
            SLEEP = 10,
            FROSTBITE = 11,
            RECOVERY = 99
        }
    }

    from_sql_enum! {
        pub enum Element {
            FLAME = 1,
            WATER = 2,
            WIND = 3,
            LIGHT = 4,
            SHADOW = 5
        }
    }

    from_sql_enum! {
        pub enum Weapon {
            SWORD = 1,
            BLADE = 2,
            DAGGER = 3,
            AXE = 4,
            LANCE = 5,
            BOW = 6,
            WAND = 7,
            STAFF = 8
        }
    }
}

pub mod action;
pub use action::*;
pub mod ability;
pub use ability::*;
pub mod skill;
pub use skill::*;
pub mod label;
pub use label::*;
pub mod character;
pub use character::*;
pub mod equipment;
pub use equipment::*;

#[cfg(test)]
mod tests {
    extern crate rusqlite;
    use super::{TextLabel, DB_FILE};
    use rusqlite::{Connection, Result};

    #[test]
    fn sanity() -> Result<()> {
        let conn = Connection::open(DB_FILE)?;
        let res = TextLabel::populate(&conn, &"CHARA_NAME_10140101".to_string())?;
        assert_eq!(res._Text, "Euden");
        Ok(())
    }
}
