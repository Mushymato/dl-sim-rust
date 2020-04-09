extern crate rusqlite;
use rusqlite::{Connection, Result};

pub const DB_FILE: &str = "dl.sqlite";

macro_rules! db_data_struct {
    (pub struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            // pub fn field_names() -> &'static [&'static str] {
            //     static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
            //     NAMES
            // }

            pub fn populate(conn: Connection, pk: String) -> Result<$name>{
                let mut x = 0;
                let mut counter = || {x+=1; return x-1};
                return conn.query_row(
                    stringify!(SELECT $(($fname)),* FROM $name WHERE _Id=?;),
                    &[pk],
                    |r| {
                    Ok($name {
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
