extern crate rusqlite;
use crate::data::mappings::{Element, Weapon};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{Connection, Result};

from_sql_enum! {
    pub enum ModeChange {
        // 2: fs, 7: auto
        SKILL = 1,
        STANCE = 2,
        DRAGON = 3
    }
}

db_data_struct! {
    pub struct CharaData {
        _Id: u32,
        pub _Name: String,
        pub _SecondName: String,
        // _EmblemId: u32,
        pub _WeaponType: Weapon,
        pub _Rarity: u8,
        _MaxLimitBreakCount: u8,
        pub _ElementalType: Element,
        // _CharaType: u32,
        pub _BaseId: u32,
        pub _VariationId: u32,
        // stats related
        _MinHp3: u32,
        _MinHp4: u32,
        _MinHp5: u32,
        _MaxHp: u32,
        _AddMaxHp1: u32,
        _PlusHp0: u32,
        _PlusHp1: u32,
        _PlusHp2: u32,
        _PlusHp3: u32,
        _PlusHp4: u32,
        _PlusHp5: u32,
        _McFullBonusHp5: u32,
        _MinAtk3: u32,
        _MinAtk4: u32,
        _MinAtk5: u32,
        _MaxAtk: u32,
        _AddMaxAtk1: u32,
        _PlusAtk0: u32,
        _PlusAtk1: u32,
        _PlusAtk2: u32,
        _PlusAtk3: u32,
        _PlusAtk4: u32,
        _PlusAtk5: u32,
        _McFullBonusAtk5: u32,
        _MinDef: u32,
        _DefCoef: f64,
        // modes
        _ModeChangeType: ModeChange,
        _ModeId1: u32,
        _ModeId2: u32,
        _ModeId3: u32,
        _OriginCombo: u32,
        // _Mode1Combo: u32,
        // _Mode2Combo: u32,
        _BurstAttack: u32,
        _DashAttack: u32,
        _Avoid: u32,
        // _Appear: u32,
        // _Revive: u32,
        // _Disappear: u32,
        // _Win: u32,
        // _EffNameCriticalHit: String,
        _Skill1: u32,
        _Skill2: u32,
        // _SkillChainValidTime: f64, // Galex: 3,0s
        _Abilities11: u32,
        _Abilities12: u32,
        _Abilities13: u32,
        // _Abilities14: u32,
        _Abilities21: u32,
        _Abilities22: u32,
        _Abilities23: u32,
        // _Abilities24: u32,
        _Abilities31: u32,
        _Abilities32: u32,
        _Abilities33: u32,
        // _Abilities34: u32,
        _ExAbilityData5: u32,
        _ExAbility2Data5: u32,
        // _ChargeType: u32, // can move during charge?
        _MaxChargeLv: u32, // mh fs
        _UniqueDragonId: u32,
        pub _IsPlayable: u32
    }
}
