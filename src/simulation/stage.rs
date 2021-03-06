use rusqlite::Connection;

use crate::entities::{AbilityData, AmuletData, CharaData, DragonData, WeaponData};
use crate::push_if_exists;

use crate::simulation::AbilityHandler;

#[derive(Debug)]
pub struct Player {
    chara: CharaData,
    dragon: DragonData,
    weapon: WeaponData,
    amulets: [AmuletData; 2],
    pub abilities: Vec<AbilityHandler>,
    hp: u32,
    atk: u32,
}

impl Player {
    pub fn new(
        conn: &Connection,
        chara_id: u32,
        dragon_id: u32,
        weapon_id: u32,
        amulet_ids: [u32; 2],
    ) -> Player {
        let chara = CharaData::populate(&conn, &chara_id).unwrap();
        let dragon = DragonData::populate(&conn, &dragon_id).unwrap();
        let weapon = WeaponData::populate(&conn, &weapon_id).unwrap();
        let amulets = [
            AmuletData::populate(&conn, &amulet_ids[0]).unwrap(),
            AmuletData::populate(&conn, &amulet_ids[1]).unwrap(),
        ];

        let (hp, atk) = Player::calculate_total_stats(&chara, &dragon, &weapon, &amulets);
        let abilities: Vec<AbilityHandler> =
            Player::link_all_abilities(&conn, &chara, &dragon, &weapon, &amulets);
        return Player {
            chara: chara,
            dragon: dragon,
            weapon: weapon,
            amulets: amulets,
            abilities: abilities,
            hp: hp,
            atk: atk,
        };
    }

    fn link_all_abilities(
        conn: &Connection,
        chara: &CharaData,
        dragon: &DragonData,
        weapon: &WeaponData,
        amulets: &[AmuletData; 2],
    ) -> Vec<AbilityHandler> {
        let mut abilities: Vec<AbilityData> = chara.link_abilities(&conn);
        push_if_exists!(abilities, dragon.link_ability_1(&conn));
        push_if_exists!(abilities, dragon.link_ability_2(&conn));
        push_if_exists!(abilities, weapon.link_ability_1(&conn));
        push_if_exists!(abilities, weapon.link_ability_2(&conn));
        for am in amulets {
            push_if_exists!(abilities, am.link_ability_1(&conn));
            push_if_exists!(abilities, am.link_ability_2(&conn));
        }
        let mut ref_ab = AbilityData::link_all_referenced_ability(&conn, &abilities);
        while ref_ab.len() > 0 {
            let next_ref_ab = AbilityData::link_all_referenced_ability(&conn, &ref_ab);
            abilities.extend(ref_ab);
            ref_ab = next_ref_ab;
        }
        return abilities
            .into_iter()
            .map(|ab| AbilityHandler::new(ab))
            .collect();
    }

    /* SELECT tl._Text as _Name, fpd1._AssetGroup, fpd1._Level, fpd1._EffectId, fpd1._EffArgs1, fpd1._EffArgs2, fpd1._EffArgs3
    FROM FortPlantDetail fpd1
    INNER JOIN
    (SELECT _AssetGroup, MAX(_Level) AS _Level FROM FortPlantDetail GROUP BY _AssetGroup) as fpd2
    ON fpd1._AssetGroup=fpd2._AssetGroup AND fpd1._Level=fpd2._Level
    INNER JOIN FortPlantData fpd
    ON fpd1._AssetGroup=fpd._Id
    INNER JOIN TextLabel tl
    ON tl._Id=fpd._Name
    WHERE fpd1._EffectId > 0; */
    fn calculate_total_stats(
        chara: &CharaData,
        dragon: &DragonData,
        weapon: &WeaponData,
        amulets: &[AmuletData; 2],
    ) -> (u32, u32) {
        let mut hp = amulets[0]._MaxHp + 100 + amulets[1]._MaxHp + 100 + weapon._MaxHp;
        let mut atk = amulets[0]._MaxAtk + 100 + amulets[1]._MaxAtk + 100 + weapon._MaxAtk;
        if chara._ElementalType == weapon._ElementalType {
            hp += f32::ceil(weapon._MaxHp as f32 * 0.5) as u32;
            atk += f32::ceil(weapon._MaxAtk as f32 * 0.5) as u32;
        }
        // 11.5% hp 11.5% atk
        let dragon_hp = f32::ceil(dragon._MaxHp as f32 * 1.115) as u32;
        let dragon_atk = f32::ceil(dragon._MaxAtk as f32 * 1.115) as u32;
        hp += dragon_hp;
        atk += dragon_atk;
        if chara._ElementalType == dragon._ElementalType {
            hp += f32::ceil(dragon_hp as f32 * 0.5) as u32;
            atk += f32::ceil(dragon_atk as f32 * 0.5) as u32;
        }
        let mut chara_hp_mult: f32 = 1.0 + 0.09 * 2.0 + 0.04 + 0.30;
        let mut chara_atk_mult: f32 = 1.0 + 0.09 * 2.0 + 0.04 + 0.30;
        match chara._ElementalType {
            1 => {
                chara_hp_mult += 0.095 + 0.16;
                chara_atk_mult += 0.085 + 0.16;
            }
            2 => {
                chara_hp_mult += 0.095 + 0.085;
                chara_atk_mult += 0.085 + 0.070;
            }
            3 => {
                chara_hp_mult += 0.095;
                chara_atk_mult += 0.085;
            }
            4 => {
                chara_hp_mult += 0.095 + 0.085;
                chara_atk_mult += 0.085 + 0.070;
            }
            5 => {
                chara_hp_mult += 0.085 + 0.16;
                chara_atk_mult += 0.070 + 0.16;
            }
            _ => (),
        }
        match chara._WeaponType {
            // blade dagger bow wand
            2 | 3 | 6 | 7 => {
                chara_hp_mult += 0.05;
                chara_atk_mult += 0.05;
            }
            _ => (),
        }
        hp += f32::ceil((chara.max_hp() + 100) as f32 * chara_hp_mult) as u32;
        atk += f32::ceil((chara.max_atk() + 100) as f32 * chara_atk_mult) as u32;
        return (hp, atk);
    }
}
