use num_enum::IntoPrimitive;
use rustgie::types::destiny::{
    definitions::{
        DestinyInventoryItemDefinition, DestinyItemCategoryDefinition,
        DestinyItemCraftingBlockBonusPlugDefinition, DestinyRewardSourceCategory,
    },
    DamageType, DestinyAmmunitionType, DestinyEnergyType, DestinyItemSubType, DestinyItemType,
    TierType,
};
use std::collections::HashMap;
use std::{cell::RefCell, collections::HashSet};

type WeaponMap = HashMap<u32, DestinyInventoryItemDefinition>;
type BungieHash = u32;
type BungieHashSet = HashSet<BungieHash>;

thread_local! {
    pub static WEAPONS: RefCell<WeaponMap> = RefCell::new(WeaponMap::new());
    pub static ADEPT: RefCell<BungieHashSet> = RefCell::new(BungieHashSet::new());
}
#[allow(dead_code)]
#[derive(Default)]
pub struct FilterRequest {
    items: WeaponMap,
    //family: Option<DestinyItemType>,
    familty: Option<DestinyItemSubType>,
    stats: Option<HashMap<BungieHash, StatSplit>>, //probably need to change this to a vec
    energy: Option<DamageType>,
    adept: Option<bool>,
    craftable: Option<bool>,
    name: Option<String>,
    source: Option<Source>,
}

pub enum Source {}

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
enum StatSplit {
    Above(i32),
    Between(i32, i32),
    Below(i32),
    AtOrAbove(i32),
    AtOrBelow(i32),
    AtOrBetween(i32, i32),
    At(i32),
}

#[derive(IntoPrimitive, Clone, Copy)]
#[repr(u32)]
enum Slot {
    Top = 1498876634,
    Middle = 2465295065,
    Bottom = 953998645,
}

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

async fn filter_names() {}

async fn filter_perks() {}

async fn filter_source() {}

async fn filter_weapon_type(
    items: WeaponMap,
    search: DestinyItemSubType,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();
    for (hash, item) in items {
        if (item.item_sub_type == search) {
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
        let buffer = match item.clone().inventory.unwrap().recipe_item_hash {
            Some(_) => true,
            None => false,
        };
        if buffer == search {
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
        if item.clone().default_damage_type == search {
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
        if item.clone().inventory.unwrap().tier_type == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}

async fn filter_adept(
    items: WeaponMap,
    search: bool,
) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();

    for (hash, item) in items {
        ADEPT.with(|data| {
            let buffer = match data.borrow().get(&hash) {
                Some(_) => true,
                None => false,
            };
            if buffer == search {
                found_weapons.insert(hash, item);
            }
        })
    }
    Ok(found_weapons)
}

async fn whatever(items: WeaponMap, search: bool) -> Result<WeaponMap, Box<dyn std::error::Error>> {
    let mut found_weapons: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();

    for (hash, item) in items {
        //do craftable thingy
    }
    Ok(found_weapons)
}

async fn filter_slot(
    items: WeaponMap,
    search: Slot,
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
        if item.clone().equipping_block.unwrap().ammo_type == search {
            found_weapons.insert(hash, item);
        }
    }
    Ok(found_weapons)
}
async fn filter_handler() {}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rustgie::RustgieClientBuilder::new()
        .with_api_key("a9ef58513a6d4a87bf329b34c20ddc85")
        .build()?;
    let manifest_response = client.destiny2_get_destiny_manifest(None).await?;
    let weapon_path = manifest_response
        .json_world_component_content_paths
        .clone()
        .unwrap()
        .get("en")
        .unwrap()
        .get("DestinyInventoryItemDefinition")
        .unwrap()
        .to_owned();
    let manifestjson: WeaponMap = reqwest::get(format!("https://www.bungie.net{weapon_path}"))
        .await?
        .json()
        .await?;

    ADEPT.with(|data| {
        data.borrow_mut().clear();
        let buffer: BungieHashSet = reqwest::blocking::get("https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/adept-weapon-hashes.json").unwrap().json().unwrap();
        data.borrow_mut().extend(buffer.iter())});

    let buffer = preprocess_manifest(DestinyItemType::Weapon, &manifestjson).await;

    let mut stats: HashMap<u32, StatSplit> = HashMap::new();
    stats.insert(StatHashes::Reload.into(), StatSplit::At(55));
    stats.insert(StatHashes::Handling.into(), StatSplit::At(60));
    stats.insert(StatHashes::Range.into(), StatSplit::At(43));

    let start = std::time::Instant::now();
    /*let found = filter_stats(buffer, stats, DestinyItemSubType::HandCannon)
        .await
        .unwrap();
    */
    /*let found = filter_ammo(buffer, DestinyAmmunitionType::Primary).await?;

    let found = filter_stats(weapons, stats).await?;
    let found = filter_slot(found, Slot::Top).await?;*/
    let weapons = filter_weapon_type(buffer, DestinyItemSubType::HandCannon).await?;
    let found = filter_adept(weapons, true).await?;
    let end = start.elapsed();
    println!("{:?}", found);
    println!("{:?}", end);

    //print!("{}", test);
    Ok(())
}
