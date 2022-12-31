use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub command_prefix: String,
    pub inspect_urls: bool,
    pub inspect_rejects: Vec<String>
}
// impl Default for Config {
//     fn default() -> Self { Self { inspect_urls: false } }
// }
