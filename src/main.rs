use num_enum::IntoPrimitive;
use rustgie::types::destiny::{
    definitions::DestinyInventoryItemDefinition,
    historical_stats::definitions::DestinyStatsCategoryType, DestinyItemSubType, DestinyItemType,
};
use std::collections::HashMap;

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum StatHashes {
    ACCURACY = 1591432999,
    AIM_ASSIST = 1345609583,
    AIRBORNE = 2714457168,
    AMMO_CAPACITY = 925767036,
    ATTACK = 1480404414,
    BLAST_RADIUS = 3614673599,
    CHARGE_RATE = 3022301683,
    CHARGE_TIME = 2961396640,
    DRAW_TIME = 447667954,
    GUARD_EFFICIENCY = 2762071195,
    GUARD_ENDURANCE = 3736848092,
    GUARD_RESISTANCE = 209426660,
    HANDLING = 943549884,
    IMPACT = 4043523819,
    INVENTORY_SIZE = 1931675084,
    MAGAZINE = 3871231066,
    RANGE = 1240592695,
    RECOIL_DIR = 2715839340,
    RECOVERY = 1943323491,
    RELOAD = 4188031367,
    RPM = 4284893193,
    SHIELD_DURATION = 1842278586,
    SWING_SPEED = 2837207746,
    VELOCITY = 2523465841,
    ZOOM = 3555269338,
    UNKNOWN = 0,
}
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
            let stat_option = &item_stats.get(&stat);

            let check_stat = match stat_option {
                Some(stat) => stat.value,
                None => {
                    condition = false;
                    break;
                }
            };

            if !match stat_range {
                StatSplit::Above(stat_above) => stat_above < check_stat,
                StatSplit::Between(stat_low, stat_high) => {
                    stat_high > check_stat || stat_low < check_stat
                }
                StatSplit::Below(stat_below) => stat_below > check_stat,
                StatSplit::AtOrAbove(stat_above) => stat_above <= check_stat,
                StatSplit::AtOrBetween(stat_low, stat_high) => {
                    stat_high >= check_stat || stat_low <= check_stat
                }
                StatSplit::AtOrBelow(stat_below) => stat_below >= check_stat,
                StatSplit::At(stat_at) => stat_at == check_stat,
            } {
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
    let mut buffer: HashMap<u32, DestinyInventoryItemDefinition> = HashMap::new();

    for (hash, item) in manifestjson {
        if item.item_type == DestinyItemType::Weapon {
            buffer.insert(hash, item);
        }
    }

    let mut stats: HashMap<u32, StatSplit> = HashMap::new();
    //stats.insert(StatHashes::RANGE.into(), StatSplit::Above(70));
    stats.insert(StatHashes::RANGE.into(), StatSplit::Below(25));

    let start = std::time::Instant::now();
    let found = find_weps(buffer, stats, DestinyItemSubType::HandCannon).unwrap();
    let end = start.elapsed();
    println!("{:?}", found);
    println!("{:?}", end);

    //print!("{}", test);
    Ok(())
}
