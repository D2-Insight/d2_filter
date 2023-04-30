///This is used to produce links for icons
///Example: "https://www.bungie.net/common/destiny2_content/icons/0f584e8a13b2cc4cb60379b1777362e5.jpg"
///Doing 4 u64s instead of 2 u128s for wasm compatability
#[derive(serde::Deserialize, Clone, Copy)]
pub struct MiniIcon {
    icon_array: [u64; 2],
}
//Needs to be in format of /common/destiny2_content/icons/ ... .jpg
//This format is given from api/rustgie.
impl MiniIcon {
    fn new(url: &String) -> Option<MiniIcon> {
        if &url[0..=30] != "/common/destiny2_content/icons/" || &url[63..=66] != ".jpg" {
            return None;
        }
        let hex = &url[31..=62];
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

impl MiniIcon {
    fn get_url(val: MiniIcon) -> String {
        format!(
            "https://www.bungie.net/common/destiny2_content/icons/{:016x}{:016x}.jpg",
            val.icon_array[0], val.icon_array[1]
        )
    }
}

//Used to reduce storage per wep
#[derive(serde_repr::Deserialize_repr, Clone, Copy, num_enum::FromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum MiniWatermark {
    #[default]
    Unknown,
    Dawning,       //d91c738e8179465a165e35f7a249701b
    FOTL,          //215100c99216b9c0bd83b9daa50ace45
    GuardianGames, //97c65a76255ef764a9a98f24e50b859d
    Solstice,      //24ee3aca8624643ed02b684b2f7ef78b
    CrimsonDays,   //f80e5bb37ddd09573fd768af932075b4
    RedWar,
    CurseOfOsiris,
    Warmind,
    Forsaken, //Technically season of outlaw unfortnite
    Forge,
    Drifter,
    Opulence,
    Shadowkeep,
    Undying,
    Dawn,
    Worthy,
    Arrivals,
    BeyondLight,
    Hunt,
    Chosen,
    Splicer,
    Lost,
    Anniversary,
    WitchQueen,
    Risen,
    Haunted,
    Plunder,
    Seraph,
    LightFall,
    Defiance,
}

//Produced URL for icon from season
//This is used during runtime
impl From<MiniWatermark> for Option<String> {
    fn from(val: MiniWatermark) -> Self {
        let buffer = match val {
            MiniWatermark::Dawning => "d91c738e8179465a165e35f7a249701b",
            MiniWatermark::FOTL => "215100c99216b9c0bd83b9daa50ace45",
            MiniWatermark::GuardianGames => "97c65a76255ef764a9a98f24e50b859d",
            MiniWatermark::CrimsonDays => "f80e5bb37ddd09573fd768af932075b4",
            MiniWatermark::Solstice => "24ee3aca8624643ed02b684b2f7ef78b",
            MiniWatermark::RedWar => "0dac2f181f0245cfc64494eccb7db9f7",
            MiniWatermark::CurseOfOsiris => "591f14483308beaad3278c3cd397e284",
            MiniWatermark::Warmind => "e10338777d1d8633e073846e613a1c1f",
            MiniWatermark::Forsaken => "0669efb55951e8bc9e99f3989eacc861",
            MiniWatermark::Forge => "bbddbe06ab72b61e708afc4fdbe15d95",
            MiniWatermark::Drifter => "f9110e633634d112cff72a67159e3b12",
            MiniWatermark::Opulence => "785e5a64153cabd5637d68dcccb7fea6",
            MiniWatermark::Shadowkeep => "8aae1c411642683d341b2c4f16a7130c",
            MiniWatermark::Undying => "d4141b2247cf999c73d3dc409f9d00f7",
            MiniWatermark::Dawn => "ac012e11fa8bb032b923ad85e2ffb29c",
            MiniWatermark::Worthy => "3d335ddc3ec6668469aae60baad8548d",
            MiniWatermark::Arrivals => "796813aa6cf8afe55aed4efc2f9c609b",
            MiniWatermark::BeyondLight => "0aff1f4463f6f44e9863370ab1ce6983",
            MiniWatermark::Hunt => "2347cc2407b51e1debbac020bfcd0224",
            MiniWatermark::Chosen => "6a52f7cd9099990157c739a8260babea",
            MiniWatermark::Splicer => "b07d89064a1fc9a8e061f59b7c747fa5",
            MiniWatermark::Lost => "4368a3e344977c5551407845ede830c2",
            MiniWatermark::Anniversary => "dd4dd93c5606998595d9e5a06d5bfc9c",
            MiniWatermark::WitchQueen => "4fe83598190610f122497d22579a1fd9",
            MiniWatermark::Risen => "b0406992c49c84bdc5febad94048dc01",
            MiniWatermark::Haunted => "81edbfbf0bacf8e2117c00d1d6115f1b",
            MiniWatermark::Plunder => "f359d68324ae21522c299983ff1ef9f2",
            MiniWatermark::Seraph => "1a68ada4fb21371c5f2b7e2eae1ebce8",
            MiniWatermark::LightFall => "849de2c6bd5e9b8ced8abe8cca56d724",
            MiniWatermark::Defiance => "e6af18ae79b74e76dab327ec183f8228",
            MiniWatermark::Unknown => {
                return None;
            }
        };
        Some(format!(
            "https://www.bungie.net/common/destiny2_content/icons/{}.png",
            buffer
        ))
    }
}

//Expects String to be /common/destiny2_content/icons/ ... .png
//Comes from API / Rustgie
//Only for pregen
impl Into<MiniWatermark> for Option<String> {
    fn into(self) -> MiniWatermark {
        //Red war exotics come back from api as None
        let buffer = match self {
            Some(x) => x,
            None => {
                return MiniWatermark::RedWar;
            }
        }
        .clone();
        if &buffer[0..=30] != "/common/destiny2_content/icons/" || &buffer[63..=66] != ".png" {
            return MiniWatermark::Unknown;
        }
        //Extracts the needed portion
        match &buffer[31..=62] {
            "d91c738e8179465a165e35f7a249701b" => MiniWatermark::Dawning,
            "215100c99216b9c0bd83b9daa50ace45" => MiniWatermark::FOTL,
            "97c65a76255ef764a9a98f24e50b859d" => MiniWatermark::GuardianGames,
            "f80e5bb37ddd09573fd768af932075b4" => MiniWatermark::CrimsonDays,
            "24ee3aca8624643ed02b684b2f7ef78b" => MiniWatermark::Solstice,
            "0dac2f181f0245cfc64494eccb7db9f7" => MiniWatermark::RedWar,
            "591f14483308beaad3278c3cd397e284" => MiniWatermark::CurseOfOsiris,
            "e10338777d1d8633e073846e613a1c1f" => MiniWatermark::Warmind,
            "0669efb55951e8bc9e99f3989eacc861" => MiniWatermark::Forsaken,
            "bbddbe06ab72b61e708afc4fdbe15d95" => MiniWatermark::Forge,
            "f9110e633634d112cff72a67159e3b12" => MiniWatermark::Drifter,
            "785e5a64153cabd5637d68dcccb7fea6" => MiniWatermark::Opulence,
            "8aae1c411642683d341b2c4f16a7130c" => MiniWatermark::Shadowkeep,
            "d4141b2247cf999c73d3dc409f9d00f7" => MiniWatermark::Undying,
            "ac012e11fa8bb032b923ad85e2ffb29c" => MiniWatermark::Dawn,
            "3d335ddc3ec6668469aae60baad8548d" => MiniWatermark::Worthy,
            "796813aa6cf8afe55aed4efc2f9c609b" => MiniWatermark::Arrivals,
            "0aff1f4463f6f44e9863370ab1ce6983" => MiniWatermark::BeyondLight,
            "2347cc2407b51e1debbac020bfcd0224" => MiniWatermark::Hunt,
            "6a52f7cd9099990157c739a8260babea" => MiniWatermark::Chosen,
            "b07d89064a1fc9a8e061f59b7c747fa5" => MiniWatermark::Splicer,
            "4368a3e344977c5551407845ede830c2" => MiniWatermark::Lost,
            "dd4dd93c5606998595d9e5a06d5bfc9c" => MiniWatermark::Anniversary,
            "4fe83598190610f122497d22579a1fd9" => MiniWatermark::WitchQueen,
            "b0406992c49c84bdc5febad94048dc01" => MiniWatermark::Risen,
            "81edbfbf0bacf8e2117c00d1d6115f1b" => MiniWatermark::Haunted,
            "f359d68324ae21522c299983ff1ef9f2" => MiniWatermark::Plunder,
            "1a68ada4fb21371c5f2b7e2eae1ebce8" => MiniWatermark::Seraph,
            "849de2c6bd5e9b8ced8abe8cca56d724" => MiniWatermark::LightFall,
            "e6af18ae79b74e76dab327ec183f8228" => MiniWatermark::Defiance,
            _ => MiniWatermark::Unknown,
        }
    }
}

//Gives season number from enum
impl MiniWatermark {
    fn get_season(&self) -> Option<u8> {
        Some(match self {
            Self::RedWar => 1,
            Self::CurseOfOsiris => 2,
            Self::Warmind => 3,
            Self::Forsaken => 4,
            Self::Forge => 5,
            Self::Drifter => 6,
            Self::Opulence => 7,
            Self::Shadowkeep => 8,
            Self::Undying => 8,
            Self::Dawn => 9,
            Self::Worthy => 10,
            Self::Arrivals => 11,
            Self::BeyondLight => 12,
            Self::Hunt => 12,
            Self::Chosen => 13,
            Self::Splicer => 14,
            Self::Lost => 15,
            Self::Anniversary => 15,
            Self::WitchQueen => 16,
            Self::Risen => 16,
            Self::Haunted => 17,
            Self::Plunder => 18,
            Self::Seraph => 19,
            Self::LightFall => 20,
            Self::Defiance => 20,
            _ => {
                return None;
            }
        })
    }
}
