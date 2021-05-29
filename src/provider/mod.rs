mod mcloud;

use crate::Result;
use mcloud::MCloud;

pub const PROVIDERS: [&str;1] = [
    mcloud::PROVIDER
];

pub const DEFAULT_PROVIDER: &str = mcloud::PROVIDER;

pub fn get_provider(provider: &str) -> Option<Box<dyn DomainServiceProvider>> {
    match provider {
        mcloud::PROVIDER => Some(Box::new(MCloud)),
        _ => None
    }
}

pub trait DomainServiceProvider {
    fn signup(&self) -> Result<()>;
    fn check_domain(&self, domain: &str) -> Result<bool>;
}
