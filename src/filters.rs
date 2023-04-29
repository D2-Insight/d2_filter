use crate::enums::*;
use crate::*;
#[inline(always)]
fn check_stats(stat_range: &StatFilter, check_stat: &i32) -> bool {
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
pub fn filter_stats(item: &MinimizedWeapon, stats: &Vec<(BungieHash, StatFilter)>) -> bool {
    let item_stats = &item.stats;
    for (stat, stat_range) in stats {
        if !item_stats
            .get(stat)
            .and_then(|stat_option| Option::Some(check_stats(stat_range, stat_option)))
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

#[inline(always)]
pub fn filter_names(item: &MinimizedWeapon, search: &String) -> bool {
    item.name
        .to_lowercase()
        .contains(search.to_lowercase().as_str())
}

#[inline(always)]
pub fn filter_perks(perks: &GunPerkMap, item: &MinimizedWeapon, search: &PerkMap) -> bool {
    let hash = &item.hash;

    for (perk_hash, slot) in search {
        let perk = perks.get(hash).unwrap();
        if !perk
            .get(perk_hash)
            .and_then(|actual_slot| {
                Option::Some(
                    slot == actual_slot
                        || slot == &PerkSlot::LeftRight
                            && matches!(actual_slot, &PerkSlot::Left | &PerkSlot::Right),
                )
            })
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

#[inline(always)]
pub fn filter_weapon_type(item: &MinimizedWeapon, search: DestinyItemSubType) -> bool {
    item.weapon_type == search
}

#[inline(always)]
pub fn filter_craftable(item: &MinimizedWeapon, search: bool, craftables: &BungieHashSet) -> bool {
    craftables.get(&item.hash).is_some() == search
}

#[inline(always)]
pub fn filter_energy(item: &MinimizedWeapon, search: DamageType) -> bool {
    item.energy == search
}

#[inline(always)]
pub fn filter_rarity(item: &MinimizedWeapon, search: TierType) -> bool {
    item.rarity == search
}

#[inline(always)]
pub fn filter_adept(item: &MinimizedWeapon, search: bool, adept: &BungieHashSet) -> bool {
    adept.get(&item.hash).is_some() == search
}

#[inline(always)]
pub fn filter_slot(item: &MinimizedWeapon, search: u32) -> bool {
    item.slot == search
}

#[inline(always)]
pub fn filter_ammo(item: &MinimizedWeapon, search: DestinyAmmunitionType) -> bool {
    item.ammo_type == search
}
