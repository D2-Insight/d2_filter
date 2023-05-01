use d2_minify::stats::MiniStat;

use crate::enums::*;
use crate::weapons::structs::MinimizedWeapon;
use crate::*;
#[inline(always)]
fn check_stats(stat_range: &StatFilter, check_stat: &i8) -> bool {
    match stat_range {
        StatFilter::Above(stat_above) => stat_above < check_stat,
        StatFilter::Between(stat_low, stat_high) => stat_high > check_stat || stat_low < check_stat,
        StatFilter::Below(stat_below) => stat_below > check_stat,
        StatFilter::AtOrAbove(stat_above) => stat_above <= check_stat,
        StatFilter::AtOrBetween(stat_low, stat_high) => {
            stat_high >= check_stat || stat_low <= check_stat
        }
        StatFilter::AtOrBelow(stat_below) => stat_below >= check_stat,
        StatFilter::At(stat_at) => stat_at == check_stat,
        _ => true,
    }
}

#[inline(always)]
pub fn filter_stats(item: &MinimizedWeapon, stats: &Vec<(MiniStat, StatFilter)>) -> bool {
    let item_stats = &item.stats;
    for (stat, stat_range) in stats {
        if !item_stats
            .get(stat)
            .map(|stat_option| check_stats(stat_range, stat_option))
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

#[inline(always)]
pub fn filter_perks(perks: &GunPerkMap, item: &MinimizedWeapon, search: &PerkMap) -> bool {
    let hash = &item.hash;

    for (perk_hash, slot) in search {
        let perk = perks.get(hash).unwrap();
        if !perk
            .get(perk_hash)
            .map(|actual_slot| {
                slot == actual_slot
                    || slot == &PerkSlot::LeftRight
                        && matches!(actual_slot, &PerkSlot::Left | &PerkSlot::Right)
            })
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

#[inline(always)]
pub fn filter_item_type(item: &MinimizedWeapon, search: DestinyItemSubType) -> bool {
    item.weapon_type == search as u8
}

#[inline(always)]
pub fn filter_craftable(item: &MinimizedWeapon, search: bool, craftables: &BungieHashSet) -> bool {
    craftables.get(&item.hash).is_some() == search
}

#[inline(always)]
pub fn filter_adept(item: &MinimizedWeapon, search: bool, adept: &BungieHashSet) -> bool {
    adept.get(&item.hash).is_some() == search
}

#[inline(always)]
pub fn filter_ammo(item: &MinimizedWeapon, search: DestinyAmmunitionType) -> bool {
    item.ammo_type == search as u8
}
