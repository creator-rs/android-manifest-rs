#[macro_use]
extern crate yaserde_derive;

mod action;
mod activity;
mod activity_alias;
mod application;
mod attribute_list;
mod category;
mod compatible_screens;
mod data;
mod grant_uri_permission;
mod instrumentation;
mod intent_filter;
mod layout;
mod manifest;
mod meta_data;
mod path_permission;
mod permission;
mod permission_group;
mod permission_tree;
mod provider;
mod receiver;
mod resources;
mod service;
mod supports_gl_texture;
mod supports_screens;
mod ui_options;
mod uses_configuration;
mod uses_feature;
mod uses_library;
mod uses_permission;
mod uses_permission_sdk_23;
mod uses_sdk;

pub use action::*;
pub use activity::*;
pub use activity_alias::*;
pub use application::*;
pub use category::*;
pub use compatible_screens::*;
pub use data::*;
pub use grant_uri_permission::*;
pub use instrumentation::*;
pub use intent_filter::*;
pub use layout::*;
pub use manifest::*;
pub use meta_data::*;
pub use path_permission::*;
pub use permission::*;
pub use permission_group::*;
pub use permission_tree::*;
pub use provider::*;
pub use receiver::*;
pub use resources::*;
pub use service::*;
pub use supports_gl_texture::*;
pub use supports_screens::*;
pub use ui_options::*;
pub use uses_configuration::*;
pub use uses_feature::*;
pub use uses_library::*;
pub use uses_permission::*;
pub use uses_permission_sdk_23::*;
pub use uses_sdk::*;

// #[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
// #[yaserde(
//     rename = "manifest",
//     namespace = "html: http://schemas.android.com/apk/res/android"
// )]
// pub struct TestManifest {
//     pub application: TestApplication,
//     #[serde(skip_serializing_if = "Vec::is_empty", default)]
//     #[yaserde(rename = "compatible-screens")]
//     pub compatible_screens: Vec<TestCompatibleScreens>,
// }

// #[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Default)]
// pub struct TestApplication {
//     #[yaserde(attribute, prefix = "android", rename = "allowTaskReparenting")]
//     pub allow_task_reparenting: Option<bool>,
//     #[yaserde(attribute, prefix = "android", rename = "allowBackup")]
//     pub allow_backup: Option<bool>,
// }

// #[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
// pub struct TestCompatibleScreens {
//     pub screen: Vec<TestScreen>,
// }

// #[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
// #[yaserde(prefix = "android")]
// pub struct TestScreen {
//     #[yaserde(attribute, prefix = "android", rename = "screenSize")]
//     pub screen_size: TestScreenSize,
//     #[yaserde(attribute, prefix = "android", rename = "screenDensity")]
//     pub screen_density: TestScreenDensity,
// }

// #[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub enum TestScreenSize {
//     Small,
//     Normal,
//     Large,
//     Xlarge,
// }

// impl Default for TestScreenSize {
//     fn default() -> TestScreenSize {
//         TestScreenSize::Small
//     }
// }

// #[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub enum TestScreenDensity {
//     /// "ldpi" (approximately 120 dpi)
//     Ldpi,
//     /// "mdpi" (approximately 160 dpi)
//     Mdpi,
//     /// "hdpi" (approximately 240 dpi)
//     Hdpi,
//     /// "xhdpi" (approximately 320 dpi)
//     Xhdpi,
//     /// "xxhdpi" (approximately 480 dpi)
//     Xxhdpi,
//     /// "xxxhdpi" (approximately 560-640 dpi)
//     Xxxhdpi,
// }

// impl Default for TestScreenDensity {
//     fn default() -> TestScreenDensity {
//         TestScreenDensity::Xhdpi
//     }
// }
