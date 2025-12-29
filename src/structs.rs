use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
        pub font: Font,
        pub icons: Icons,
        pub colors: Colors,
        pub window: Window,
        pub misc: Misc,
        pub apps: Vec<App>,
}

impl Config {
        pub fn new(temp: ConfigTemp) -> Self {
                // Helper closures
                let str_or_default = |opt: Option<String>, default: &str| -> String {
                        opt.unwrap_or_else(|| default.to_string())
                };
                let char_or_default =
                |opt: Option<char>, default: char| -> char { opt.unwrap_or_else(|| default) };
                let bool_or_default =
                |opt: Option<bool>, default: bool| -> bool { opt.unwrap_or_else(|| default) };
                let u32_or_default = |opt: Option<u32>, default: u32| -> u32 { opt.unwrap_or(default) };
                let f32_or_default = |opt: Option<f32>, default: f32| -> f32 { opt.unwrap_or(default) };
                let u8_or_default = |opt: Option<u8>, default: u8| -> u8 { opt.unwrap_or(default) };
                let u64_or_default = |opt: Option<u64>, default: u64| -> u64 { opt.unwrap_or(default) };

                Config {
                        font: Font {
                                path: str_or_default(temp.font.path, ""),
                                size: f32_or_default(temp.font.size, 14.0),
                        },
                        icons: Icons {
                                entry: char_or_default(temp.icons.entry, ''),
                                entry_icon: bool_or_default(temp.icons.entry_icon, false),
                        },
                        colors: Colors {
                                text: str_or_default(temp.colors.text, "#ffffff"),
                                text_aux: str_or_default(temp.colors.text_aux, "#888888"),
                                accent: str_or_default(temp.colors.accent, "#333333"),
                                background: str_or_default(temp.colors.background, "#1a1a1a"),
                        },
                        window: Window {
                                width: u32_or_default(temp.window.width, 400),
                                height: u32_or_default(temp.window.height, 600),
                                elem_cnr_rad: u8_or_default(temp.window.elem_cnr_rad, 10),
                                row_height: f32_or_default(temp.window.row_height, 30.0),
                        },
                        misc: Misc {
                                search_hint: str_or_default(temp.misc.search_hint, "Search..."),
                                timeout: u64_or_default(temp.misc.timeout, 20),
                        },
                        apps: temp
                                .apps
                                .into_iter()
                                .map(|a| App {
                                        name: str_or_default(a.name, "x"),
                                        icon: char_or_default(a.icon, 'x'),
                                        command: str_or_default(a.command, "x"),
                                })
                                .collect(),
                }
        }
}

#[derive(Deserialize, Debug)]
pub struct ConfigTemp {
        #[serde(rename = "Font")]
        pub font: FontTemp,
        #[serde(rename = "Icons")]
        pub icons: IconsTemp,
        #[serde(rename = "Colors")]
        pub colors: ColorsTemp,
        #[serde(rename = "Window")]
        pub window: WindowTemp,
        #[serde(rename = "Misc")]
        pub misc: MiscTemp,
        #[serde(rename = "App")]
        pub apps: Vec<AppTemp>,
}

impl ConfigTemp {
        pub fn get_font(&self) -> FontTemp {
                FontTemp {
                        path: Some(self.font.path.clone().unwrap_or("Default Font".to_string())),
                        size: Some(self.font.size.clone().unwrap_or(14.0)),
                }
        }

        pub fn get_icons(&self) -> IconsTemp {
                IconsTemp {
                        entry: Some(self.icons.entry.clone().unwrap_or('')),
                        entry_icon: Some(self.icons.entry_icon.clone().unwrap_or(false)),
                }
        }

        pub fn get_window(&self) -> WindowTemp {
                WindowTemp {
                        width: Some(self.window.width.unwrap_or(400)),
                        height: Some(self.window.height.unwrap_or(600)),
                        elem_cnr_rad: Some(self.window.elem_cnr_rad.unwrap_or(10)),
                        row_height: Some(self.window.row_height.unwrap_or(30.0)),
                }
        }

        pub fn get_misc(&self) -> MiscTemp {
                MiscTemp {
                        search_hint: Some(
                                self.misc
                                        .search_hint
                                        .clone()
                                        .unwrap_or("Search...".to_string()),
                        ),
                        timeout: Some(self.misc.timeout.unwrap_or(20)),
                }
        }
        pub fn get_colors(&self) -> ColorsTemp {
                ColorsTemp {
                        text: Some(self.colors.text.clone().unwrap_or("#ffffff".to_string())),
                        text_aux: Some(self.colors.text.clone().unwrap_or("#888888".to_string())),
                        accent: Some(self.colors.accent.clone().unwrap_or("#333333".to_string())),
                        background: Some(
                                self.colors
                                        .background
                                        .clone()
                                        .unwrap_or("#1a1a1a".to_string()),
                        ),
                }
        }

        pub fn get_apps(&self) -> Vec<AppTemp> {
                self.apps
                        .iter()
                        .map(|a| AppTemp {
                                name: Some(a.name.clone().unwrap_or("x".to_string())),
                                icon: Some(a.icon.clone().unwrap_or('x')),
                                command: Some(a.command.clone().unwrap_or("x".to_string())),
                        })
                        .collect()
        }
}

// ==== font ====
#[derive(Clone, Deserialize, Debug)]
pub struct Font {
        pub path: String,
        pub size: f32,
}
#[derive(Deserialize, Debug)]
pub struct FontTemp {
        pub path: Option<String>,
        pub size: Option<f32>,
}

// ==== icons ====
#[derive(Deserialize, Debug)]
pub struct Icons {
        pub entry: char,
        pub entry_icon: bool,
}
#[derive(Deserialize, Debug)]
pub struct IconsTemp {
        pub entry: Option<char>,
        pub entry_icon: Option<bool>,
}

// ==== colors ====
#[derive(Deserialize, Debug)]
pub struct Colors {
        pub text: String,
        pub text_aux: String,
        pub accent: String,
        pub background: String,
}
#[derive(Deserialize, Debug)]
pub struct ColorsTemp {
        pub text: Option<String>,
        pub text_aux: Option<String>,
        pub accent: Option<String>,
        pub background: Option<String>,
}

// ==== window ====
#[derive(Deserialize, Debug)]
pub struct Window {
        pub width: u32,
        pub height: u32,
        pub elem_cnr_rad: u8,
        pub row_height: f32,
}
#[derive(Deserialize, Debug)]
pub struct WindowTemp {
        pub width: Option<u32>,
        pub height: Option<u32>,
        pub elem_cnr_rad: Option<u8>,
        pub row_height: Option<f32>,
}

// ==== misc ====
#[derive(Deserialize, Debug)]
pub struct Misc {
        pub search_hint: String,
        pub timeout: u64,
}
#[derive(Deserialize, Debug)]
pub struct MiscTemp {
        pub search_hint: Option<String>,
        pub timeout: Option<u64>,
}
// ==== app ====
#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub struct App {
        pub name: String,
        pub icon: char,
        pub command: String,
}
#[derive(Clone, Deserialize, Debug)]
pub struct AppTemp {
        pub name: Option<String>,
        pub icon: Option<char>,
        pub command: Option<String>,
}
