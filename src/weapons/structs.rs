use std::collections::HashMap;

use d2_minify::{foundry::MiniFoundry, icons::MiniIcon, stats::MiniStat, watermark::MiniWatermark};
use serde::Deserialize;

//Planning on reducing memory usage by preprocessing manifest into this struct.
//craftable, adept, and sunset should just be in a seperate hashset to reduce space.
//I could reduce a lot of these to u8 but keeping it near bungie spec
#[derive(Clone, Deserialize)]
pub struct MinimizedWeapon {
    pub name: String,
    pub hash: u32,
    pub slot: u8,
    pub energy: u8,
    pub rarity: u8,
    pub ammo_type: u8,
    pub weapon_type: u8,
    pub stats: HashMap<MiniStat, i8>,
    pub icon: MiniIcon,
    pub season: MiniWatermark,
    pub foundry: MiniFoundry,
}
