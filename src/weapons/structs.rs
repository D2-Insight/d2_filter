use std::collections::HashMap;

use rustgie_types::destiny::{DamageType, DestinyAmmunitionType, DestinyItemSubType, TierType};
use serde::Deserialize;

use crate::{BungieHash, generic::MiniIcon};

//Planning on reducing memory usage by preprocessing manifest into this struct.
//craftable, adept, and sunset should just be in a seperate hashset to reduce space.
//I could reduce a lot of these to u8 but keeping it near bungie spec
#[derive(Clone, Deserialize)]
pub struct MinimizedWeapon {
    pub name: String,
    pub hash: u32,
    pub slot: u32,
    pub energy: DamageType,
    pub rarity: TierType,
    pub ammo_type: DestinyAmmunitionType,
    pub weapon_type: DestinyItemSubType,
    pub stats: HashMap<BungieHash, i32>,
    pub icon: MiniIcon,
}
