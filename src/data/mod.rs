pub const DB_FILE: &str = "dl.sqlite";

/* define */

macro_rules! db_data_struct {
    (pub struct $name:ident { $pkname:ident : $pktype:ty, $($fname:ident : $ftype:ty),* }) => {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct $name {
            $pkname : $pktype,
            $($fname : $ftype),*
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
                    stringify!(SELECT $pkname, $(($fname)),* FROM $name WHERE $pkname=?;),
                    &[&pk],
                    |r| {
                    Ok($name {
                        $pkname : r.get(0)?,
                        $($fname: r.get(counter())?),*
                    })
                });
            }
        }
    }
}

macro_rules! link_data_struct {
    ($name:ty {$($func:ident : $dkey:ident -> $dname:ident),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection, cb: fn(&$dname)) {
                match $dname::populate(&conn, &self.$dkey){
                    Ok(r) => cb(&r),
                    Err(_e) => (),
                }
            })*
        }
    };
}

pub mod mappings {
    extern crate rusqlite;
    use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
    macro_rules! from_sql_enum(
        (pub enum $name:ident {$($ename:ident = $evalue:tt),*}) => {
            #[derive(Debug)]
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
        }
    );

    from_sql_enum! {
        pub enum HitExecType {
            ATTACK = 1,
            BUFF = 2
        }
    }

    from_sql_enum! {
        pub enum TargetGroup {
            // targets, 1: self, 2: team, 3: enemy, 4: ?, 5: dodge, 6: also team, 7: lowest hp teammate, 8: buffs(?)
            SELF = 1,
            TEAM = 2,
            ENEMY = 3,
            DODGE = 5,
            TEAMBUFF = 6,
            TEAMMATE = 7,
            ENEMYHIT = 8
        }
    }

    from_sql_enum! {
        pub enum TargetAction {
            // 2: fs, 7: auto
            BURST = 2,
            AUTO = 7
        }
    }

    from_sql_enum! {
        pub enum SkillIndex {
            S1 = 1,
            S2 = 2,
            S3 = 3
        }
    }

    from_sql_enum! {
        pub enum KillerState {
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
            DEFDOWN = 103,
            BUFFED = 198,
            BREAK = 201
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

pub mod hit;
pub use hit::*;
pub mod ability;
pub use ability::*;
