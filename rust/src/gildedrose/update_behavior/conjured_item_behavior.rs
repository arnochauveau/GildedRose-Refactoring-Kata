use std::cmp::max;

use crate::gildedrose::{item::Item, update_behavior::UpdateBehavior};

pub static CONJURED_ITEM_BEHAVIOR: ConjuredItemBehavior =
    ConjuredItemBehavior {};
pub struct ConjuredItemBehavior;
impl UpdateBehavior for ConjuredItemBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        let mut owned_item = item.clone();

        let amount_to_decrease = match owned_item.sell_in {
            ..=0 => 4,
            1.. => 2,
        };

        owned_item.quality = max(owned_item.quality - amount_to_decrease, 0);
        owned_item.sell_in -= 1;

        owned_item
    }
}

#[cfg(test)]
mod conjured_item_tests {
    use super::*;

    #[test]
    fn should_decrease_with_2_with_sellin_above_0() {
        let result = CONJURED_ITEM_BEHAVIOR.update(&Item::new(
            "Conjured Mana Cake",
            4,
            30,
        ));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 28);
    }

    #[test]
    fn should_decrease_with_4_with_sellin_0() {
        let result = CONJURED_ITEM_BEHAVIOR.update(&Item::new(
            "Conjured Mana Cake",
            0,
            30,
        ));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 26);
    }

    #[test]
    fn should_decrease_with_4_with_sellin_below_0() {
        let result = CONJURED_ITEM_BEHAVIOR.update(&Item::new(
            "Conjured Mana Cake",
            -1,
            30,
        ));

        assert_eq!(result.sell_in, -2);
        assert_eq!(result.quality, 26);
    }

    #[test]
    fn should_not_go_under_0_with_sellin_above_0() {
        let result = CONJURED_ITEM_BEHAVIOR.update(&Item::new(
            "Conjured Mana Cake",
            4,
            1,
        ));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 0);
    }

    #[test]
    fn should_not_go_under_0_with_sellin_0() {
        let result = CONJURED_ITEM_BEHAVIOR.update(&Item::new(
            "Conjured Mana Cake",
            0,
            3,
        ));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 0);
    }

    #[test]
    fn should_not_go_under_0_with_sellin_below_0() {
        let result = CONJURED_ITEM_BEHAVIOR.update(&Item::new(
            "Conjured Mana Cake",
            -1,
            3,
        ));

        assert_eq!(result.sell_in, -2);
        assert_eq!(result.quality, 0);
    }
}
