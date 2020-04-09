extern crate rusqlite;
use rusqlite::{Connection, Result};

pub const DB_FILE: &str = "dl.sqlite";

macro_rules! db_data_struct {
    (pub struct $name:ident { $pkname:ident : $pktype:ty, $($fname:ident : $ftype:ty),* }) => {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct $name {
            $pkname : $pktype,
            $($fname : $ftype),*
        }

        #[allow(dead_code)]
        impl $name {
            // pub fn field_names() -> &'static [&'static str] {
            //     static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
            //     NAMES
            // }

            pub fn populate(conn: &Connection, pk: $pktype) -> Result<$name>{
                let mut x = 0;
                let mut counter = || {x+=1; return x};
                return conn.query_row(
                    stringify!(SELECT $pkname, $(($fname)),* FROM $name WHERE $pkname=?;),
                    &[pk],
                    |r| {
                    Ok($name {
                        $pkname : r.get(0)?,
                        $($fname: r.get(counter())?),*
                    })
                });
            }
        }
    }
}

db_data_struct! {
    pub struct PlayerActionHitAttribute {
        _Id: String, // name of this hit attr
        // _FontEffect: String,
        _HitExecType: u8, // 1: attack, 2: buff
        _TargetGroup: u8, // targets, 1: self, 2: team, 3: enemy, 4: ?, 5: dodge, 6: also team, 7: lowest hp teammate, 8: buffs(?)
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
        _RecoverySpSkillIndex: u8, // sp gain target, 0: all, 1: s1, etc
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
        _KillerState1: u8, // killer
        _KillerState2: u8,
        _KillerState3: u8,
        // 1: Poison, 2: Burn, 3: Freeze, 4: Paralysis, 5: Blind, 6: Stun, 7: Curse, 8: UNKNOWN08, 9: Bog, 10: Sleep, 11: Frostbite, 103: Def down, 198: Buffed, 201: Break
        _KillerStateDamageRate: f64, // killer modifier
        // _KillerStateRelease: u8,
        _DamageUpRateByBuffCount: f64, // more damage per buff stack
        // _SplitDamageCount: u32, // seems like enemy only stuff
        // _SplitDamageCount2: u32,
        _ArmorBreakLv: u8 // iframe, 1: roll/buff, 4: skill iframe
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
        _Type: u8, // affliction type
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
        _TargetAction: u8, // target action, 2: fs, 3: s1?, 4: s2?, 5: buff skills
        _TargetElemental: u8, // bit flags, 00001: flame, 00010: water, 00100: wind, 01000: light, 10000: shadow
        // _ConditionAbs: 0,
        _ConditionDebuff: u8, // debuff cond, 16 for bleed
        _RateHP: f64,
        _RateAttack: f64,
        _RateDefense: f64, // def down
        _RateDefenseB: f64, // zone def down
        _RateCritical: f64,
        _RateSkill: f64,
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
        _RateReraise: f64,
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
        _DisableAction: u32, // laxi shut down
        _DisableMove: u32, // tobias alt x
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

impl PlayerActionHitAttribute {
    pub fn link_action_condition(self, conn: &Connection) -> Result<ActionCondition> {
        return ActionCondition::populate(&conn, self._ActionCondition1);
    }
}
