#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Heavy,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Regular
    }
}

impl FontWeight {
    pub fn from_u16(weight: u16) -> Self {
        match weight {
            100 => FontWeight::Thin,
            200 => FontWeight::ExtraLight,
            300 => FontWeight::Light,
            400 => FontWeight::Regular,
            500 => FontWeight::Medium,
            600 => FontWeight::SemiBold,
            700 => FontWeight::Bold,
            800 => FontWeight::ExtraBold,
            900 => FontWeight::Heavy,
            _ => FontWeight::Regular,
        }
    }
    pub fn as_u16(self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Heavy => 900,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

impl FontStyle {
    pub fn from_u8(style: u8) -> Self {
        match style {
            0 => FontStyle::Normal,
            1 => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            FontStyle::Normal => 0,
            FontStyle::Italic => 1,
        }
    }
}

/// A font face consisting of a typeface and other style properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Font {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub cached_face_id: Option<String>,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "rbxasset://fonts/families/SourceSansPro.json".to_owned(),
            weight: FontWeight::default(),
            style: FontStyle::default(),
            cached_face_id: None,
        }
    }
}

impl Font {
    pub fn new(family: &str, weight: FontWeight, style: FontStyle) -> Self {
        Self {
            family: family.to_owned(),
            weight,
            style,
            cached_face_id: None,
        }
    }
    pub fn regular(family: &str) -> Self {
        Self {
            family: family.to_owned(),
            ..Default::default()
        }
    }
    pub fn from_font_enum(value: u32) -> Option<Font> {
        return Some(match value {
            0 => Font::regular(&"rbxasset://fonts/families/LegacyArial.json"),
            1 => Font::regular(&"rbxasset://fonts/families/Arial.json"),
            2 => Font::new(
                &"rbxasset://fonts/families/Arial.json",
                FontWeight::Bold,
                FontStyle::Normal,
            ),
            3 => Font::regular(&"rbxasset://fonts/families/SourceSansPro.json"),
            4 => Font::new(
                &"rbxasset://fonts/families/SourceSansPro.json",
                FontWeight::Bold,
                FontStyle::Normal,
            ),
            16 => Font::new(
                &"rbxasset://fonts/families/SourceSansPro.json",
                FontWeight::SemiBold,
                FontStyle::Normal,
            ),
            5 => Font::new(
                &"rbxasset://fonts/families/SourceSansPro.json",
                FontWeight::Light,
                FontStyle::Normal,
            ),
            6 => Font::new(
                &"rbxasset://fonts/families/SourceSansPro.json",
                FontWeight::Regular,
                FontStyle::Italic,
            ),
            7 => Font::regular(&"rbxasset://fonts/families/AccanthisADFStd.json"),
            8 => Font::regular(&"rbxasset://fonts/families/Guru.json"),
            9 => Font::regular(&"rbxasset://fonts/families/ComicNeueAngular.json"),
            10 => Font::regular(&"rbxasset://fonts/families/Inconsolata.json"),
            11 => Font::regular(&"rbxasset://fonts/families/HighwayGothic.json"),
            12 => Font::regular(&"rbxasset://fonts/families/Zekton.json"),
            13 => Font::regular(&"rbxasset://fonts/families/PressStart2P.json"),
            14 => Font::regular(&"rbxasset://fonts/families/Balthazar.json"),
            15 => Font::regular(&"rbxasset://fonts/families/RomanAntique.json"),
            17 => Font::regular(&"rbxasset://fonts/families/GothamSSm.json"),
            18 => Font::new(
                &"rbxasset://fonts/families/GothamSSm.json",
                FontWeight::Medium,
                FontStyle::Normal,
            ),
            19 => Font::new(
                &"rbxasset://fonts/families/GothamSSm.json",
                FontWeight::Bold,
                FontStyle::Normal,
            ),
            20 => Font::new(
                &"rbxasset://fonts/families/GothamSSm.json",
                FontWeight::Heavy,
                FontStyle::Normal,
            ),
            21 => Font::regular(&"rbxasset://fonts/families/AmaticSC.json"),
            22 => Font::regular(&"rbxasset://fonts/families/Bangers.json"),
            23 => Font::regular(&"rbxasset://fonts/families/Creepster.json"),
            24 => Font::regular(&"rbxasset://fonts/families/DenkOne.json"),
            25 => Font::regular(&"rbxasset://fonts/families/Fondamento.json"),
            26 => Font::regular(&"rbxasset://fonts/families/FredokaOne.json"),
            27 => Font::regular(&"rbxasset://fonts/families/GrenzeGotisch.json"),
            28 => Font::regular(&"rbxasset://fonts/families/IndieFlower.json"),
            29 => Font::regular(&"rbxasset://fonts/families/JosefinSans.json"),
            30 => Font::regular(&"rbxasset://fonts/families/Jura.json"),
            31 => Font::regular(&"rbxasset://fonts/families/Kalam.json"),
            32 => Font::regular(&"rbxasset://fonts/families/LuckiestGuy.json"),
            33 => Font::regular(&"rbxasset://fonts/families/Merriweather.json"),
            34 => Font::regular(&"rbxasset://fonts/families/Michroma.json"),
            35 => Font::regular(&"rbxasset://fonts/families/Nunito.json"),
            36 => Font::regular(&"rbxasset://fonts/families/Oswald.json"),
            37 => Font::regular(&"rbxasset://fonts/families/PatrickHand.json"),
            38 => Font::regular(&"rbxasset://fonts/families/PermanentMarker.json"),
            39 => Font::regular(&"rbxasset://fonts/families/Roboto.json"),
            40 => Font::regular(&"rbxasset://fonts/families/RobotoCondensed.json"),
            41 => Font::regular(&"rbxasset://fonts/families/RobotoMono.json"),
            42 => Font::regular(&"rbxasset://fonts/families/Sarpanch.json"),
            43 => Font::regular(&"rbxasset://fonts/families/SpecialElite.json"),
            44 => Font::regular(&"rbxasset://fonts/families/TitilliumWeb.json"),
            45 => Font::regular(&"rbxasset://fonts/families/Ubuntu.json"),
            _ => return None,
        });
    }
}
