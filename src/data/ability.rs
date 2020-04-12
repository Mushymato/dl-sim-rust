extern crate rusqlite;
use crate::data::mappings::{Element, SkillIndex, Weapon};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{Connection, Result};

from_sql_enum! {
    pub enum TargetAction {
        // 2: fs, 7: auto
        BURST = 2,
        SKILL = 6,
        AUTO = 7
    }
}

db_data_struct! {
    pub struct ExAbilityData {
        _Id: u32,
        // _Name: String,
        // _Details: String,
        // _Category: u32,
        // _AbilityIconName: String,
        // _PartyPowerWeight: u32,
        // _UnitType: u32,
        // _ElementalType: u32,
        // _WeaponType: u32,
        _ConditionType: u32,
        _ConditionValue: f64,
        _Probability: u32,
        _AbilityType1: u32,
        _VariousId1: u32,
        _TargetAction1: TargetAction,
        _AbilityType1UpValue0: f64,
        _AbilityType2: u32,
        _VariousId2: u32,
        _TargetAction2: TargetAction,
        _AbilityType2UpValue0: f64,
        _AbilityType3: u32,
        _VariousId3: u32,
        _TargetAction3: TargetAction,
        _AbilityType3UpValue0: f64
    }
}

db_data_struct! {
    pub struct AbilityLimitedGroup {
        _Id: u32,
        _IsEffectMix: bool,
        _MaxLimitedValue: f64
    }
}

db_data_struct! {
    pub struct AbilityData {
        _Id: u32,
        // _EventId: u32,
        _PartyPowerWeight: u32, // might
        // _Name: String, // label
        // _Details: String, // label
        // _ViewAbilityGroupId1: u32,
        // _ViewAbilityGroupId2: u32,
        // _ViewAbilityGroupId3: u32,
        // _AbilityIconName: String,
        _UnitType: u8, // 0: self, 1: team?
        _ElementalType: Element,
        _WeaponType: Weapon,
        _OnSkill: SkillIndex,
        // _SkillOwner: u32,
        // _OwnerMode: u32,
        _ConditionType: u32,
        _ExpireCondition: bool, // used for afflict guards
        _ConditionValue: f64,
        _Probability: u32,
        _OccurenceNum: u32, // number of times buff can happen
        _MaxCount: u32, // kinda like above, but different :v
        _CoolTime: f64, // cd in seconds
        _TargetAction: TargetAction,
        // _ShiftGroupId: u32, // see AbilityShiftGroup
        // _HeadText: String,
        _AbilityType1: u32,
        _VariousId1a: u32,
        _VariousId1b: u32,
        _VariousId1c: u32,
        _VariousId1str: String,
        _AbilityLimitedGroupId1: u32,
        _TargetAction1: TargetAction,
        _AbilityType1UpValue: f64,
        _AbilityType2: u32,
        _VariousId2a: u32,
        _VariousId2b: u32,
        _VariousId2c: u32,
        _VariousId2str: String,
        _AbilityLimitedGroupId2: u32,
        _TargetAction2: TargetAction,
        _AbilityType2UpValue: f64,
        _AbilityType3: u32,
        _VariousId3a: u32,
        _VariousId3b: u32,
        _VariousId3c: u32,
        _VariousId3str: String,
        _AbilityLimitedGroupId3: u32,
        _TargetAction3: TargetAction,
        _AbilityType3UpValue: f64
    }
}

// use crate::data::hit::{ActionCondition, PlayerActionHitAttribute};

link_data_struct!(
    AbilityData {
        link_ability_limited_group_1: _AbilityLimitedGroupId1 -> AbilityLimitedGroup,
        link_ability_limited_group_2: _AbilityLimitedGroupId2 -> AbilityLimitedGroup,
        link_ability_limited_group_3: _AbilityLimitedGroupId3 -> AbilityLimitedGroup
    }
);
