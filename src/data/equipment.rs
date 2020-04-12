use std::collections::HashMap;

extern crate rusqlite;
use crate::data::ability::AbilityData;
use crate::data::action::PlayerAction;
use crate::data::label::TextLabel;
use crate::data::skill::SkillData;
use rusqlite::{Connection, Result, NO_PARAMS};

db_data_struct! {
    pub struct AmuletData {
        _Id: u32,
        _Name: String,
        // _Rarity: u32,
        // _AmuletType: u32,
        // _MinHp: u32,
        _MaxHp: u32,
        // _MinAtk: u32,
        _MaxAtk: u32,
        _BaseId: u32,
        _VariationId: u32,
        // _Abilities11: u32,
        // _Abilities12: u32,
        _Abilities13: u32,
        // _Abilities21: u32,
        // _Abilities22: u32,
        _Abilities23: u32,
        // _Abilities31: u32,
        // _Abilities32: u32,
        // _Abilities33: u32,
        // _LimitBreakId: u32,
        _IsPlayable: u32
    }
}

link_data_struct! {
    AmuletData {
        link_ability_1: _Abilities13 -> AbilityData,
        link_ability_2: _Abilities23 -> AbilityData
    }
}

link_label! {
    AmuletData {
        link_name: _Name -> TextLabel
    }
}

db_data_struct! {
    pub struct DragonData {
        _Id: u32,
        _Name: String,
        _SecondName: String,
        // _EmblemId: u32,
        _Rarity: u32,
        _ElementalType: u8,
        _BaseId: u32,
        _VariationId: u32,
        _IsPlayable: u32,
        // _AnimFileName: String,
        // _Deco1: u32,
        // _Deco2: u32,
        // _MinHp: u32,
        _MaxHp: u32,
        // _MinAtk: u32,
        _MaxAtk: u32,
        // _LimitBreakId: u32,
        _Skill1: u32,
        // _Abilities11: u32,
        _Abilities12: u32,
        // _Abilities21: u32,
        _Abilities22: u32,
        // _DragonExplosionDetail: String,
        // _DragonExplosionIcon: String,
        // _Profile: String,
        // _FavoriteType: u32,
        // _VoiceType: u32,
        // _CvInfo: String,
        // _CvInfoEn: String,
        // _SellCoin: u32,
        // _SellDewPoint: u32,
        // _AvoidActionFront: u32, // not same as back for some dragons
        _AvoidActionBack: u32,
        _Transform: u32,
        _DefaultSkill: u32,
        _ComboMax: u32
        // _ScaleSize: f64,
        // _DcScaleSize: f64,
        // _DcRotationY: f64,
        // _ShadowSize: f64,
        // _GaugeHeightOffset: f64,
        // _MoveSpeed: f64,
        // _DashSpeedRatio: f64,
        // _TurnSpeed: f64,
        // _IsTurnToDamageDir: u32,
        // _IsLongRangeInGame: u32,
        // _DragonCameraDistance: f64,
        // _MoveType: u32,
        // _IsLongLange: u32,
        // _IsDetailimage: u32,
        // _SilhouetteValue: u32
    }
}

link_data_struct! {
    DragonData {
        link_skill_data: _Skill1 -> SkillData,
        link_ability_1: _Abilities12 -> AbilityData,
        link_ability_2: _Abilities22 -> AbilityData,
        link_transform_action: _Transform -> PlayerAction,
        link_dodge_action: _AvoidActionBack -> PlayerAction
    }
}

link_combo_chain_actions! {
    DragonData {
        link_combo_actions: _DefaultSkill, _ComboMax -> PlayerAction
    }
}

link_label! {
    DragonData {
        link_name: _Name -> TextLabel,
        link_secondname: _SecondName -> TextLabel
    }
}

db_data_struct! {
    pub struct WeaponData {
        _Id: u32,
        _Name: String,
        _Type: u8,
        _Rarity: u32,
        _ElementalType: u8,
        // _MinHp: u32,
        _MaxHp: u32,
        // _MinAtk: u32,
        _MaxAtk: u32,
        // _LimitBreakId: u32,
        _BaseId: u32,
        _VariationId: u32,
        // _FormId: u32,
        // _DecBaseId: u32,
        // _DecVariationId: u32,
        // _BulletBaseId: u32,
        // _BulletVariationId: u32,
        _Skill: u32,
        _Abilities11: u32,
        _Abilities21: u32,
        _IsPlayable: u32,
        // _Text: String,
        // _SellCoin: u32,
        // _SellDewPoint: u32,
        _GradeId: u8,
        _CraftSeriesId: u8
    }
}

link_data_struct! {
    WeaponData {
        link_skill_data: _Skill -> SkillData,
        link_ability_1: _Abilities11 -> AbilityData,
        link_ability_2: _Abilities21 -> AbilityData
    }
}

link_label! {
    WeaponData {
        link_name: _Name -> TextLabel
    }
}
