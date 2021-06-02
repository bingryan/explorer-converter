use std::any::Any;
use crate::config::Settings;

mod redis;


pub trait Filter: Any + Send + Sync {
    /// filter name
    fn name(&self) -> &'static str;

    /// load settings
    fn build(&self,settings: &Settings) -> Box<dyn Filter>;

    /// fingerprint
    fn fingerprint(self) -> bool;

    /// clear filter collections
    fn clear(self) -> bool;
}