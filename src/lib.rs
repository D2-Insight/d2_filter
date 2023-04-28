use num_enum::IntoPrimitive;
use rustgie_types::destiny::{DamageType, DestinyAmmunitionType, DestinyItemSubType, TierType};
use serde_repr::Deserialize_repr;
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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum PerkSlot {
    Barrel = 0,
    Magazine = 1,
    Left = 2,
    Right = 3,
    Origin = 4,
    Unknown = 5,
    LeftRight,
}
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

//thanks calc api lol
#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum StatHashes {
    Accuracy = 1591432999,
    AimAssist = 1345609583,
    Airborne = 2714457168,
    AmmoCapacity = 925767036,
    Attack = 1480404414,
    BlastRadius = 3614673599,
    ChargeRate = 3022301683,
    ChargeTime = 2961396640,
    DrawTime = 447667954,
    GuardEfficiency = 2762071195,
    GuardEndurance = 3736848092,
    GuardResistance = 209426660,
    Handling = 943549884,
    Impact = 4043523819,
    InventorySize = 1931675084,
    Magazine = 3871231066,
    Range = 1240592695,
    RecoilDir = 2715839340,
    Recovery = 1943323491,
    Reload = 4188031367,
    Rpm = 4284893193,
    ShieldDuration = 1842278586,
    SwingSpeed = 2837207746,
    Velocity = 2523465841,
    Zoom = 3555269338,
    Unkown = 0,
}

#[derive(Clone, Copy)]
pub enum StatFilter {
    Above(i32),
    Between(i32, i32),
    Below(i32),
    AtOrAbove(i32),
    AtOrBelow(i32),
    AtOrBetween(i32, i32),
    At(i32),
    Minimum,
    Maximum,
}

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u32)]
pub enum WeaponSlot {
    Top = 1498876634,
    Middle = 2465295065,
    Bottom = 953998645,
}

#[repr(u8)]
enum MiniWeaponSlot {
    Top = 1,
    Middle = 2,
    Bottom = 3,
}

impl Into<WeaponSlot> for MiniWeaponSlot {
    fn into(self) -> WeaponSlot {
        match self {
            MiniWeaponSlot::Bottom => WeaponSlot::Bottom,
            MiniWeaponSlot::Middle => WeaponSlot::Middle,
            MiniWeaponSlot::Top => WeaponSlot::Top,
        }
    }
}

impl Into<MiniWeaponSlot> for WeaponSlot {
    fn into(self) -> MiniWeaponSlot {
        match self {
            WeaponSlot::Bottom => MiniWeaponSlot::Bottom,
            WeaponSlot::Middle => MiniWeaponSlot::Middle,
            WeaponSlot::Top => MiniWeaponSlot::Top,
        }
    }
}

//Planning on reducing memory usage by preprocessing manifest into this struct.
//craftable, adept, and sunset should just be in a seperate hashset to reduce space.
//I could reduce a lot of these to u8 but keeping it near bungie spec
#[derive(Clone, serde::Deserialize)]
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
fn filter_stats(item: &MinimizedWeapon, stats: &Vec<(BungieHash, StatFilter)>) -> bool {
    let item_stats = &item.stats;
    for (stat, stat_range) in stats {
        if let Some(stat_option) = item_stats.get(stat) {
            if !check_stats(stat_range, stat_option) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[inline(always)]
fn filter_names(item: &MinimizedWeapon, search: &str) -> bool {
    item.name
        .to_lowercase()
        .contains(search.to_lowercase().as_str())
}

#[inline(always)]
fn filter_perks(perks: &GunPerkMap, item: &MinimizedWeapon, search: &PerkMap) -> bool {
    let hash = &item.hash;

    for (perk_hash, slot) in search {
        if let Some(actual_slot) = perks.get(hash).unwrap().get(perk_hash) {
            if !(slot == actual_slot
                && slot == &PerkSlot::LeftRight
                && matches!(actual_slot, &PerkSlot::Left | &PerkSlot::Right))
            {
                return false;
            }
        }
    }
    true
}

#[inline(always)]
fn filter_weapon_type(item: &MinimizedWeapon, search: DestinyItemSubType) -> bool {
    item.weapon_type == search
}

#[inline(always)]
fn filter_craftable(item: &MinimizedWeapon, search: bool, craftables: &BungieHashSet) -> bool {
    craftables.get(&item.hash).is_some() == search
}

#[inline(always)]
fn filter_energy(item: &MinimizedWeapon, search: DamageType) -> bool {
    item.energy == search
}

#[inline(always)]
fn filter_rarity(item: &MinimizedWeapon, search: TierType) -> bool {
    item.rarity == search
}

#[inline(always)]
fn filter_adept(item: &MinimizedWeapon, search: bool, adept: &BungieHashSet) -> bool {
    adept.get(&item.hash).is_some() == search
}

#[inline(always)]
fn filter_slot(item: &MinimizedWeapon, search: u32) -> bool {
    item.slot == search
}

#[inline(always)]
fn filter_ammo(item: &MinimizedWeapon, search: DestinyAmmunitionType) -> bool {
    item.ammo_type == search
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
            if !filter_names(item, query.as_str()) {
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
