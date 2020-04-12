extern crate rusqlite;
use rusqlite::{Connection, Result};

db_data_struct! {
    pub struct SkillChainData {
        _Id: u32,
        _GroupId: u32,
        _ActivateCondition: u32,
        _ConditionArgs1: u32
    }
}

db_data_struct! {
    pub struct SkillData {
        _Id: u32,
        _Name: String,
        // _SkillLv1IconName: String,
        // _SkillLv2IconName: String,
        // _SkillLv3IconName: String,
        // _SkillLv4IconName: String,
        // _Description1: String,
        // _Description2: String,
        // _Description3: String,
        // _Description4: String,
        _Sp: u32,
        _SpLv2: u32,
        // _ActionId1: u32,
        _ActionId2: u32,
        _ActionId3: u32,
        _ActionId4: u32,
        _AdvancedSkillLv1: u32,
        _AdvancedActionId1: u32,
        _Ability1: u32,
        _Ability2: u32,
        _Ability3: u32,
        _Ability4: u32,
        _TransSkill: u32,
        _TransCondition: u32,
        _TransHitCount: u32,
        _ChainGroupId: u32,
        _MaxUseNum: u32, // psiren
        _ModeChange: u32,
        // _Support: u32, // special helper skill
        // _TransText: String,
        _TransTime: f64,
        _TransBuff: u32,
        // _MaxAdditionalCount: u32,
        _IsAffectedByTension: bool
        // _VoiceType: u32,
        // _OtherVoice: String,
        // _FocusType: u32,
        // _ZoominTime: f64,
        // _ZoominDistance: f64,
        // _Zoom2Time: f64,
        // _Zoom2Distance: f64,
        // _AtPositionDistance: f64,
        // _ZoomWaitTime: f64
    }
}
