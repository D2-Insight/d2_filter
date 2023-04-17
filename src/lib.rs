use num_enum::{FromPrimitive, IntoPrimitive};
use rustgie::types::destiny::{
    definitions::{sockets::DestinyPlugSetDefinition, DestinyInventoryItemDefinition},
    DamageType, DestinyAmmunitionType, DestinyItemSubType, DestinyItemType, TierType,
};
use std::collections::{HashMap, HashSet};

type WeaponMap = HashMap<u32, DestinyInventoryItemDefinition>;
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
    weapons: WeaponMap,
    adept: BungieHashSet,
    perks: GunPerkMap,
}

pub enum Source {}

impl Filter {
    //This is the slowest part, but mostly because networking + bungie, everything else is fast af
    pub async fn new() -> Self {
        let client = rustgie::RustgieClientBuilder::new()
            .with_api_key("")
            .build()
            .unwrap();
        let manifest_response = client.destiny2_get_destiny_manifest(None).await.unwrap();
        let weapon_path = manifest_response
            .json_world_component_content_paths
            .clone()
            .unwrap()
            .get("en")
            .unwrap()
            .get("DestinyInventoryItemDefinition")
            .unwrap()
            .to_owned();

        let perk_path = manifest_response
            .json_world_component_content_paths
            .clone()
            .unwrap()
            .get("en")
            .unwrap()
            .get("DestinyPlugSetDefinition")
            .unwrap()
            .to_owned();

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
        let weapons = preprocess_manifest(DestinyItemType::Weapon, &inventory_items).await;
        let adept: BungieHashSet = reqwest::get("https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/adept-weapon-hashes.json").await.unwrap().json().await.unwrap();
        let mut perks: GunPerkMap = HashMap::new();
        perks.reserve(weapons.len() - perks.capacity());
        for (hash, item) in &weapons {
            let mut cat_index: Vec<i32> = Vec::new();
            for index in item.sockets.clone().unwrap().socket_categories.unwrap() {
                if index.socket_category_hash == 4241085061
                /*Weapon Perks*/
                {
                    cat_index = Vec::from(index.socket_indexes.unwrap());
                    break;
                }
            }
            let sockets = Vec::from(
                item.sockets
                    .clone()
                    .unwrap()
                    .socket_entries
                    .to_owned()
                    .unwrap(),
            );
            let mut count: u8 = 0;
            let mut perks_holy_shit: HashMap<u32, PerkSlot> = HashMap::new();
            for socket_index in cat_index.clone() {
                let socket = sockets.get(socket_index as usize).unwrap();
                if socket.single_initial_item_hash == 2302094943
                /*Kill tracker >:(*/
                {
                    continue;
                }

                //STATIC PERK
                perks_holy_shit
                    .insert(socket.single_initial_item_hash, PerkSlot::from(count as u8));
                //STATIC PERKS SOME TIMES?
                if let Some(hash) = socket.reusable_plug_items.clone() {
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
                        .clone()
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
                        .clone()
                        .unwrap()
                    {
                        perks_holy_shit.insert(perk.plug_item_hash, PerkSlot::from(count as u8));
                    }
                }
                count += 1;
            }
            perks.insert(hash.clone(), perks_holy_shit);
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

fn check_stats(stat_range: StatSplit, check_stat: i32) -> bool {
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
    items: WeaponMap,
    stats: std::collections::HashMap<u32, StatSplit>,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    '_weapons: for (hash, item) in items {
        let item_stats = item.clone().stats.unwrap().stats.unwrap();
        for (stat, stat_range) in stats.clone() {
            let stat_option = item_stats.get(&stat);
            if stat_option.is_none() {
                continue '_weapons;
            }
            if !check_stats(stat_range, stat_option.unwrap().value) {
                continue '_weapons;
            }
        }
        found_weapons.insert(hash, item);
    }
    Ok(found_weapons)
}

async fn filter_names(
    items: WeaponMap,
    search: String,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();

    for (hash, item) in items {
        if item
            .display_properties
            .clone()
            .unwrap()
            .name
            .unwrap()
            .to_lowercase()
            .contains(search.to_lowercase().as_str())
        {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

pub async fn filter_perks(
    perks: GunPerkMap,
    items: WeaponMap,
    search: PerkMap,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        for (perk_hash, slot) in &search {
            if let Some(actual_slot) = perks.get(&hash).unwrap().get(&perk_hash) {
                if slot == actual_slot
                    || (slot == &PerkSlot::LeftRight
                        && matches!(actual_slot, &PerkSlot::Left | &PerkSlot::Right))
                {
                    found_weapons.insert(hash, item.clone());
                }
            }
        }
    }
    Ok(found_weapons)
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
    items: WeaponMap,
    search: DestinyItemSubType,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if item.item_sub_type == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

async fn filter_craftable(
    items: WeaponMap,
    search: bool,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if item.inventory.clone().unwrap().recipe_item_hash.is_some() == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

async fn filter_frame() {}

async fn filter_energy(
    items: WeaponMap,
    search: DamageType,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if item.default_damage_type.clone() == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

async fn filter_rarity(
    items: WeaponMap,
    search: TierType,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if item.inventory.clone().unwrap().tier_type == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

async fn filter_adept(
    items: WeaponMap,
    search: bool,
    adept: BungieHashSet,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();

    for (hash, item) in items {
        if adept.get(&hash).is_some() == search {
            found_weapons.insert(hash, item);
        }
    }
    return Ok(found_weapons);
}

async fn filter_slot(
    items: WeaponMap,
    search: WeaponSlot,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        let weapon_slot = item
            .clone()
            .equipping_block
            .unwrap()
            .equipment_slot_type_hash;
        if weapon_slot == search.to_owned() as u32 {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

async fn filter_ammo(
    items: WeaponMap,
    search: DestinyAmmunitionType,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if item.equipping_block.clone().unwrap().ammo_type == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}
impl Filter {
    pub async fn filter_for(
        &self,
        search: FilterRequest,
    ) -> Result<WeaponMap, Box<dyn std::error::Error>> {
        let mut buffer = self.weapons.clone();
        if let Some(query) = search.family {
            buffer = filter_weapon_type(buffer, query).await?;
        }
        if let Some(query) = search.perks {
            buffer = filter_perks(self.perks.clone(), buffer, query).await?;
        }
        if let Some(query) = search.ammo {
            buffer = filter_ammo(buffer, query).await?;
        }
        if let Some(query) = search.slot {
            buffer = filter_slot(buffer, query).await?;
        }
        if let Some(query) = search.energy {
            buffer = filter_energy(buffer, query).await?;
        }
        if let Some(query) = search.rarity {
            buffer = filter_rarity(buffer, query).await?;
        }
        if let Some(query) = search.adept {
            buffer = filter_adept(buffer, query, self.adept.clone()).await?;
        }
        if let Some(query) = search.craftable {
            buffer = filter_craftable(buffer, query).await?;
        }
        if let Some(query) = search.stats {
            buffer = filter_stats(buffer, query).await?;
        }
        if let Some(query) = search.name {
            buffer = filter_names(buffer, query).await?;
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {

    use rustgie::types::destiny::*;

    use crate::{FilterRequest, PerkMap, PerkSlot};

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
        assert_eq!(result.get(&3193598749).is_some(), true);
        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_perks() {
        let weapon_filter = crate::Filter::new().await;
        let mut filter_params = FilterRequest::new();
        // /filter_params.perks = Some(365154968);
        let mut perks: PerkMap = std::collections::HashMap::new();
        perks.insert(3619207468, PerkSlot::LeftRight);
        filter_params.perks = Some(perks);
        let start = std::time::Instant::now();
        let result = weapon_filter.filter_for(filter_params).await.unwrap();
        let duration = start.elapsed();
        // /println!("{:?}", result);
        println!("{} MS", duration.as_millis());
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
        println!("{}", duration.as_millis());
    }
}
