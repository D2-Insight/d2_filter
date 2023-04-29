pub mod enums;
pub mod filters;

use crate::filters::*;

use enums::*;
use rustgie_types::destiny::{DamageType, DestinyAmmunitionType, DestinyItemSubType, TierType};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;

pub type BungieHash = u32;
pub type WeaponHash = u32;
pub type PerkHash = u32;
type StatVec = Vec<(BungieHash, StatFilter)>;
type BungieHashSet = HashSet<BungieHash>;
pub type PerkMap = HashMap<WeaponHash, PerkSlot>;
/// K: PerkHash V: Guns that use it
type GunPerkMap = HashMap<PerkHash, PerkMap>;

pub struct Filter {
    weapons: Vec<MinimizedWeapon>,
    adept: BungieHashSet,
    perks: GunPerkMap,
    craftable: BungieHashSet,
}

pub struct FilterRequest {
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

impl Default for FilterRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl FilterRequest {
    pub fn new() -> Self {
        FilterRequest {
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

//Planning on reducing memory usage by preprocessing manifest into this struct.
//craftable, adept, and sunset should just be in a seperate hashset to reduce space.
//I could reduce a lot of these to u8 but keeping it near bungie spec
#[derive(Clone, Deserialize)]
pub struct MinimizedWeapon {
    name: String,
    hash: u32,
    slot: u32,
    energy: DamageType,
    rarity: TierType,
    ammo_type: DestinyAmmunitionType,
    weapon_type: DestinyItemSubType,
    stats: HashMap<BungieHash, i32>,
}

impl Filter {
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

        Filter {
            weapons,
            adept,
            perks,
            craftable,
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter {
    #[inline(always)]
    pub fn check_weapon(&self, item: &MinimizedWeapon, search: &FilterRequest) -> bool {
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
            if !filter_weapon_type(item, query) {
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

impl Filter {
    #[inline(always)]
    pub fn filter_for(&self, search: FilterRequest) -> Vec<MinimizedWeapon> {
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

#[cfg(test)]
mod tests {

    use rustgie_types::destiny::*;

    use crate::{BungieHash, FilterRequest, StatFilter, StatHashes};

    #[test]
    fn test() {
        let weapon_filter = crate::Filter::new();
        let mut filter_params = FilterRequest::new();
        filter_params.adept = Some(true);
        filter_params.family = Some(DestinyItemSubType::SubmachineGun);
        filter_params.slot = Some(crate::WeaponSlot::Top);
        filter_params.energy = Some(DamageType::Strand);
        let mut perks: std::collections::HashMap<u32, crate::PerkSlot> =
            std::collections::HashMap::new();
        perks.insert(365154968, crate::PerkSlot::LeftRight);
        filter_params.perks = Some(perks);
        let start = std::time::Instant::now();
        let result = weapon_filter.filter_for(filter_params);
        let duration = start.elapsed();
        println!("{}", duration.as_millis());
        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_perks() {
        let weapon_filter = crate::Filter::new();
        let mut filter_params = FilterRequest::new();
        // /filter_params.perks = Some(365154968);
        //let mut perks: PerkMap = std::collections::HashMap::new();
        //perks.insert(3619207468, PerkSlot::LeftRight);
        //filter_params.perks = Some(perks);
        let mut stats: Vec<(BungieHash, StatFilter)> = Vec::new();
        //filter_params.family = Some(DestinyItemSubType::RocketLauncher);
        stats.push((StatHashes::Velocity.into(), StatFilter::Below(35)));
        //filter_params.ammo = Some(DestinyAmmunitionType::Heavy);
        filter_params.stats = Some(stats);
        let start: std::time::Instant = std::time::Instant::now();
        //let result = weapon_filter.filter_for(filter_params).unwrap();
        let result = weapon_filter.filter_for(filter_params);
        let duration = start.elapsed();
        //println!("{:?}", result);
        println!("{} Micro Seconds", duration.as_micros());
        println!("{} Items", result.len());

        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        //assert_eq!(result.get(&3193598749).is_some(), true);
    }

    #[test]
    fn test_rose() {
        let weapon_filter = crate::Filter::new();
        let start = std::time::Instant::now();

        let test = weapon_filter.perks.get(&854379020).unwrap();
        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        let duration = start.elapsed();

        println!("{:?}", test);
        println!("{}", duration.as_nanos());
    }
}
