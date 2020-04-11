extern crate rusqlite;
use crate::data::mappings::{Affliction, SkillIndex};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{Connection, Result};

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

db_data_struct! {
    pub struct PlayerActionHitAttribute {
        _Id: String, // name of this hit attr
        // _FontEffect: String,
        _HitExecType: HitExecType, // 1: attack, 2: buff
        _TargetGroup: TargetGroup, // targets, 1: self, 2: team, 3: enemy, 4: ?, 5: dodge, 6: also team, 7: lowest hp teammate, 8: buffs(?)
        // _Elemental01: u8,
        // _Elemental02: u8,
        // _Attributes02: u8,
        // _Attributes03: u8,
        // _LookToDamageType: u8,
        // _Attributes04: u8,
        // _Attributes05: u8,
        // _Attributes07: u8,
        // _Attributes08: u8,
        // _AttrIgnoreBarrier: bool, // ignore enemy barriers
        // _AttrNoReaction: u8, // something related to enemy absorb
        // _AttrShare: u8,
        _DamageAdjustment: f64, // base damage modifier
        _ToOdDmgRate: f64, // OD accel
        _ToBreakDmgRate: f64, // BK accel
        _IsDamageMyself: bool, // self damage
        _SetCurrentHpRate: f64, // set self hp %
        // _ConsumeHpRate: f64,
        _RecoveryValue: u32, // heal potency
        _AdditionRecoverySp: u32, // sp gain
        _RecoverySpRatio: f64, // sp gain %
        _RecoverySpSkillIndex: SkillIndex, // sp gain target, 0: all, 1: s1, etc
        _AdditionRecoveryDpPercentage: f64, // dp gain %
        _RecoveryDragonTime: f64, // dtime gain
        _AdditionRecoveryDpLv1: u32, // dp gain
        // _AdditionRecoveryDpAbility: u8,
        _RecoveryEp: u32, // ammo gain
        _AdditionActiveGaugeValue: u32, // granzal gauges
        _AdditionRecoveryUtp: u32, // utp gain
        _AddUtp: i64, // utp gain
        _FixedDamage: bool, // fixed damage
        _CurrentHpRateDamage: i64, // grav attack
        _HpDrainRate: f64, // uncapped hp drain %
        _HpDrainRate2: f64, // capped hp drain %
        _HpDrainLimitRate: f64, // limit
        // _HpDrainAttribute: String, // hit attr for the drain heal
        _DamageCounterCoef: f64, // counter damage
        _CrisisLimitRate: f64, // crisis mod
        // _DamageDispDelaySec: f64,
        _IsDisableHealSpOnCurse: bool, // no sp if cursed(?)
        _ActionCondition1: u32, // act cond stuff
        // _HeadText: String,
        // _ActionGrant: u32,
        _KillerState1: KillerState, // killer
        _KillerState2: KillerState,
        _KillerState3: KillerState,
        // 1: Poison, 2: Burn, 3: Freeze, 4: Paralysis, 5: Blind, 6: Stun, 7: Curse, 8: UNKNOWN08, 9: Bog, 10: Sleep, 11: Frostbite, 103: Def down, 198: Buffed, 201: Break
        _KillerStateDamageRate: f64, // killer modifier
        // _KillerStateRelease: u8,
        _DamageUpRateByBuffCount: f64 // more damage per buff stack
        // _SplitDamageCount: u32, // seems like enemy only stuff
        // _SplitDamageCount2: u32,
        // _ArmorBreakLv: u8 // iframe, 1: roll/buff, 4: skill iframe
        // _InvincibleBreakLv: u8,
        // _KnockBackType: u8,
        // _KnockBackDistance: f64,
        // _KnockBackDependsOnMass: bool,
        // _KnockBackDurationSec: f64,
        // _UseDamageMotionTimeScale: bool,
        // _DamageMotionTimeScale: f64,
        // _BlastHeight: f64,
        // _BlastAngle: f64,
        // _BlastGravity: f64,
    }
}

