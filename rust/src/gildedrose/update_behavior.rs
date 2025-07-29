pub mod resolver;

pub mod aged_brie_behavior;
pub mod backstage_pass_behavior;
pub mod conjured_item_behavior;
pub mod default_behavior;
pub mod legendary_item_behavior;

use super::Item;

pub trait UpdateBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item;
}
