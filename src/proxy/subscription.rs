use anyhow::Result;

pub struct SubscriptionManager;

impl SubscriptionManager {
    pub fn new() -> Self {
        Self
    }

    pub fn add_subscription(&self, url: &str) -> Result<()> {
        println!("Adding subscription: {}", url);
        Ok(())
    }

    pub fn update_subscription(&self, name: &str) -> Result<()> {
        println!("Updating subscription: {}", name);
        Ok(())
    }
}