db_data_struct! {
    pub struct ActionCondition {
        _Id: u32,
        _Type: Affliction, // affliction type
        // _Text: String, // label
        // _TextEx: String, // label
        _UniqueIcon: u32, // icon id
        // _ResistBuffReset: u32,
        _UnifiedManagement: bool, // composite buffs
        _Overwrite: u8, // stacking 1: no stack, 2: nobu (?)
        _OverwriteIdenticalOwner: bool, // stacking when same userid 1: no stack
        _OverwriteGroupId: bool, // stacking group id
        // _UsePowerUpEffect: u32,
        _LostOnDragon: bool, // remove on dragon
        _RestoreOnReborn: bool, // restory on revive
        _Rate: u32, // rate in int %
        // _EfficacyType: u32,
        _RemoveConditionId: u32, // remove target act cond
        _DurationSec: f64, // duration
        _DurationNum: u32, // charges
        _MinDurationSec: f64, // min duration
        // _RemoveAciton: bool, // lol
        _SlipDamageIntervalSec: f64, // interval
        _SlipDamageFixed: i64, // fixed
        _SlipDamageRatio: f64, // percent
        // _SlipDamageMax: 0,
        _SlipDamagePower: f64, // modifier
        _RegenePower: f64, // heal modifier
        // _EventProbability: u64, // blind rate
        // _EventCoefficient: f64, // bog move speed
        // _DamageCoefficient: f64, // bog damage mod
        // _TargetAction: u8,
        _TargetElemental: u8, // bit flags, 00001: flame, 00010: water, 00100: wind, 01000: light, 10000: shadow
        // _ConditionAbs: 0,
        _ConditionDebuff: u8, // debuff cond, 16 for bleed
        _RateHP: f64, // hp
        _RateAttack: f64, // str
        _RateDefense: f64, // def down
        _RateDefenseB: f64, // zone def down
        _RateCritical: f64, // crit rate
        _RateSkill: f64, // skill dmg
        _RateBurst: f64, // fs
        _RateRecovery: f64, // rec potency
        _RateRecoverySp: f64, // skill haste
        _RateAttackSpeed: f64, // speed
        _RateChargeSpeed: f64, // fs charge speed
        // _RatePoison: f64,
        // _RateBurn: f64,
        // _RateFreeze: f64,
        // _RateParalysis: f64,
        // _RateDarkness: f64,
        // _RateSwoon: f64,
        // _RateCurse: f64,
        // _RateSlowMove: f64,
        // _RateSleep: f64,
        // _RateFrostbite: f64,
        // _RateFire: f64, // flame res
        // _RateWater: f64, // water res
        // _RateWind: f64, // wind res
        // _RateLight: f64, // light res
        // _RateDark: f64, // dark res
        // _RateMagicCreature: f64,
        // _RateNatural: f64,
        // _RateDemiHuman: f64,
        // _RateBeast: f64,
        // _RateUndead: f64,
        // _RateDeamon: f64,
        // _RateHuman: f64,
        // _RateDragon: f64,
        // _RateDamageCut: f64, // damage cut on hvan
        // _RateDamageCut2: f64,
        // _RateWeakInvalid: f64,
        // _HealInvalid: u32,
        // _ValidRegeneHP: f64,
        // _ValidRegeneSP: f64,
        // _ValidRegeneDP: f64,
        // _ValidSlipHp: f64,
        _UniqueRegeneSp01: f64, // sp regen
        _AutoRegeneS1: f64, // also sp regen (?)
        // _AutoRegeneSW: f64,
        // _RateReraise: f64,
        _RateArmored: f64, // knockback res
        _RateDamageShield: f64, // shield
        _RateDamageShield2: f64, // divergent shield
        _RateDamageShield3: f64, // nyaruko shield
        _RateSacrificeShield: f64, // life shield
        // _Malaise01: 0,
        // _Malaise02: 0,
        // _Malaise03: 0,
        // _RateNicked: f64,
        // _TransSkill: f64, somthing for skill shift?
        // _GrantSkill: 0,
        _DisableAction: u8, // laxi shut down
        _DisableMove: u8, // tobias alt x
        // _InvincibleLv: u32,
        _ComboShift: bool, // stance change
        _EnhancedBurstAttack: u32, // alt fs
        _EnhancedSkill1: u32, // alt s1
        _EnhancedSkill2: u32, // alt s2
        _EnhancedSkillWeapon: u32, // alt weapon skill, used for agito
        _EnhancedCritical: f64, // crit damage
        _Tension: bool, // can energize
        _Inspiration: bool, // can inspire
        _Sparking: bool, // electrified
        _RateHpDrain: f64, // hp drain buff
        // _HpDrainLimitRate: f64,
        // _SelfDamageRate: f64,
        _HpConsumptionRate: f64, // chelsea self damage
        // _HpConsumptionCoef: f64,
        _RemoveTrigger: bool, // trigger effect on removal, burning ambition
        _DamageLink: String // hit attr, after taking damage
        // _ExtraBuffType: 0
    }
}

from_sql_enum! {
    pub enum CommandType{
        PARTS_MOTION = 2,
        MOVEMENT = 5,
        ROTATION = 7,
        MARKER = 8,
        BULLET = 9,
        HIT = 10,
        EFFECT = 11,
        SOUND = 12,
        CAMERA_MOTION = 13,
        SEND_SIGNAL = 14,
        ACTIVE_CANCEL = 15,
        TARGETING = 17, // spin cancel ?
        ACTION_END = 23,
        MULTI_BULLET = 24,
        ANIMATION = 25, // or maybe effects ?
        CONTROL = 30, // some kinda control related thing
        COLLISION = 37, // seen in arrange bullet
        PARABOLA_BULLET = 41,
        TIMESTOP = 48, // control animation playback ?
        TIMECURVE = 49, // control animation playback ?
        PIVOT_BULLET = 53,
        MOVEMENT_IN_SKILL = 54, // only eze s1 uses this
        ROTATION_IN_SKILL = 55,
        FIRE_STOCK_BULLET = 59,
        CONDITION_TEXT = 63, // unsure where text is sourced, not in TextLabel
        SETTING_HIT = 66
    }
}

