///This is used to produce links for icons
///Example: "https://www.bungie.net/common/destiny2_content/icons/0f584e8a13b2cc4cb60379b1777362e5.jpg"
///Doing 4 u64s instead of 2 u128s for wasm compatability
#[derive(serde::Deserialize, Clone, Copy)]
pub struct MiniIcon {
    icon_array: [u64; 2],
}

impl MiniIcon {
    fn new(url: String) -> Option<MiniIcon> {
        let hex = url
            .strip_prefix("/common/destiny2_content/icons/")
            .unwrap_or_default()
            .strip_suffix(".jpg")
            .unwrap_or_default();
        if hex == String::default() || hex.len() != 32 {
            return None;
        }
        //Should be safe after confirming the rest? Unless bungie changes from hex :/
        //Only will be used during pregen anyways so o7
        Some(MiniIcon {
            icon_array: [
                u64::from_str_radix(&hex[0..=15], 16).unwrap(),
                u64::from_str_radix(&hex[16..=31], 16).unwrap(),
            ],
        })
    }
}

impl From<MiniIcon> for String {
    fn from(val: MiniIcon) -> Self {
        format!(
            "https://www.bungie.net/common/destiny2_content/icons/{:016x}{:016x}.jpg",
            val.icon_array[0], val.icon_array[1]
        )
    }
}

#[derive(serde_repr::Deserialize_repr, Clone, Copy, num_enum::FromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum MiniSeason {
    #[default]
    Unknown = 0,
    RedWar = 1,
    CurseOfOsiris = 2,
    Warmind = 3,
    Forsaken = 4, //Technically season of outlaw unfortnite
    Forge = 5,
    Drifter = 6,
    Opulence = 7,
    Shadowkeep = 8,
    Undying = 9,
    Dawn = 10,
    Worthy = 11,
    Arrivals = 12,
    BeyondLight = 13,
    Hunt = 14,
    Chosen = 15,
    Splicer = 16,
    Lost = 17,
    Anniversary = 18,
    WitchQueen = 19,
    Risen = 20,
    Haunted = 21,
    Plunder = 22,
    Seraph = 23,
    LightFall = 24,
    Defiance = 25,
}

impl From<MiniSeason> for Option<String> {
    fn from(val: MiniSeason) -> Self {
        let buffer = match val {
            MiniSeason::RedWar => "0dac2f181f0245cfc64494eccb7db9f7",
            MiniSeason::CurseOfOsiris => "591f14483308beaad3278c3cd397e284",
            MiniSeason::Warmind => "e10338777d1d8633e073846e613a1c1f",
            MiniSeason::Forsaken => "0669efb55951e8bc9e99f3989eacc861",
            MiniSeason::Forge => "bbddbe06ab72b61e708afc4fdbe15d95",
            MiniSeason::Drifter => "f9110e633634d112cff72a67159e3b12",
            MiniSeason::Opulence => "785e5a64153cabd5637d68dcccb7fea6",
            MiniSeason::Shadowkeep => "8aae1c411642683d341b2c4f16a7130c",
            MiniSeason::Undying => "d4141b2247cf999c73d3dc409f9d00f7",
            MiniSeason::Dawn => "ac012e11fa8bb032b923ad85e2ffb29c",
            MiniSeason::Worthy => "3d335ddc3ec6668469aae60baad8548d",
            MiniSeason::Arrivals => "796813aa6cf8afe55aed4efc2f9c609b",
            MiniSeason::BeyondLight => "0aff1f4463f6f44e9863370ab1ce6983",
            MiniSeason::Hunt => "2347cc2407b51e1debbac020bfcd0224",
            MiniSeason::Chosen => "6a52f7cd9099990157c739a8260babea",
            MiniSeason::Splicer => "b07d89064a1fc9a8e061f59b7c747fa5",
            MiniSeason::Lost => "4368a3e344977c5551407845ede830c2",
            MiniSeason::Anniversary => "dd4dd93c5606998595d9e5a06d5bfc9c",
            MiniSeason::WitchQueen => "4fe83598190610f122497d22579a1fd9",
            MiniSeason::Risen => "b0406992c49c84bdc5febad94048dc01",
            MiniSeason::Haunted => "81edbfbf0bacf8e2117c00d1d6115f1b",
            MiniSeason::Plunder => "f359d68324ae21522c299983ff1ef9f2",
            MiniSeason::Seraph => "1a68ada4fb21371c5f2b7e2eae1ebce8",
            MiniSeason::LightFall => "849de2c6bd5e9b8ced8abe8cca56d724",
            MiniSeason::Defiance => "e6af18ae79b74e76dab327ec183f8228",
            _ => {
                return None;
            }
        };
        Some(format!(
            "https://www.bungie.net/common/destiny2_content/icons/{}.png",
            buffer
        ))
    }
}

