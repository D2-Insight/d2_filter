use serde_repr::Deserialize_repr;
use num_enum::IntoPrimitive;

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