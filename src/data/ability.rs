extern crate rusqlite;
use crate::data::mappings::*;
use rusqlite::{Connection, Result};

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
        _ShiftGroupId: u32, // see AbilityShiftGroup
        // _HeadText: String,
        _AbilityType1: u32,
        _VariousId1a: u32,
        _VariousId1b: u32,
        _VariousId1c: u32,
        _VariousId1str: String,
        _AbilityLimitedGroupId1: u32, // see AbilityLimitedGroup
        _TargetAction1: TargetAction,
        _AbilityType1UpValue: f64,
        _AbilityType2: u32,
        _VariousId2a: u32,
        _VariousId2b: u32,
        _VariousId2c: u32,
        _VariousId2str: String,
        _AbilityLimitedGroupId2: u32, // see AbilityLimitedGroup
        _TargetAction2: TargetAction,
        _AbilityType2UpValue: f64,
        _AbilityType3: u32,
        _VariousId3a: u32,
        _VariousId3b: u32,
        _VariousId3c: u32,
        _VariousId3str: String,
        _AbilityLimitedGroupId3: u32, // see AbilityLimitedGroup
        _TargetAction3: TargetAction,
        _AbilityType3UpValue: f64
    }
}

// use crate::data::hit::{ActionCondition, PlayerActionHitAttribute};

// link_data_struct!(
//     ActionCondition {
//         link_damaged_hit_attr: _DamageLink -> PlayerActionHitAttribute,
//         link_remove_condition: _RemoveConditionId -> ActionCondition
//     }
// );
