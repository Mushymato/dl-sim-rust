use std::collections::HashMap;

extern crate rusqlite;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{Connection, Result, NO_PARAMS};

from_sql_enum! {
    pub enum TargetAction {
        BURST = 2,
        S1 = 3,
        S2 = 4,
        SKILL = 6,
        AUTO = 7
    }
}

from_sql_enum! {
    pub enum ConditionType {
        HP_GEQ = 1,
        HP_LEQ = 2,
        // 3
        BUFFED = 4,
        TRANSFORMED = 5,
        BREAK = 6,
        // 7
        DOUBLEBUFF = 8,
        COMBO_GEQ = 9,
        HP_GEQ_CHAIN = 10, // for chains
        SLAYER = 11, // STRIKER if target action = 2
        SHIFT = 12,
        HP_GEQ_BUFF = 13, // for chains
        HP_UNDER = 14,
        QUEST_START = 15,
        OVERDRIVE = 16,
        // 17
        WHILE_ENERGIZED = 18, // only on yaten s1
        ON_ENERGIZED = 19,
        BLEED = 20,
        COMBO_COUNTER = 21,
        // 22 23 24
        AUTOCHARGE_S1 = 25,
        // 26 27
        AUTOCHARGE_S2 = 28,
        ON_AFFLICTION = 29,
        ON_RESIST = 30,
        // TRANSFORM = 31
        TEAMMATES_ALIVE = 32,
        // 33
        ENERGY_LEVEL = 34,
        // 35
        ENERGY_BUFFED = 36,
        HP_LEQ_HIT_ATTR = 37,
        // 38
        PRIMED = 39,
        // 40 41 42
        ON_DEFDOWN = 43,
        BUFF_ICONS = 44,
        ON_CRIT = 45,
        KNOCKED_BACK = 46,
        NOT_KNOCKED_BACK = 47,
        BUFF_BY = 48,
        WHILE_DEFDOWN = 50
    }
}

from_sql_enum! {
    pub enum AbilityType {
        STAT = 1,
        AFFLICTION_RESIST = 2,
        AFFLICTION_PROC_RATE = 3,
        TRIBE_RESIST = 4,
        TRIBE_BANE = 5,
        DAMAGE = 6, // skill damage, fs damage, etc
        CRITICAL_RATE = 7,
        RECOVERY_POTENCY = 8,
        GAUGE_ACCELERATOR = 9,
        // 10
        STRIKING_HASTE = 11,
        // 12 13
        ACTION_CONDITON = 14,
        // 15
        DEBUFF_CHANCE = 16,
        SKILL_PREP = 17,
        BUFF_TIME = 18,
        // 19
        AFFLICTION_PUNISHER = 20,
        PLAYER_EXP = 21,
        ADVENTURER_EXP = 22,
        RUPIES = 23,
        MANA = 24,
        COND_ACTION_GRANT = 25,
        CRITICAL_DAMAGE = 26,
        SHAPESHIFT_PREP = 27,
        ELEMENTAL_RESIST = 28,
        ENEMY_RESIST = 29,
        ENEMY_BANE = 30,
        // 31 32
        EVENT_POINTS = 33,
        EVENT_DROPS = 34,
        GAUGE_INHIBITOR = 35,
        DRAGON_DAMAGE = 36,
        DULL_RESIST = 37,
        // 38
        ACTION_GRANT = 39,
        GAUGE_DEF = 40,
        // 41
        // ??? = 42
        ABILITY_REF = 43,
        ACTION = 44,
        // 45
        DRAGON_TIME_ADD = 46,
        // 47
        DRAGON_TIMER_RATE = 48,
        SHAPESHIFT_FILL = 49,
        // 50
        RANDOM_BUFF = 51,
        COMBO_DMG_BOOST = 54,
        COMBO_TIME = 55,
        DRAGONDRIVE = 56
        // 57
        // ??? = 58
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
        _AbilityType1: AbilityType,
        _VariousId1: u32,
        _TargetAction1: TargetAction,
        _AbilityType1UpValue0: f64,
        _AbilityType2: AbilityType,
        _VariousId2: u32,
        _TargetAction2: TargetAction,
        _AbilityType2UpValue0: f64,
        _AbilityType3: AbilityType,
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
        _ElementalType: u8,
        _WeaponType: u8,
        _OnSkill: u8,
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
        _AbilityType1: AbilityType,
        _VariousId1a: u32,
        _VariousId1b: u32,
        _VariousId1c: u32,
        _VariousId1str: String,
        _AbilityLimitedGroupId1: u32,
        _TargetAction1: TargetAction,
        _AbilityType1UpValue: f64,
        _AbilityType2: AbilityType,
        _VariousId2a: u32,
        _VariousId2b: u32,
        _VariousId2c: u32,
        _VariousId2str: String,
        _AbilityLimitedGroupId2: u32,
        _TargetAction2: TargetAction,
        _AbilityType2UpValue: f64,
        _AbilityType3: AbilityType,
        _VariousId3a: u32,
        _VariousId3b: u32,
        _VariousId3c: u32,
        _VariousId3str: String,
        _AbilityLimitedGroupId3: u32,
        _TargetAction3: TargetAction,
        _AbilityType3UpValue: f64
    }
}

link_data_struct! {
    AbilityData {
        link_ability_limited_group_1: _AbilityLimitedGroupId1 -> AbilityLimitedGroup,
        link_ability_limited_group_2: _AbilityLimitedGroupId2 -> AbilityLimitedGroup,
        link_ability_limited_group_3: _AbilityLimitedGroupId3 -> AbilityLimitedGroup
    }
}
