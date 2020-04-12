extern crate rusqlite;
use crate::data::ability::{AbilityData, ExAbilityData};
use crate::data::action::PlayerAction;
use crate::data::equipment::DragonData;
use crate::data::label::TextLabel;
use crate::data::mappings::{Element, Weapon};
use crate::data::skill::SkillData;
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
    pub struct CharaUniqueCombo {
        _Id: u32,
        _ActionId: u32,
        _MaxComboNum: u32,
        _ShiftConditionType: ModeChange,
        _ConditionArgs1: u32,
        _BuffHitAttribute: String
    }
}

link_combo_chain_actions! (
    CharaUniqueCombo {
        link_combo_actions: _ActionId, _MaxComboNum -> PlayerAction
    }
);

db_data_struct! {
    pub struct CharaModeData {
        _Id: u32,
        _ActionId: u32,
        _UniqueComboId: u32,
        _Skill1Id: u32,
        _Skill2Id: u32,
        _BurstAttackId: u32
        // _EffectName: String,
        // _EffectTrigger: u32,
        // _EffectAttachName: String,
        // _IconName: String,
        // _Text String
    }
}

link_data_struct!(
    CharaModeData {
        link_action: _ActionId -> PlayerAction,
        link_unique_combo: _UniqueComboId -> CharaUniqueCombo,
        link_skill_1: _Skill1Id -> SkillData,
        link_skill_2: _Skill2Id -> SkillData
    }
);

link_burst_attack_actions! (
    CharaModeData {
        link_burst_attack_actions: _BurstAttackId -> PlayerAction
    }
);

db_data_struct! {
    pub struct CharaData {
        _Id: u32,
        _Name: String,
        _SecondName: String,
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
        _Abilities14: u32,
        _Abilities21: u32,
        _Abilities22: u32,
        _Abilities23: u32,
        _Abilities24: u32,
        _Abilities31: u32,
        _Abilities32: u32,
        _Abilities33: u32,
        _Abilities34: u32,
        _ExAbilityData5: u32,
        _ExAbility2Data5: u32,
        // _ChargeType: u32, // can move during charge?
        _MaxChargeLv: u32, // mh fs
        _UniqueDragonId: u32,
        pub _IsPlayable: u32
    }
}

link_data_struct!(
    CharaData {
        link_mode_1: _ModeId1 -> CharaModeData,
        link_mode_2: _ModeId2 -> CharaModeData,
        link_mode_3: _ModeId3 -> CharaModeData,
        link_origin_combo: _OriginCombo -> CharaUniqueCombo,
        link_skill_1: _Skill1 -> SkillData,
        link_skill_2: _Skill2 -> SkillData,
        link_ex1_ability: _ExAbilityData5 -> ExAbilityData,
        link_ex2_ability: _ExAbility2Data5 -> AbilityData,
        link_unique_dragon: _UniqueDragonId -> DragonData,
        link_dash_attack_action: _DashAttack -> PlayerAction,
        link_dodge_action: _Avoid -> PlayerAction
    }
);

link_burst_attack_actions! {
    CharaData {
        link_burst_attack_actions: _BurstAttack -> PlayerAction
    }
}

link_label! {
    CharaData {
        link_name: _Name -> TextLabel,
        link_fullname: _SecondName -> TextLabel
    }
}

impl CharaData {
    fn max_hp(&self) -> u32 {
        let mut s = self._MaxHp;
        s += self._PlusHp0;
        s += self._PlusHp1;
        s += self._PlusHp2;
        s += self._PlusHp3;
        s += self._PlusHp4;
        s += self._McFullBonusHp5;
        if self._MaxLimitBreakCount > 4 {
            s += self._AddMaxHp1;
            s += self._PlusHp5;
        }
        return s;
    }

    fn max_atk(&self) -> u32 {
        let mut s = self._MaxAtk;
        s += self._PlusAtk0;
        s += self._PlusAtk1;
        s += self._PlusAtk2;
        s += self._PlusAtk3;
        s += self._PlusAtk4;
        s += self._McFullBonusAtk5;
        if self._MaxLimitBreakCount > 4 {
            s += self._AddMaxAtk1;
            s += self._PlusAtk5;
        }
        return s;
    }
}
