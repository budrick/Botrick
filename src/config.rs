use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub inspect_urls: bool,
}
// impl Default for Config {
//     fn default() -> Self { Self { inspect_urls: false } }
// }