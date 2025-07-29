use std::cmp::max;

use crate::gildedrose::{item::Item, update_behavior::UpdateBehavior};

pub struct DefaultBehavior;

impl UpdateBehavior for DefaultBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        let mut owned_item = item.clone();

        let amount_to_decrease = match owned_item.sell_in {
            ..=0 => 2,
            1.. => 1,
        };

        owned_item.quality = max(owned_item.quality - amount_to_decrease, 0);

        owned_item.sell_in = owned_item.sell_in - 1;

        owned_item
    }
}

#[cfg(test)]
mod default_behavior_tests {
    use super::*;

    #[test]
    pub fn quality_and_sellin_decrease_for_standard_items() {
        let behavior = DefaultBehavior {};
        let result = behavior.update(&Item::new("standard item", 4, 4));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 3);
    }

    #[test]
    pub fn double_quality_degradation_sellin_equal_to_zero() {
        let behavior = DefaultBehavior {};
        let result = behavior.update(&Item::new("standard item", 0, 4));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 2);
    }

    #[test]
    pub fn double_quality_degradation_sellin_lower_than_zero() {
        let behavior = DefaultBehavior {};
        let result = behavior.update(&Item::new("standard item", -1, 4));

        assert_eq!(result.sell_in, -2);
        assert_eq!(result.quality, 2);
    }

    #[test]
    fn quality_cant_degrade_below_zero() {
        let behavior = DefaultBehavior {};
        let result = behavior.update(&Item::new("standard item", 2, 0));

        assert_eq!(result.sell_in, 1);
        assert_eq!(result.quality, 0);
    }
}
