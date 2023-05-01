
use crate::weapons::structs::MinimizedWeapon;
use d2_minify::watermark::MiniWatermark;

#[inline(always)]
pub fn filter_names(item: &crate::weapons::structs::MinimizedWeapon, search: &String) -> bool {
    item.name
        .to_lowercase()
        .contains(search.to_lowercase().as_str())
}

#[inline(always)]
pub fn filter_rarity(item: &MinimizedWeapon, search: u32) -> bool {
    item.rarity == search as u8
}

#[inline(always)]
pub fn filter_slot(item: &MinimizedWeapon, search: u32) -> bool {
    item.slot == search as u8
}

#[inline(always)]
pub fn filter_energy(item: &MinimizedWeapon, search: u32) -> bool {
    item.energy == search as u8
}

#[inline(always)]
pub fn filter_season(item: &MinimizedWeapon, search: MiniWatermark) -> bool {
    item.season == search
}
