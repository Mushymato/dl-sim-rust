use std::collections::HashMap;

extern crate rusqlite;
use crate::data::ability::{AbilityData, ExAbilityData};
use crate::data::action::PlayerAction;
use crate::data::equipment::DragonData;
use crate::data::label::TextLabel;
use crate::data::skill::SkillData;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{Connection, Result, NO_PARAMS};

from_sql_enum! {
    pub enum Tribe {
        THAUMIAN = 1,
        PHYSIAN = 2,
        DEMIHUMAN = 3,
        THERION = 4,
        UNDEAD = 5,
        DEMON = 6,
        HUMAN = 7,
        DRAGON = 8
    }
}

from_sql_enum! {
    pub enum DpDrop {
        ON_DEATH = 1,
        ON_INCREMENT = 2
    }
}

db_data_struct! {
    pub struct EnemyList {
        _Id: u32,
        _Name: String,
        _TribeType: Tribe
    }
}

link_label! {
    EnemyList {
        link_name: _Name -> TextLabel
    }
}

db_data_struct! {
    pub struct EnemyData {
        _Id: u32,
        _BaseId: u32,
        _VariationId: u32,
        _ElementalType: u8,
        _BookId: u32,
        _BreakDuration: f64,
        _BreakAtkRate: f64,
        _BreakDefRate: f64,
        _ObAtkRate: f64
    }
}

link_data_struct! {
    EnemyData {
        link_enemy_list: _BookId -> EnemyList
    }
}

db_data_struct! {
    pub struct EnemyParam {
        _Id: u32,
        _ParamGroupName: String,
        _DataId: u32,
        // _Ai: String,
        // _ActionSet: u32,
        // _RouteMode: u32,
        // _Ability01: u32,
        // _Ability02: u32,
        // _Ability03: u32,
        // _Ability04: u32,
        // _Tough: u32,
        // _RareStayTime: f64,
        // _IsHideUI: u32,
        // _IsDeadRetention: u32,
        // _IsDeadNoEff: u32,
        // _IconType: u32,
        _DropDpPattern: DpDrop,
        _DropDpValue: u32,
        _HP: u32,
        _Atk: u32,
        _Def: u32,
        // _Overwhelm: u32,
        _BaseOD: u32,
        _BaseBreak: u32,
        // _CounterRate: u32,
        // _BarrierRate: u32,
        // _ActionBehaviorSec: f64,
        // _GetupActionRate: u32,
        // _IsTargetSpecialHate: u32,
        // _UniqueGroup: u32,
        _RegistAbnormalRate01: u32,
        _RegistAbnormalRate02: u32,
        _RegistAbnormalRate03: u32,
        _RegistAbnormalRate04: u32,
        _RegistAbnormalRate05: u32,
        _RegistAbnormalRate06: u32,
        _RegistAbnormalRate07: u32,
        _RegistAbnormalRate08: u32,
        _RegistAbnormalRate09: u32,
        _RegistAbnormalRate10: u32,
        _Form2nd: u32
        // _Child01Param: u32,
        // _Child01Num: u32,
        // _Child02Param: u32,
        // _Child02Num: u32,
        // _Child03Param: u32,
        // _Child03Num: u32,
        // _IsChildIgnoreCollide: u32,
        // _PartsA: u32,
        // _PartsB: u32,
        // _PartsC: u32,
        // _PartsD: u32,
        // _PartsNode: String,
        // _CrashedHpRate: u32,
        // _BossAppearVoiceId: String,
        // _KeepLegacyRotateToTarget: u32
    }
}

link_data_struct! {
    EnemyParam {
        link_enemy_data: _DataId -> EnemyData
    }
}
