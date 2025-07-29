use std::cmp::min;

use crate::gildedrose::{
    config::CONFIG, item::Item, update_behavior::UpdateBehavior,
};

pub static AGED_BRIE_BEHAVIOR: AgedBrieBehavior = AgedBrieBehavior {};
pub struct AgedBrieBehavior;

impl UpdateBehavior for AgedBrieBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        let mut owned_item = item.clone();

        let increase_amount = match owned_item.sell_in {
            ..=0 => 2,
            1.. => 1,
        };

        owned_item.quality =
            min(owned_item.quality + increase_amount, CONFIG.max_quality);
        owned_item.sell_in -= 1;

        owned_item
    }
}

#[cfg(test)]
mod aged_brie_tests {
    use super::*;

    #[test]
    fn increases_in_quality() {
        let result = AGED_BRIE_BEHAVIOR.update(&Item::new("Aged Brie", 2, 2));

        assert_eq!(result.sell_in, 1);
        assert_eq!(result.quality, 3);
    }

    #[test]
    fn double_increases_if_quality_sellin_zero() {
        let result = AGED_BRIE_BEHAVIOR.update(&Item::new("Aged Brie", 0, 2));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 4);
    }

    #[test]
    fn double_increases_if_quality_sellin_lower_than_zero() {
        let result = AGED_BRIE_BEHAVIOR.update(&Item::new("Aged Brie", -1, 2));

        assert_eq!(result.sell_in, -2);
        assert_eq!(result.quality, 4);
    }

    #[test]
    fn quality_doesnt_go_above_50() {
        let result = AGED_BRIE_BEHAVIOR.update(&Item::new("Aged Brie", 4, 50));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 50);
    }
}
