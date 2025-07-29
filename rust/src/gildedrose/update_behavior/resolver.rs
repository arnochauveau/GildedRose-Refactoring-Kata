use crate::gildedrose::update_behavior::{
    aged_brie_behavior::AgedBrieBehavior,
    backstage_pass_behavior::BackstagePassBehavior,
    legacy_behavior::LegacyBehavior, UpdateBehavior,
};

use super::Item;

pub fn resolve_update_behavior(item: &Item) -> Box<dyn UpdateBehavior> {
    match item.name.as_str() {
        "Aged Brie" => Box::new(AgedBrieBehavior {}),
        "Backstage passes to a TAFKAL80ETC concert" => {
            Box::new(BackstagePassBehavior {})
        }
        _ => Box::new(LegacyBehavior {}),
    }
}
