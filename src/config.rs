//! Myex2 Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

//use config::{Config, File, FileFormat, FileSourceFile, Map, Value};
use directories::BaseDirs;
use ron;
use serde::{Deserialize, Serialize};
use url::Url;

/// Myex2 Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Myex2Config {
    /// An example configuration section
    pub proxy: Option<Option<Url>>,
    /// An example configuration section
    pub example: ExampleSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on Myex2Config instead.
impl Default for Myex2Config {
    fn default() -> Self {
        let cf = "__github-helper.ron"; //: &str = "github-helper.toml";
        let cf = BaseDirs::new().and_then(|dirs| Some(dirs.config_dir().join(cf)));
        //println!("~/.config/...: {cf:?}");
        if let Some(f) = cf && let Ok(cf) = std::fs::OpenOptions::new().read(true).open(&f) {
            return ron::de::from_reader::<_, Myex2Config>(cf).unwrap();
        }
        // if let Some(cf) = cf.filter(|f| f.exists()) {
        //     // let config = Config::builder()
        //     //     .add_source(File::from(cf).format(FileFormat::Ron))
        //     //     .build()
        //     //     .unwrap();
        //     // config.try_deserialize::<Myex2Config>().unwrap() //dbg!()
        // }
        Self {
            proxy: None,
            example: ExampleSection::default(),
        }
    }
}

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExampleSection {
    /// Example configuration value
    pub recipient: String,
}

impl Default for ExampleSection {
    fn default() -> Self {
        Self {
            recipient: "=world=".to_owned(),
        }
    }
}