db_data_struct! {
    pub struct ActionParts {
        // _Id: u32,
        _ref: u32,
        pub _seq: u32,
        _seconds: f64,
        _speed: f64,
        _duration: f64,
        // _activateId: u32,
        pub commandType: CommandType,
        _motionState: String,
        _motionFrame: u32,
        _blendDuration: f64,
        _isBlend: u32,
        _isEndSyncMotion: u32,
        _isIgnoreFinishCondition: u32,
        _isIdleAfterCancel: u32,
        _chargeSec: f64,
        _chargeLvSec: String,
        _bulletSpeed: f64,
        _delayTime: f64,
        _collisionHitInterval: f64,
        _isHitDelete: u32,
        _hitLabel: String,
        _hitAttrLabel: String,
        _abHitAttrLabel: String,
        _bulletNum: u32,
        _generateNum: u32,
        _generateDelay: f64,
        _signalType: u32,
        _decoId: u32,
        _actionId: u32,
        _keepActionEnd: u32,
        _keepActionId1: u32,
        _keepActionId2: u32,
        _actionType: u32,
        _motionEnd: bool,
        // _animationName: String,
        _isVisible: bool,
        _isActionClear: bool
    }
}

db_data_struct! {
    pub struct PlayerAction {
        _Id: u32,
        // _ActionName: String,
        _Range: f64,
        _CanTurnPrepare: u32,
        _IsDragonAttack: bool,
        _IsDefaultSkill: bool,
        _IsChargeSkill: bool,
        _IsHeroSkill: bool,
        _HealType: u32,
        _MaxStockBullet: u32,
        _NextAction: u32,
        _IsLoopAction: bool,
        _MaxAdditionalInput: u32,
        _IsAllyTarget: bool,
        _BurstMarkerId: u32,
        _IsLongRangeCamera: u32,
        _IgnoreLongRangeCamera: u32,
        _OverwriteVoice: String,
        _ConsumeEp: u32
    }
}

link_data_struct!(
    ActionCondition {
        link_damaged_hit_attr: _DamageLink -> PlayerActionHitAttribute,
        link_remove_condition: _RemoveConditionId -> ActionCondition
    }
);

link_data_struct!(
    PlayerActionHitAttribute {
        link_action_condition: _ActionCondition1 -> ActionCondition
    }
);

// link_data_struct!(
//     ActionParts {
//         link_hit_label: _hitLabel -> PlayerActionHitAttribute,
//         link_hit_attr_label: _hitAttrLabel -> PlayerActionHitAttribute,
//         link_ab_hit_attr_label: _abHitAttrLabel -> PlayerActionHitAttribute
//     }
// );

link_data_struct!(
    PlayerAction {
        link_next_action: _NextAction -> PlayerAction
    }
);

link_data_struct_multi!(
    PlayerAction {
        link_action_parts: _Id -> ActionParts
    }
);

macro_rules! link_hit_attr_levels {
    ($name:ty {$($func:ident : $dkey:ident -> $dname:ident),*}) => {
        #[allow(dead_code)]
        impl $name {
            $(pub fn $func(&self, conn: &Connection) -> Result<Vec<$dname>> {
                let len = self.$dkey.len();
                if len > 4 && &self.$dkey[(len-4)..(len-2)] == "LV"{
                    let skey = &self.$dkey[0..(len-2)].to_string();
                    return $dname::populate_conditionally(&conn, "PlayerActionHitAttribute._Id LIKE ? || \'%\'", &[&skey]);
                }else{
                    return $dname::populate_multiple(&conn, &self.$dkey);
                }
            })*
        }
    };
}

link_hit_attr_levels! (
    ActionParts {
        link_hit_label: _hitLabel -> PlayerActionHitAttribute,
        link_hit_attr_label: _hitAttrLabel -> PlayerActionHitAttribute,
        link_ab_hit_attr_label: _abHitAttrLabel -> PlayerActionHitAttribute
    }
);

// impl ActionParts {
//     pub fn link_hit_label(&self, conn: &Connection) -> Result<Vec<PlayerActionHitAttribute>> {
//         // let mut stmt = conn.prepare(stringify!(SELECT $pkname, $(($fname)),* FROM $name WHERE $pkname=?;))?;
//         // let rows = stmt.query_map(&[&val], |r| {
//         //     let mut x = 0;
//         //     let mut counter = || {x+=1; return x};
//         //     Ok($name {
//         //         $pkname : r.get(0)?,
//         //         $($fname: match r.get(counter()) {
//         //             Ok(d) => d,
//         //             Err(_e) => <$ftype>::default(),
//         //         }),*
//         //     })
//         // })?;
//         // let mut res = Vec::new();
//         // for q_res in rows {
//         //     res.push(q_res?);
//         // }
//         // return Ok(res);
//     }
// }
