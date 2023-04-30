use std::fmt::format;

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
            .strip_prefix("https://www.bungie.net/common/destiny2_content/icons/")
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

impl Into<String> for MiniIcon {
    fn into(self) -> String {
        format!(
            "https://www.bungie.net/common/destiny2_content/icons/{:016x}{:016x}.jpg",
            self.icon_array[0], self.icon_array[1]
        )
    }
}
