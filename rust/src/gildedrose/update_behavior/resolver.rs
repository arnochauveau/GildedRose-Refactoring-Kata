use crate::gildedrose::update_behavior::{
    aged_brie_behavior::AgedBrieBehavior, legacy_behavior::LegacyBehavior,
    UpdateBehavior,
};

use super::Item;

pub fn resolve_update_behavior(item: &Item) -> Box<dyn UpdateBehavior> {
    match item.name.as_str() {
        "Aged Brie" => Box::new(AgedBrieBehavior {}),
        _ => Box::new(LegacyBehavior {}),
    }
}
