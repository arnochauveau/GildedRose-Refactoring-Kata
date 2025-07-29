use crate::gildedrose::update_behavior::{
    aged_brie_behavior::AGED_BRIE_BEHAVIOR,
    backstage_pass_behavior::BACKSTAGE_PASS_BEHAVIOR,
    conjured_item_behavior::CONJURED_ITEM_BEHAVIOR,
    default_behavior::DEFAULT_BEHAVIOR,
    legendary_item_behavior::LEGENDARY_ITEM_BEHAVIOR, UpdateBehavior,
};

use super::Item;

pub fn resolve_update_behavior(item: &Item) -> &'static dyn UpdateBehavior {
    match item.name.as_str() {
        "Aged Brie" => &AGED_BRIE_BEHAVIOR,
        "Backstage passes to a TAFKAL80ETC concert" => &BACKSTAGE_PASS_BEHAVIOR,
        "Sulfuras, Hand of Ragnaros" => &LEGENDARY_ITEM_BEHAVIOR,
        "Conjured Mana Cake" => &CONJURED_ITEM_BEHAVIOR,

        _ => &DEFAULT_BEHAVIOR,
    }
}
