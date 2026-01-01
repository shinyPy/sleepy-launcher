use relm4::prelude::*;
use adw::prelude::*;
use gtk::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::zzz::config::Config;
use anime_launcher_sdk::zzz::zzmi;

use std::path::PathBuf;

use crate::*;

use super::EnhancementsAppMsg;

pub struct ModsSettingsPage {
    enabled: bool,
    mods_folder: PathBuf,
    mods_folder_display: String,
}

#[derive(Debug, Clone)]
pub enum ModsSettingsPageMsg {
    SetEnabled(bool),
    SelectModsFolder,
    ModsFolderSelected(PathBuf),
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ModsSettingsPage {
    type Init = ();
    type Input = ModsSettingsPageMsg;
    type Output = EnhancementsAppMsg;

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: &tr!("mods")
                    }
                },

                adw::PreferencesPage {
                    set_title: &tr!("mods"),
                    set_icon_name: Some("applications-games-symbolic"),

                    add = &adw::PreferencesGroup {
                        set_title: &tr!("zzmi-mods"),
                        set_description: Some("ZZMI components are automatically downloaded"),

                        adw::ActionRow {
                            set_title: &tr!("enable-mods"),
                            set_subtitle: "Auto-downloads ZZMI + 3DMigoto on first launch",

                            add_suffix = &gtk::Switch {
                                set_valign: gtk::Align::Center,
                                #[watch]
                                set_active: model.enabled,

                                connect_state_notify[sender] => move |switch| {
                                    sender.input(ModsSettingsPageMsg::SetEnabled(switch.is_active()));
                                }
                            }
                        }
                    },

                    add = &adw::PreferencesGroup {
                        set_title: "Mods Location",
                        set_description: Some("Where your character mods are stored"),

                        adw::ActionRow {
                            set_title: "Mods Folder",
                            #[watch]
                            set_subtitle: if model.mods_folder_display.is_empty() {
                                "Using default (~/.local/share/sleepy-launcher/zzmi/Mods)"
                            } else {
                                &model.mods_folder_display
                            },

                            set_activatable: true,

                            connect_activated[sender] => move |_| {
                                sender.input(ModsSettingsPageMsg::SelectModsFolder);
                            },

                            add_suffix = &gtk::Image {
                                set_icon_name: Some("folder-open-symbolic")
                            }
                        }
                    },

                    add = &adw::PreferencesGroup {
                        set_title: "Info",

                        adw::ActionRow {
                            set_title: "Auto-Download",
                            set_subtitle: "XXMI-Libs and ZZMI-Package are downloaded automatically"
                        },

                        adw::ActionRow {
                            set_title: "Mod Installation",
                            set_subtitle: "Place character mod folders in your Mods folder"
                        }
                    }
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing mods settings");

        let mods_folder = CONFIG.game.mods.mods_folder.clone();
        let mods_folder_display = mods_folder.to_string_lossy().to_string();

        let model = Self {
            enabled: CONFIG.game.mods.enabled,
            mods_folder,
            mods_folder_display,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            ModsSettingsPageMsg::SetEnabled(enabled) => {
                if let Ok(mut config) = Config::get() {
                    config.game.mods.enabled = enabled;
                    Config::update(config);
                }
                self.enabled = enabled;
            }

            ModsSettingsPageMsg::SelectModsFolder => {
                // Get default directory for file dialog
                let start_dir = if self.mods_folder.as_os_str().is_empty() {
                    zzmi::get_default_mods_dir().ok()
                } else {
                    Some(self.mods_folder.clone())
                };

                let mut dialog = rfd::AsyncFileDialog::new()
                    .set_title("Select Mods Folder");
                
                if let Some(dir) = start_dir {
                    dialog = dialog.set_directory(&dir);
                }

                if let Some(folder) = dialog.pick_folder().await {
                    let path = folder.path().to_path_buf();
                    sender.input(ModsSettingsPageMsg::ModsFolderSelected(path));
                }
            }

            ModsSettingsPageMsg::ModsFolderSelected(path) => {
                self.mods_folder_display = path.to_string_lossy().to_string();
                self.mods_folder = path.clone();

                if let Ok(mut config) = Config::get() {
                    config.game.mods.mods_folder = path;
                    Config::update(config);
                }
            }
        }
    }
}
