use num_enum::{FromPrimitive, IntoPrimitive};
use rustgie_types::{
    api_response_::BungieApiResponse,
    destiny::{
        config::DestinyManifest,
        definitions::{sockets::DestinyPlugSetDefinition, DestinyInventoryItemDefinition},
        DamageType, DestinyAmmunitionType, DestinyItemSubType, DestinyItemType, TierType,
    },
};
use std::collections::{HashMap, HashSet};

type WeaponMap = HashMap<u32, DestinyInventoryItemDefinition>;
pub type WeaponArray = Vec<DestinyInventoryItemDefinition>;
type BungieHash = u32;
type WeaponHash = u32;
type BungieHashSet = HashSet<BungieHash>;
type PlugMap = HashMap<u32, DestinyPlugSetDefinition>;
type PerkMap = HashMap<WeaponHash, PerkSlot>;
/// K: GunHash V: Guns that use it
type GunPerkMap = HashMap<WeaponHash, PerkMap>;
#[derive(FromPrimitive, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum PerkSlot {
    Barrel = 0,
    Magazine = 1,
    Left = 2,
    Right = 3,
    Origin = 4,
    #[num_enum(default)]
    Unknown = 5,
    LeftRight,
}
pub struct Filter {
    weapons: WeaponArray,
    adept: BungieHashSet,
    perks: GunPerkMap,
}

#[allow(dead_code)]
pub struct FilterRequest {
    pub family: Option<DestinyItemSubType>,
    pub stats: Option<HashMap<BungieHash, StatFilter>>, //probably need to change this to a vec
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

#[derive(Clone)]
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

#[derive(IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum WeaponSlot {
    Top = 1498876634,
    Middle = 2465295065,
    Bottom = 953998645,
}

impl Filter {
    //This is the slowest part, but mostly because networking + bungie, everything else is fast af
    pub fn new() -> Self {
        let test: BungieApiResponse<DestinyManifest> =
            reqwest::blocking::get("https://www.bungie.net/Platform/Destiny2/Manifest/")
                .unwrap()
                .json()
                .unwrap();
        let manifest_response = test.response.unwrap();
        let weapon_path = manifest_response
            .json_world_component_content_paths
            .as_ref()
            .unwrap()
            .get("en")
            .unwrap()
            .get("DestinyInventoryItemDefinition")
            .unwrap();

        let perk_path = manifest_response
            .json_world_component_content_paths
            .as_ref()
            .unwrap()
            .get("en")
            .unwrap()
            .get("DestinyPlugSetDefinition")
            .unwrap();

        let inventory_items: WeaponMap =
            reqwest::blocking::get(format!("https://www.bungie.net{weapon_path}"))
                .unwrap()
                .json()
                .unwrap();

        let plug_sets: PlugMap =
            reqwest::blocking::get(format!("https://www.bungie.net{perk_path}"))
                .unwrap()
                .json()
                .unwrap();

        let weapons = preprocess_manifest(DestinyItemType::Weapon, inventory_items);
        let adept: BungieHashSet = reqwest::blocking::get("https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/adept-weapon-hashes.json").unwrap().json().unwrap();
        let mut perks: GunPerkMap = HashMap::new();
        perks.reserve(weapons.len() - perks.capacity());
        for item in &weapons {
            let hash = item.hash;
            let mut count: u8 = 0;
            let mut cat_index: Vec<i32> = Vec::new();
            let sockets = item.sockets.as_ref().unwrap();
            for index in sockets.socket_categories.as_ref().unwrap() {
                if index.socket_category_hash == 4241085061
                /*Weapon Perks*/
                {
                    cat_index = Vec::from(index.socket_indexes.as_deref().unwrap());
                    break;
                }
            }
            let socket_entries = Vec::from(sockets.socket_entries.as_deref().unwrap());
            let mut perk_map: PerkMap = HashMap::new();
            for socket_index in &cat_index {
                let socket = socket_entries.get(*socket_index as usize).unwrap();
                if socket.single_initial_item_hash == 2302094943
                /*Kill tracker >:(*/
                {
                    continue;
                }

                //STATIC PERK
                perk_map.insert(socket.single_initial_item_hash, PerkSlot::from(count));
                //STATIC PERKS SOME TIMES?
                if let Some(hash) = &socket.reusable_plug_items {
                    for static_perk in hash {
                        perk_map.insert(static_perk.plug_item_hash, PerkSlot::from(count));
                    }
                }

                //STATIC PERK TOO??
                if let Some(hash) = socket.reusable_plug_set_hash {
                    for perk in plug_sets
                        .get(&hash)
                        .unwrap()
                        .reusable_plug_items
                        .as_ref()
                        .unwrap()
                    {
                        perk_map.insert(perk.plug_item_hash, PerkSlot::from(count));
                    }
                }

                //RANDOM PERKS
                if let Some(hash) = socket.randomized_plug_set_hash {
                    for perk in plug_sets
                        .get(&hash)
                        .unwrap()
                        .reusable_plug_items
                        .as_ref()
                        .unwrap()
                    {
                        perk_map.insert(perk.plug_item_hash, PerkSlot::from(count));
                    }
                }
                count += 1;
            }
            perks.insert(hash, perk_map);
        }
        perks.shrink_to_fit();

        Filter {
            weapons,
            adept,
            perks,
        }
    }
}

