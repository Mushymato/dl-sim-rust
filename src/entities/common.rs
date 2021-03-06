use std::collections::HashMap;

use crate::entities::action::PlayerAction;
use crate::entities::label::TextLabel;

extern crate rusqlite;
use rusqlite::{Connection, Result, NO_PARAMS};

db_data_struct! {
    pub struct WeaponType {
        _Id: u8,
        _Label: String,
        // _DefaultWeapon: u32,
        // _CoRecoverSp: f64,
        // _SearchRange: f64,
        // _SearchAngle: f64,
        _CriticalProbability: u32,
        // _DamageCoef: f64,
        // _MinDamageCoef: f64,
        // _AdditionStartDistance: f64,
        // _AdditionEndDistance: f64,
        // _DecayStartDistance: f64,
        // _DecayEndDistance: f64,
        _AttackRangeType: u32,
        _DefaultSkill01: u32,
        _DefaultSkill02: u32,
        _DefaultSkill03: u32,
        _DefaultSkill04: u32,
        _DefaultSkill05: u32,
        _DefaultSkill05Ex: u32, // blade lul
        _BurstChargePhase1: u32,
        _BurstChargePhase2: u32,
        _BurstPhase1: u32,
        // _BurstPhase2: u32,
        _ChargeCancel: u32,
        _ChargeMarker: u32,
        // _ChargeSuperArmorTime: f64,
        // _ChargeSuperArmorLv: u32,
        // _BurstCameraFollowSpeed: f64,
        _DashSkill: u32,
        _AiBurstProbabilityOD: f64,
        _AiBurstProbabilityBarrier: f64,
        _AiBurstProbabilityEnhanced: f64
    }
}

link_data_struct! {
    WeaponType {
        link_dash_attack_action: _DashSkill -> PlayerAction
    }
}

impl WeaponType {
    pub fn link_burst_attack_actions(&self, conn: &Connection) -> Result<Vec<PlayerAction>> {
        return PlayerAction::populate_conditionally(
            &conn,
            "PlayerAction._Id IN (?,?,?,?)",
            &[
                &self._BurstChargePhase1,
                &self._BurstChargePhase2,
                &self._ChargeMarker,
                &self._ChargeCancel,
            ],
        );
    }
    pub fn link_combo_chain_actions(&self, conn: &Connection) -> Result<Vec<PlayerAction>> {
        return PlayerAction::populate_conditionally(
            &conn,
            "PlayerAction._Id IN (?,?,?,?,?,?)",
            &[
                &self._DefaultSkill01,
                &self._DefaultSkill02,
                &self._DefaultSkill03,
                &self._DefaultSkill04,
                &self._DefaultSkill05,
                &self._DefaultSkill05Ex,
            ],
        );
    }
}
// ($name:ty {$($func:ident : $dkey:ident, $cmaxkey:ident -> PlayerAction),*}) => {
//     #[allow(dead_code)]
//     impl $name {
//         $(pub fn $func(&self, conn: &Connection) -> Result<Vec<PlayerAction>> {
//             return PlayerAction::populate_conditionally(&conn, "PlayerAction._Id >= ? AND PlayerAction._Id < ?", &[&self.$dkey, &(self.$dkey+self.$cmaxkey)]);
//         })*
//     }
// };

db_data_struct! {
    pub struct AbnormalStatusType {
        _Id: u32,
        _AbnormalName: String,
        // _IsViewDetail: bool,
        // _SortId: u32,
        _Group: u32,
        // _Priority: u32,
        _ResistGain: u32
        // _DisplayPriority: u32
    }
}

link_label! {
    AbnormalStatusType {
        link_name: _AbnormalName -> TextLabel
    }
}

db_data_struct! {
    pub struct ElementalType {
        _Id: u32,
        _Label: String,
        _DamageRatioNoElementP2E: f64,
        _DamageRatioFireP2E: f64,
        _DamageRatioWaterP2E: f64,
        _DamageRatioWindP2E: f64,
        _DamageRatioLightP2E: f64,
        _DamageRatioDarkP2E: f64,
        _DamageRatioNoElementE2P: f64,
        _DamageRatioFireE2P: f64,
        _DamageRatioWaterE2P: f64,
        _DamageRatioWindE2P: f64,
        _DamageRatioLightE2P: f64,
        _DamageRatioDarkE2P: f64
    }
}
