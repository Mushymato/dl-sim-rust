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
        // _MinHp3: u32,
        // _MinHp4: u32,
        // _MinHp5: u32,
        _MaxHp: u32,
        _AddMaxHp1: u32,
        _PlusHp0: u32,
        _PlusHp1: u32,
        _PlusHp2: u32,
        _PlusHp3: u32,
        _PlusHp4: u32,
        _PlusHp5: u32,
        _McFullBonusHp5: u32,
        // _MinAtk3: u32,
        // _MinAtk4: u32,
        // _MinAtk5: u32,
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
        // _SkillChainValidTime: f64, // Galex: 3.0s
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

db_data_struct! {
    pub struct AmuletData {
        _Id: u32,
        pub _Name: String,
        // _Rarity: u32,
        // _AmuletType: u32,
        // _MinHp: u32,
        pub _MaxHp: u32,
        // _MinAtk: u32,
        pub _MaxAtk: u32,
        pub _BaseId: u32,
        pub _VariationId: u32,
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
        pub _IsPlayable: u32
    }
}

db_data_struct! {
    pub struct DragonData {
        _Id: u32,
        pub _Name: String,
        pub _SecondName: String,
        // _EmblemId: u32,
        _Rarity: u32,
        pub _ElementalType: Element,
        pub _BaseId: u32,
        pub _VariationId: u32,
        pub _IsPlayable: u32,
        // _AnimFileName: String,
        // _Deco1: u32,
        // _Deco2: u32,
        // _MinHp: u32,
        pub _MaxHp: u32,
        // _MinAtk: u32,
        pub _MaxAtk: u32,
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

db_data_struct! {
    pub struct WeaponData {
        _Id: u32,
        pub _Name: String,
        pub _Type: Weapon,
        _Rarity: u32,
        pub _ElementalType: Element,
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
