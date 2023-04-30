use rustgie_types::destiny::{TierType, DamageType};

use crate::weapons::structs::MinimizedWeapon;

#[inline(always)]
pub fn filter_names(item: &crate::weapons::structs::MinimizedWeapon, search: &String) -> bool {
    item.name
        .to_lowercase()
        .contains(search.to_lowercase().as_str())
}

#[inline(always)]
pub fn filter_rarity(item: &MinimizedWeapon, search: TierType) -> bool {
    item.rarity == search
}

#[inline(always)]
pub fn filter_slot(item: &MinimizedWeapon, search: u32) -> bool {
    item.slot == search
}

#[inline(always)]
pub fn filter_energy(item: &MinimizedWeapon, search: DamageType) -> bool {
    item.energy == search
}