///Removes all unneeded manifest BS
fn preprocess_manifest(item_type: DestinyItemType, map: WeaponMap) -> WeaponArray {
    let mut buffer: WeaponArray = Vec::new();
    for (_, item) in map {
        if item.item_type == item_type {
            buffer.push(item);
        }
    }
    buffer.shrink_to_fit();
    buffer
}

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

fn filter_stats(
    item: &DestinyInventoryItemDefinition,
    stats: &std::collections::HashMap<u32, StatFilter>,
) -> bool {
    let item_stats = item.stats.as_ref().unwrap().stats.as_ref().unwrap();
    for (stat, stat_range) in stats {
        if let Some(stat_option) = item_stats.get(stat) {
            if !check_stats(stat_range, &stat_option.value) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn filter_names(item: &DestinyInventoryItemDefinition, search: &str) -> bool {
    item.display_properties
        .as_ref()
        .unwrap()
        .name
        .as_ref()
        .unwrap()
        .to_lowercase()
        .contains(search.to_lowercase().as_str())
}

fn filter_perks(
    perks: &GunPerkMap,
    item: &DestinyInventoryItemDefinition,
    search: &PerkMap,
) -> bool {
    let hash = &item.hash;

    for (perk_hash, slot) in search {
        if let Some(actual_slot) = perks.get(hash).unwrap().get(perk_hash) {
            if slot != actual_slot
                && !(slot == &PerkSlot::LeftRight
                    && matches!(actual_slot, &PerkSlot::Left | &PerkSlot::Right))
            {
                return false;
            }
        }
    }
    true
}

fn filter_weapon_type(item: &DestinyInventoryItemDefinition, search: DestinyItemSubType) -> bool {
    item.item_sub_type == search
}

fn filter_craftable(item: &DestinyInventoryItemDefinition, search: bool) -> bool {
    item.inventory.as_ref().unwrap().recipe_item_hash.is_some() == search
}

fn filter_energy(item: &DestinyInventoryItemDefinition, search: DamageType) -> bool {
    item.default_damage_type == search
}

fn filter_rarity(item: &DestinyInventoryItemDefinition, search: TierType) -> bool {
    item.inventory.as_ref().unwrap().tier_type == search
}

fn filter_adept(
    item: &DestinyInventoryItemDefinition,
    search: bool,
    adept: &BungieHashSet,
) -> bool {
    adept.get(&item.hash).is_some() == search
}

fn filter_slot(item: &DestinyInventoryItemDefinition, search: u32) -> bool {
    item.equipping_block
        .as_ref()
        .unwrap()
        .equipment_slot_type_hash
        == search
}

fn filter_ammo(item: &DestinyInventoryItemDefinition, search: DestinyAmmunitionType) -> bool {
    item.equipping_block.as_ref().unwrap().ammo_type == search
}

pub fn check_weapon(
    item: &DestinyInventoryItemDefinition,
    search: &FilterRequest,
    adept: &BungieHashSet,
    perks: &GunPerkMap,
) -> bool {
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
        if !filter_craftable(item, query) {
            return false;
        }
    }
    if let Some(query) = search.adept {
        if !filter_adept(item, query, adept) {
            return false;
        }
    }
    if let Some(query) = &search.perks {
        if !filter_perks(perks, item, query) {
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

//SUPER fast.
//less than 1ms in debug???
impl Filter {
    pub fn filter_for_new(&self, search: FilterRequest) -> WeaponArray {
        let mut result: WeaponArray = Vec::new();
        for item in &self.weapons {
            if check_weapon(item, &search, &self.adept, &self.perks) {
                result.push(item.to_owned());
            }
        }
        result.shrink_to_fit();
        result
    }
}

//18 miliseconds in debug
impl Filter {
    pub fn filter_for(
        &self,
        search: FilterRequest,
    ) -> Result<WeaponArray, Box<dyn std::error::Error>> {
        let mut buffer = self.weapons.clone();
        if let Some(query) = search.ammo {
            //buffer = filter_ammo(buffer, query).await;
            buffer.retain(|item| filter_ammo(item, query));
        }
        if let Some(query) = search.family {
            //buffer = filter_weapon_type(buffer, query).await?;
            buffer.retain(|item| filter_weapon_type(item, query))
        }
        if let Some(query) = search.perks {
            //buffer = filter_perks(self.perks.clone(), buffer, query).await?;
            buffer.retain(|item| filter_perks(&self.perks, item, &query));
        }
        if let Some(query) = search.slot {
            //buffer = filter_slot(buffer, query).await?;
            let query = query as u32;
            buffer.retain(|item| filter_slot(item, query))
        }
        if let Some(query) = search.energy {
            //buffer = filter_energy(buffer, query).await?;
            buffer.retain(|item| filter_energy(item, query));
        }
        if let Some(query) = search.rarity {
            //buffer = filter_rarity(buffer, query).await?;
            buffer.retain(|item| filter_rarity(item, query));
        }
        if let Some(query) = search.adept {
            //buffer = filter_adept(buffer, query, self.adept.clone()).await?;
            buffer.retain(|item| filter_adept(item, query, &self.adept));
        }
        if let Some(query) = search.craftable {
            //buffer = filter_craftable(buffer, query).await?;
            buffer.retain(|item| filter_craftable(item, query));
        }
        if let Some(query) = search.stats {
            //buffer = filter_stats(buffer, query).await?;
            buffer.retain(|item| filter_stats(item, &query));
        }
        if let Some(query) = search.name {
            //buffer = filter_names(buffer, query).await?;
            buffer.retain(|item| filter_names(item, query.as_str()));
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {

    use rustgie_types::destiny::*;

    use crate::{BungieHash, FilterRequest, StatFilter, StatHashes};
    use std::collections::HashMap;

    #[test]
    fn test() {
        let weapon_filter = crate::Filter::new();
        let mut filter_params = FilterRequest::new();
        filter_params.adept = Some(true);
        filter_params.family = Some(DestinyItemSubType::SubmachineGun);
        filter_params.slot = Some(crate::WeaponSlot::Top);
        filter_params.energy = Some(DamageType::Strand);
        let start = std::time::Instant::now();
        let result = weapon_filter.filter_for(filter_params).unwrap();
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
        let mut stats: HashMap<BungieHash, StatFilter> = HashMap::new();
        //filter_params.family = Some(DestinyItemSubType::RocketLauncher);
        stats.insert(StatHashes::Velocity.into(), StatFilter::Below(35));
        //filter_params.ammo = Some(DestinyAmmunitionType::Heavy);
        filter_params.stats = Some(stats);
        let start: std::time::Instant = std::time::Instant::now();
        //let result = weapon_filter.filter_for(filter_params).unwrap();
        let result = weapon_filter.filter_for_new(filter_params);
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
