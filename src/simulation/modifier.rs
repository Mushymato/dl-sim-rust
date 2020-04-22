use crate::entities::{
    AbilityData, ActionCondition, ActionParts, PlayerAction, PlayerActionHitAttribute, SkillData,
};

macro_rules! handler_struct {
    ($namevis:vis struct $name:ident { $hname:ident : $htype:ty, $($fname:ident : $ftype:ty),* }) => {
        #[derive(Debug, Default)]
        $namevis struct $name {
            $namevis $hname : $htype,
            $($namevis $fname : $ftype),*
        }

        impl $name {
            pub fn new(item: $htype) -> Self {
                $name {
                    $hname: item,
                    $($fname : <$ftype>::default()),*,
                }
            }
        }
    }
}

handler_struct! {
    pub struct ActionHandler {
        data: PlayerAction,
        status: bool,
        action_parts: Vec<ActionParts>,
        hit_attributes: Vec<PlayerActionHitAttribute>,
        action_conditions: Vec<ActionCondition>
    }
}

handler_struct! {
    pub struct AbilityHandler {
        data: AbilityData,
        status: bool,
        passive: bool, // whether we need to check this again during run
        hit_attributes: Vec<PlayerActionHitAttribute>,
        action_conditions: Vec<ActionCondition>
    }
}

handler_struct! {
    pub struct SkillHandler {
        data: SkillData,
        usable: bool,
        action: Vec<ActionHandler>,
        abilities: Vec<AbilityHandler>
    }
}
