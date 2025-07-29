use crate::gildedrose::update_behavior::{
    aged_brie_behavior::AgedBrieBehavior,
    backstage_pass_behavior::BackstagePassBehavior,
    default_behavior::DefaultBehavior,
    legendary_item_behavior::LegendaryItemBehavior, UpdateBehavior,
};

use super::Item;

pub fn resolve_update_behavior(item: &Item) -> Box<dyn UpdateBehavior> {
    match item.name.as_str() {
        "Aged Brie" => Box::new(AgedBrieBehavior {}),
        "Backstage passes to a TAFKAL80ETC concert" => {
            Box::new(BackstagePassBehavior {})
        }
        "Sulfuras, Hand of Ragnaros" => Box::new(LegendaryItemBehavior {}),

        _ => Box::new(DefaultBehavior {}),
    }
}