impl Into<MiniSeason> for Option<String> {
    fn into(self) -> MiniSeason {
        match self.unwrap_or_default().as_str() {
            "/common/destiny2_content/icons/0dac2f181f0245cfc64494eccb7db9f7.png" | "" => {
                MiniSeason::RedWar
            }
            "/common/destiny2_content/icons/591f14483308beaad3278c3cd397e284.png" => {
                MiniSeason::CurseOfOsiris
            }
            "/common/destiny2_content/icons/e10338777d1d8633e073846e613a1c1f.png" => {
                MiniSeason::Warmind
            }
            "/common/destiny2_content/icons/0669efb55951e8bc9e99f3989eacc861.png" => {
                MiniSeason::Forsaken
            }
            "/common/destiny2_content/icons/bbddbe06ab72b61e708afc4fdbe15d95.png" => {
                MiniSeason::Forge
            }
            "/common/destiny2_content/icons/f9110e633634d112cff72a67159e3b12.png" => {
                MiniSeason::Drifter
            }
            "/common/destiny2_content/icons/785e5a64153cabd5637d68dcccb7fea6.png" => {
                MiniSeason::Opulence
            }
            "/common/destiny2_content/icons/8aae1c411642683d341b2c4f16a7130c.png" => {
                MiniSeason::Shadowkeep
            }
            "/common/destiny2_content/icons/d4141b2247cf999c73d3dc409f9d00f7.png" => {
                MiniSeason::Undying
            }
            "/common/destiny2_content/icons/ac012e11fa8bb032b923ad85e2ffb29c.png" => {
                MiniSeason::Dawn
            }
            "/common/destiny2_content/icons/3d335ddc3ec6668469aae60baad8548d.png" => {
                MiniSeason::Worthy
            }
            "/common/destiny2_content/icons/796813aa6cf8afe55aed4efc2f9c609b.png" => {
                MiniSeason::Arrivals
            }
            "/common/destiny2_content/icons/0aff1f4463f6f44e9863370ab1ce6983.png" => {
                MiniSeason::BeyondLight
            }
            "/common/destiny2_content/icons/2347cc2407b51e1debbac020bfcd0224.png" => {
                MiniSeason::Hunt
            }
            "/common/destiny2_content/icons/6a52f7cd9099990157c739a8260babea.png" => {
                MiniSeason::Chosen
            }
            "/common/destiny2_content/icons/b07d89064a1fc9a8e061f59b7c747fa5.png" => {
                MiniSeason::Splicer
            }
            "/common/destiny2_content/icons/4368a3e344977c5551407845ede830c2.png" => {
                MiniSeason::Lost
            }
            "/common/destiny2_content/icons/dd4dd93c5606998595d9e5a06d5bfc9c.png" => {
                MiniSeason::Anniversary
            }
            "/common/destiny2_content/icons/4fe83598190610f122497d22579a1fd9.png" => {
                MiniSeason::WitchQueen
            }
            "/common/destiny2_content/icons/b0406992c49c84bdc5febad94048dc01.png" => {
                MiniSeason::Risen
            }
            "/common/destiny2_content/icons/81edbfbf0bacf8e2117c00d1d6115f1b.png" => {
                MiniSeason::Haunted
            }
            "/common/destiny2_content/icons/f359d68324ae21522c299983ff1ef9f2.png" => {
                MiniSeason::Plunder
            }
            "/common/destiny2_content/icons/1a68ada4fb21371c5f2b7e2eae1ebce8.png" => {
                MiniSeason::Seraph
            }
            "/common/destiny2_content/icons/849de2c6bd5e9b8ced8abe8cca56d724.png" => {
                MiniSeason::LightFall
            }
            "/common/destiny2_content/icons/e6af18ae79b74e76dab327ec183f8228.png" => {
                MiniSeason::Defiance
            }
            _ => MiniSeason::Unknown,
        }
    }
}

impl From<MiniSeason> for Option<u8> {
    fn from(value: MiniSeason) -> Self {
        Some(match value {
            MiniSeason::RedWar => 1,
            MiniSeason::CurseOfOsiris => 2,
            MiniSeason::Warmind => 3,
            MiniSeason::Forsaken => 4,
            MiniSeason::Forge => 5,
            MiniSeason::Drifter => 6,
            MiniSeason::Opulence => 7,
            MiniSeason::Shadowkeep => 8,
            MiniSeason::Undying => 8,
            MiniSeason::Dawn => 9,
            MiniSeason::Worthy => 10,
            MiniSeason::Arrivals => 11,
            MiniSeason::BeyondLight => 12,
            MiniSeason::Hunt => 12,
            MiniSeason::Chosen => 13,
            MiniSeason::Splicer => 14,
            MiniSeason::Lost => 15,
            MiniSeason::Anniversary => 15,
            MiniSeason::WitchQueen => 16,
            MiniSeason::Risen => 16,
            MiniSeason::Haunted => 17,
            MiniSeason::Plunder => 18,
            MiniSeason::Seraph => 19,
            MiniSeason::LightFall => 20,
            MiniSeason::Defiance => 20,
            _ => {
                return None;
            }
        })
    }
}
