pub mod config;
pub mod item;
pub mod update_behavior;

use item::Item;

use crate::gildedrose::update_behavior::resolver::resolve_update_behavior;

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        self.items = self
            .items
            .iter()
            .map(|item| resolve_update_behavior(item).update(item))
            .collect();
    }
}
