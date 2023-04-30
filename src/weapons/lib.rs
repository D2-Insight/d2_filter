use std::{collections::HashSet, fs::File};

use rustgie_types::destiny::{DamageType, DestinyAmmunitionType, DestinyItemSubType, TierType};

use crate::{
    enums::WeaponSlot,
    inventory_items::filters::{filter_energy, filter_names, filter_rarity, filter_slot},
    BungieHashSet, GunPerkMap, PerkMap, StatVec,
};

use super::{filters::*, structs::MinimizedWeapon};

pub struct WeaponFilter {
    pub weapons: Vec<MinimizedWeapon>,
    pub adept: BungieHashSet,
    pub perks: GunPerkMap,
    pub craftable: BungieHashSet,
}

pub struct WeaponRequest {
    pub family: Option<DestinyItemSubType>,
    pub stats: Option<StatVec>,
    pub energy: Option<DamageType>,
    pub slot: Option<WeaponSlot>,
    pub adept: Option<bool>,
    pub craftable: Option<bool>,
    pub name: Option<String>,
    pub rarity: Option<TierType>,
    pub ammo: Option<DestinyAmmunitionType>,
    pub perks: Option<PerkMap>,
}

impl Default for WeaponRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl WeaponRequest {
    pub fn new() -> Self {
        WeaponRequest {
            family: None,
            stats: None,
            energy: None,
            slot: None,
            adept: None,
            craftable: None,
            name: None,
            rarity: None,
            ammo: None,
            perks: None,
        }
    }
}

impl WeaponFilter {
    //This is the slowest part, but mostly because networking + bungie, everything else is fast af
    pub fn new() -> Self {
        let file: File = File::open("./weapons.cbor").unwrap();
        let weapons: Vec<MinimizedWeapon> = serde_cbor::from_reader(file).unwrap();
        let file: File = File::open("./adept.cbor").unwrap();
        let adept: HashSet<u32> = serde_cbor::from_reader(file).unwrap();
        let file: File = File::open("./craftable.cbor").unwrap();
        let craftable: HashSet<u32> = serde_cbor::from_reader(file).unwrap();
        let file: File = File::open("./perks.cbor").unwrap();
        let perks: GunPerkMap = serde_cbor::from_reader(file).unwrap();

        WeaponFilter {
            weapons,
            adept,
            perks,
            craftable,
        }
    }
}

impl Default for WeaponFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl WeaponFilter {
    #[inline(always)]
    pub fn check_weapon(&self, item: &MinimizedWeapon, search: &WeaponRequest) -> bool {
        if let Some(query) = search.ammo {
            if !filter_ammo(item, query) {
                return false;
            }
        }
        if let Some(query) = search.energy {
            if !filter_energy(item, query) {
                return false;
            }
        }
        if let Some(query) = search.family {
            if !filter_item_type(item, query) {
                return false;
            }
        }
        if let Some(query) = search.slot {
            if !filter_slot(item, query.into()) {
                return false;
            }
        }
        if let Some(query) = search.rarity {
            if !filter_rarity(item, query) {
                return false;
            }
        }
        if let Some(query) = search.craftable {
            if !filter_craftable(item, query, &self.craftable) {
                return false;
            }
        }
        if let Some(query) = search.adept {
            if !filter_adept(item, query, &self.adept) {
                return false;
            }
        }
        if let Some(query) = &search.perks {
            if !filter_perks(&self.perks, item, query) {
                return false;
            }
        }
        if let Some(query) = &search.stats {
            if !filter_stats(item, query) {
                return false;
            }
        }
        if let Some(query) = &search.name {
            if !filter_names(item, query) {
                return false;
            }
        }
        true
    }
}

impl WeaponFilter {
    #[inline(always)]
    pub fn filter_for(&self, search: WeaponRequest) -> Vec<MinimizedWeapon> {
        let mut result: Vec<MinimizedWeapon> = Vec::new();
        for item in &self.weapons {
            if self.check_weapon(item, &search) {
                result.push(item.to_owned());
            }
        }
        result.shrink_to_fit();
        result
    }
}
