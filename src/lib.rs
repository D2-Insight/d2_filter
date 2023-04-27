use num_enum::{FromPrimitive, IntoPrimitive};
use rustgie::types::{
    api_response_::BungieApiResponse,
    destiny::{
        config::DestinyManifest,
        definitions::{sockets::DestinyPlugSetDefinition, DestinyInventoryItemDefinition},
        DamageType, DestinyAmmunitionType, DestinyItemSubType, DestinyItemType, TierType,
    },
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type WeaponMap = HashMap<u32, DestinyInventoryItemDefinition>;
type WeaponArray = Vec<DestinyInventoryItemDefinition>;
type BungieHash = u32;
type BungieHashSet = HashSet<BungieHash>;
type PlugMap = HashMap<u32, DestinyPlugSetDefinition>;
type PerkMap = HashMap<BungieHash, PerkSlot>;
/// K: GunHash V: Guns that use it
type GunPerkMap = HashMap<BungieHash, PerkMap>;
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

pub enum Source {}

impl Filter {
    //This is the slowest part, but mostly because networking + bungie, everything else is fast af
    pub async fn new() -> Self {
        let test: BungieApiResponse<DestinyManifest> =
            reqwest::get("https://www.bungie.net/Platform/Destiny2/Manifest/")
                .await
                .unwrap()
                .json()
                .await
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
            reqwest::get(format!("https://www.bungie.net{weapon_path}"))
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

        let plug_sets: PlugMap = reqwest::get(format!("https://www.bungie.net{perk_path}"))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let weapon_manifest = preprocess_manifest(DestinyItemType::Weapon, &inventory_items).await;
        let weapons: Vec<DestinyInventoryItemDefinition> =
            Vec::from_iter(weapon_manifest.values().cloned());
        let adept: BungieHashSet = reqwest::get("https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/adept-weapon-hashes.json").await.unwrap().json().await.unwrap();
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
            let mut perks_holy_shit: HashMap<u32, PerkSlot> = HashMap::new();
            for socket_index in &cat_index {
                let socket = socket_entries.get(*socket_index as usize).unwrap();
                if socket.single_initial_item_hash == 2302094943
                /*Kill tracker >:(*/
                {
                    continue;
                }

                //STATIC PERK
                perks_holy_shit
                    .insert(socket.single_initial_item_hash, PerkSlot::from(count as u8));
                //STATIC PERKS SOME TIMES?
                if let Some(hash) = &socket.reusable_plug_items {
                    for static_perk in hash {
                        perks_holy_shit
                            .insert(static_perk.plug_item_hash, PerkSlot::from(count as u8));
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
                        perks_holy_shit.insert(perk.plug_item_hash, PerkSlot::from(count as u8));
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
                        perks_holy_shit.insert(perk.plug_item_hash, PerkSlot::from(count as u8));
                    }
                }
                count += 1;
            }
            perks.insert(hash, perks_holy_shit);
        }
        Filter {
            weapons: weapons,
            adept: adept,
            perks: perks,
        }
    }
}
#[allow(dead_code)]
pub struct FilterRequest {
    //family: Option<DestinyItemType>,
    pub family: Option<DestinyItemSubType>,
    pub stats: Option<HashMap<BungieHash, StatSplit>>, //probably need to change this to a vec
    pub energy: Option<DamageType>,
    pub slot: Option<WeaponSlot>,
    pub adept: Option<bool>,
    pub craftable: Option<bool>,
    pub name: Option<String>,
    pub rarity: Option<TierType>,
    pub ammo: Option<DestinyAmmunitionType>,
    pub perks: Option<PerkMap>,
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

//pub enum Source {}

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

#[allow(dead_code)]
#[derive(Clone)]
pub enum StatSplit {
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

///Removes all unneeded manifest BS
async fn preprocess_manifest(item_type: DestinyItemType, map: &WeaponMap) -> WeaponMap {
    let mut buffer: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in map {
        if item.item_type == item_type {
            buffer.insert(hash.to_owned(), item.to_owned());
        }
    }
    return buffer;
}

fn check_stats(stat_range: &StatSplit, check_stat: &i32) -> bool {
    match stat_range {
        StatSplit::Above(stat_above) => stat_above < check_stat,
        StatSplit::Between(stat_low, stat_high) => stat_high > check_stat || stat_low < check_stat,
        StatSplit::Below(stat_below) => stat_below > check_stat,
        StatSplit::AtOrAbove(stat_above) => stat_above <= check_stat,
        StatSplit::AtOrBetween(stat_low, stat_high) => {
            stat_high >= check_stat || stat_low <= check_stat
        }
        StatSplit::AtOrBelow(stat_below) => stat_below >= check_stat,
        StatSplit::At(stat_at) => stat_at == check_stat,
        _ => true,
    }
}

async fn filter_stats(
    items: WeaponArray,
    stats: std::collections::HashMap<u32, StatSplit>,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    '_weapons: for item in items {
        let item_stats = item.stats.as_ref().unwrap().stats.as_ref().unwrap();
        for (stat, stat_range) in &stats {
            let stat_option = item_stats.get(&stat);
            if stat_option.is_none() {
                continue '_weapons;
            }
            if !check_stats(stat_range, &stat_option.unwrap().value) {
                continue '_weapons;
            }
        }
        found_weapons.push(item);
    }
    Ok(found_weapons)
}

fn filter_stats_new(
    item: &DestinyInventoryItemDefinition,
    stats: &std::collections::HashMap<u32, StatSplit>,
) -> bool {
    let item_stats = item.stats.as_ref().unwrap().stats.as_ref().unwrap();
    for (stat, stat_range) in stats {
        if let Some(stat_option) = item_stats.get(&stat) {
            if !check_stats(stat_range, &stat_option.value) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

async fn filter_names(
    items: WeaponArray,
    search: String,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        if item
            .display_properties
            .as_ref()
            .unwrap()
            .name
            .as_ref()
            .unwrap()
            .to_lowercase()
            .contains(search.to_lowercase().as_str())
        {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_names_new(item: &DestinyInventoryItemDefinition, search: &str) -> bool {
    item.display_properties
        .as_ref()
        .unwrap()
        .name
        .as_ref()
        .unwrap()
        .to_lowercase()
        .contains(search.to_lowercase().as_str())
}

pub async fn filter_perks(
    perks: GunPerkMap,
    items: WeaponArray,
    search: PerkMap,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        let hash = &item.hash;
        let mut condition = true;
        for (perk_hash, slot) in &search {
            if let Some(actual_slot) = perks.get(hash).unwrap().get(&perk_hash) {
                if slot != actual_slot
                    && !(slot == &PerkSlot::LeftRight
                        && matches!(actual_slot, &PerkSlot::Left | &PerkSlot::Right))
                {
                    condition = false;
                }
            }
        }
        if condition == true {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_perks_new(
    perks: &GunPerkMap,
    item: &DestinyInventoryItemDefinition,
    search: &PerkMap,
) -> bool {
    let hash = &item.hash;

    for (perk_hash, slot) in search {
        if let Some(actual_slot) = perks.get(hash).unwrap().get(&perk_hash) {
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
/*async fn filter_source(
    items: WeaponMap,
    search: BungieHash,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if item.inventory == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}*/

async fn filter_weapon_type(
    items: WeaponArray,
    search: DestinyItemSubType,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        if item.item_sub_type == search {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_weapon_type_new(
    item: &DestinyInventoryItemDefinition,
    search: DestinyItemSubType,
) -> bool {
    item.item_sub_type == search
}

async fn filter_craftable(
    items: WeaponArray,
    search: bool,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        if item.inventory.as_ref().unwrap().recipe_item_hash.is_some() == search {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_craftable_new(item: &DestinyInventoryItemDefinition, search: bool) -> bool {
    item.inventory.as_ref().unwrap().recipe_item_hash.is_some() == search
}

async fn filter_frame() {}

async fn filter_energy(
    items: WeaponArray,
    search: DamageType,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        if item.default_damage_type == search {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_energy_new(item: &DestinyInventoryItemDefinition, search: DamageType) -> bool {
    item.default_damage_type == search
}

async fn filter_rarity(
    items: WeaponArray,
    search: TierType,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        if item.inventory.as_ref().unwrap().tier_type == search {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_rarity_new(item: &DestinyInventoryItemDefinition, search: TierType) -> bool {
    item.inventory.as_ref().unwrap().tier_type == search
}

async fn filter_adept(
    items: WeaponArray,
    search: bool,
    adept: BungieHashSet,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();

    for item in items {
        if adept.get(&item.hash).is_some() == search {
            found_weapons.push(item);
        }
    }
    return Ok(found_weapons);
}

fn filter_adept_new(
    item: &DestinyInventoryItemDefinition,
    search: bool,
    adept: &BungieHashSet,
) -> bool {
    adept.get(&item.hash).is_some() == search
}

async fn filter_slot(
    items: WeaponArray,
    search: WeaponSlot,
) -> Result<WeaponArray, Box<dyn std::error::Error>> {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        let weapon_slot = item
            .equipping_block
            .as_ref()
            .unwrap()
            .equipment_slot_type_hash;
        if weapon_slot == search as u32 {
            found_weapons.push(item);
        }
    }
    Ok(found_weapons)
}

fn filter_slot_new(item: &DestinyInventoryItemDefinition, search: u32) -> bool {
    item.equipping_block
        .as_ref()
        .unwrap()
        .equipment_slot_type_hash
        == search
}

async fn filter_ammo(items: WeaponArray, search: DestinyAmmunitionType) -> WeaponArray {
    let mut found_weapons: WeaponArray = Vec::new();
    for item in items {
        if item.equipping_block.as_ref().unwrap().ammo_type == search {
            found_weapons.push(item);
        }
    }
    found_weapons
}

fn filter_ammo_new(item: &DestinyInventoryItemDefinition, search: DestinyAmmunitionType) -> bool {
    item.equipping_block.as_ref().unwrap().ammo_type == search
}

pub fn filter_new(
    item: &DestinyInventoryItemDefinition,
    search: &FilterRequest,
    adept: &BungieHashSet,
    perks: &GunPerkMap,
) -> bool {
    if let Some(query) = search.ammo {
        if !filter_ammo_new(item, query) {
            return false;
        }
    }
    if let Some(query) = search.adept {
        if !filter_adept_new(item, query, adept) {
            return false;
        }
    }
    if let Some(query) = search.energy {
        if !filter_energy_new(item, query) {
            return false;
        }
    }
    if let Some(query) = search.family {
        if !filter_weapon_type_new(item, query) {
            return false;
        }
    }
    if let Some(query) = search.slot {
        if !filter_slot_new(item, query.into()) {
            return false;
        }
    }
    if let Some(query) = search.rarity {
        if !filter_rarity_new(item, query) {
            return false;
        }
    }
    if let Some(query) = search.craftable {
        if !filter_craftable_new(item, query) {
            return false;
        }
    }
    if let Some(query) = &search.perks {
        if !filter_perks_new(perks, item, query) {
            return false;
        }
    }
    if let Some(query) = &search.stats {
        if !filter_stats_new(item, query) {
            return false;
        }
    }
    if let Some(query) = &search.name {
        if !filter_names_new(item, query.as_str()) {
            return false;
        }
    }
    true
}

//SUPER fast.
impl Filter {
    pub fn filter_for_new(&self, search: FilterRequest) -> WeaponArray {
        let mut result: WeaponArray = Vec::new();
        for item in &self.weapons {
            if filter_new(item, &search, &self.adept, &self.perks) {
                result.push(item.to_owned());
            }
        }
        //buffer.retain(|item| filter_new(item, &search, &self.adept, &self.perks));
        result
    }
}

impl Filter {
    pub async fn filter_for(
        &self,
        search: FilterRequest,
    ) -> Result<WeaponArray, Box<dyn std::error::Error>> {
        let mut buffer = self.weapons.clone();
        if let Some(query) = search.ammo {
            //buffer = filter_ammo(buffer, query).await;
            buffer.retain(|item| filter_ammo_new(item, query));
        }
        if let Some(query) = search.family {
            //buffer = filter_weapon_type(buffer, query).await?;
            buffer.retain(|item| filter_weapon_type_new(item, query))
        }
        if let Some(query) = search.perks {
            //buffer = filter_perks(self.perks.clone(), buffer, query).await?;
            buffer.retain(|item| filter_perks_new(&self.perks, item, &query));
        }
        if let Some(query) = search.slot {
            //buffer = filter_slot(buffer, query).await?;
            let query = query as u32;
            buffer.retain(|item| filter_slot_new(item, query))
        }
        if let Some(query) = search.energy {
            //buffer = filter_energy(buffer, query).await?;
            buffer.retain(|item| filter_energy_new(item, query));
        }
        if let Some(query) = search.rarity {
            //buffer = filter_rarity(buffer, query).await?;
            buffer.retain(|item| filter_rarity_new(item, query));
        }
        if let Some(query) = search.adept {
            //buffer = filter_adept(buffer, query, self.adept.clone()).await?;
            buffer.retain(|item| filter_adept_new(item, query, &self.adept));
        }
        if let Some(query) = search.craftable {
            //buffer = filter_craftable(buffer, query).await?;
            buffer.retain(|item| filter_craftable_new(item, query));
        }
        if let Some(query) = search.stats {
            //buffer = filter_stats(buffer, query).await?;
            buffer.retain(|item| filter_stats_new(item, &query));
        }
        if let Some(query) = search.name {
            //buffer = filter_names(buffer, query).await?;
            buffer.retain(|item| filter_names_new(item, query.as_str()));
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {

    use rustgie::types::destiny::*;

    use crate::{BungieHash, FilterRequest, PerkMap, PerkSlot, StatHashes, StatSplit};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test() {
        let weapon_filter = crate::Filter::new().await;
        let mut filter_params = FilterRequest::new();
        filter_params.adept = Some(true);
        filter_params.family = Some(DestinyItemSubType::SubmachineGun);
        filter_params.slot = Some(crate::WeaponSlot::Top);
        filter_params.energy = Some(DamageType::Strand);
        let start = std::time::Instant::now();
        let result = weapon_filter.filter_for(filter_params).await.unwrap();
        let duration = start.elapsed();
        println!("{}", duration.as_millis());
        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_perks() {
        let weapon_filter = crate::Filter::new().await;
        let mut filter_params = FilterRequest::new();
        // /filter_params.perks = Some(365154968);
        //let mut perks: PerkMap = std::collections::HashMap::new();
        //perks.insert(3619207468, PerkSlot::LeftRight);
        //filter_params.perks = Some(perks);
        let mut stats: HashMap<BungieHash, StatSplit> = HashMap::new();
        //filter_params.family = Some(DestinyItemSubType::RocketLauncher);
        stats.insert(StatHashes::Velocity.into(), StatSplit::Below(35));
        //filter_params.ammo = Some(DestinyAmmunitionType::Heavy);
        filter_params.stats = Some(stats);
        let start: std::time::Instant = std::time::Instant::now();
        //let result = weapon_filter.filter_for(filter_params).await.unwrap();
        let result = weapon_filter.filter_for_new(filter_params);
        let duration = start.elapsed();
        //println!("{:?}", result);
        println!("{} MS", duration.as_micros());
        println!("{} Items", result.len());

        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        //assert_eq!(result.get(&3193598749).is_some(), true);
    }

    #[tokio::test]
    async fn test_rose() {
        let weapon_filter = crate::Filter::new().await;
        let start = std::time::Instant::now();

        let test = weapon_filter.perks.get(&854379020).unwrap();
        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        let duration = start.elapsed();

        println!("{:?}", test);
        println!("{}", duration.as_nanos());
    }
}
