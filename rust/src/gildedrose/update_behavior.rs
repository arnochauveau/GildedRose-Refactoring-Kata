pub mod resolver;

pub mod aged_brie_behavior;
pub mod legacy_behavior;

use super::Item;

pub trait UpdateBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item;
}
