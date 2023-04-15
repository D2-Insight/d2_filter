use num_enum::IntoPrimitive;
use rustgie::types::destiny::{
    definitions::DestinyInventoryItemDefinition, DestinyItemSubType, DestinyItemType,
};
use std::collections::HashMap;

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

fn preprocess_manifest(
    item_type: DestinyItemType,
    map: &std::collections::HashMap<u32, DestinyInventoryItemDefinition>,
) -> std::collections::HashMap<u32, DestinyInventoryItemDefinition> {
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

fn find_weps(
    items: std::collections::HashMap<u32, DestinyInventoryItemDefinition>,
    stats: std::collections::HashMap<u32, StatSplit>,
    weapon_sub_type: DestinyItemSubType,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut found_weapons: Vec<u32> = Vec::new();
    for (hash, item) in items {
        if item.item_type != DestinyItemType::Weapon
            || (item.item_sub_type != weapon_sub_type
                && weapon_sub_type != DestinyItemSubType::None)
        {
            continue;
        }
        //Check all stat conditions are met for current gun
        let item_stats = item.stats.unwrap().stats.unwrap();
        let mut condition = true;
        for (stat, stat_range) in stats.clone() {
            let stat_option = item_stats.get(&stat);

            let check_stat = match stat_option {
                Some(stat) => stat.value,
                None => {
                    condition = false;
                    break;
                }
            };

            if !check_stats(stat_range, check_stat) {
                condition = false;
                break;
            }
        }
        if condition == true {
            found_weapons.push(hash);
        }
    }
    Ok(found_weapons)
}

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
    let manifestjson: HashMap<u32, DestinyInventoryItemDefinition> =
        reqwest::get(format!("https://www.bungie.net{weapon_path}"))
            .await?
            .json()
            .await?;

    let buffer = preprocess_manifest(DestinyItemType::Weapon, &manifestjson);

    let mut stats: HashMap<u32, StatSplit> = HashMap::new();
    //stats.insert(StatHashes::RANGE.into(), StatSplit::Above(70));
    stats.insert(StatHashes::Range.into(), StatSplit::Below(25));

    let start = std::time::Instant::now();
    let found = find_weps(buffer, stats, DestinyItemSubType::HandCannon).unwrap();
    let end = start.elapsed();
    println!("{:?}", found);
    println!("{:?}", end);

    //print!("{}", test);
    Ok(())
}
