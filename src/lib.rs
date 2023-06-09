pub mod enums;
pub mod inventory_items;
pub mod weapons;

use d2_minify::stats::MiniStat;
use enums::*;
use std::collections::{HashMap, HashSet};
pub type BungieHash = u32;
pub type WeaponHash = u32;
pub type PerkHash = u32;
type StatVec = Vec<(MiniStat, StatFilter)>;
type BungieHashSet = HashSet<BungieHash>;
pub type PerkMap = HashMap<WeaponHash, PerkSlot>;
/// K: PerkHash V: Guns that use it
type GunPerkMap = HashMap<PerkHash, PerkMap>;

#[cfg(test)]
mod tests {

    use crate::{
        weapons::lib::{WeaponFilter, WeaponRequest},
        BungieHash, StatFilter,
    };

    #[test]
    fn test() {
        let weapon_filter = WeaponFilter::new();
        let mut filter_params = WeaponRequest::new();
        //filter_params.adept = Some(true);
        //filter_params.family = Some(DestinyItemSubType::SubmachineGun);
        //filter_params.slot = Some(crate::WeaponSlot::Top);
        //filter_params.energy = Some(DamageType::Strand);
        filter_params.ammo = Some(1);
        //let mut perks: std::collections::HashMap<u32, crate::PerkSlot> =
        //    std::collections::HashMap::new();
        //perks.insert(365154968, crate::PerkSlot::LeftRight);
        //filter_params.perks = Some(perks);
        let start = std::time::Instant::now();
        let result = weapon_filter.filter_for(filter_params);
        let duration = start.elapsed();
        println!("{} ms", duration.as_micros());
        println!("{}", result.len());
        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        //assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_perks() {
        let weapon_filter = WeaponFilter::new();
        let mut filter_params = WeaponRequest::new();
        // /filter_params.perks = Some(365154968);
        //let mut perks: PerkMap = std::collections::HashMap::new();
        //perks.insert(3619207468, PerkSlot::LeftRight);
        //filter_params.perks = Some(perks);
        let mut stats: Vec<(BungieHash, StatFilter)> = Vec::new();
        filter_params.foundry = Some(d2_minify::foundry::MiniFoundry::Daito);
        //filter_params.family = Some(DestinyItemSubType::RocketLauncher);
        //filter_params.name = Some("Sunshot".to_string());
        //filter_params
        //filter_params.adept = Some(true);
        //filter_params.ammo = Some(DestinyAmmunitionType::Heavy);
        //filter_params.stats = Some(stats);
        let start: std::time::Instant = std::time::Instant::now();
        //let result = weapon_filter.filter_for(filter_params).unwrap();
        let result = weapon_filter.filter_for(filter_params);
        let duration = start.elapsed();
        //println!("{:?}", result);
        println!("{} Micro Seconds", duration.as_micros());
        println!("{} Items", result.len());
        //println!("{}", String::from(result.get(0).unwrap().icon));
        /*print!(
            "{}\n",
            Option::<String>::from(result.get(0).unwrap().season).unwrap()
        );*/
        for items in result {
            println!("{}", items.name);
        }

        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        //assert_eq!(result.get(&3193598749).is_some(), true);
    }

    #[test]
    fn test_rose() {
        let weapon_filter = WeaponFilter::new();
        let start = std::time::Instant::now();

        let test = weapon_filter.perks.get(&854379020).unwrap();
        //println!("{:?}", weapon_filter.perks.get(&3193598749).unwrap());
        let duration = start.elapsed();

        println!("{:?}", test);
        println!("{}", duration.as_nanos());
    }
}
