use crate::gildedrose::{item::Item, update_behavior::UpdateBehavior};

pub struct LegacyBehavior;

impl UpdateBehavior for LegacyBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        let mut owned_item = item.clone();
        if owned_item.quality > 0 {
            if owned_item.name != "Sulfuras, Hand of Ragnaros" {
                owned_item.quality = owned_item.quality - 1;
            }
        }

        if owned_item.name != "Sulfuras, Hand of Ragnaros" {
            owned_item.sell_in = owned_item.sell_in - 1;
        }

        if owned_item.sell_in < 0 {
            if owned_item.quality > 0 {
                if owned_item.name != "Sulfuras, Hand of Ragnaros" {
                    owned_item.quality = owned_item.quality - 1;
                }
            }
        }
        owned_item
    }
}

#[cfg(test)]
mod legacy_behavior_tests {
    use super::*;

    #[test]
    pub fn quality_and_sellin_decrease_for_standard_items() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("standard item", 4, 4));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 3);
    }

    #[test]
    pub fn double_quality_degradation_sellin_equal_to_zero() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("standard item", 0, 4));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 2);
    }

    #[test]
    pub fn double_quality_degradation_sellin_lower_than_zero() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("standard item", -1, 4));

        assert_eq!(result.sell_in, -2);
        assert_eq!(result.quality, 2);
    }

    #[test]
    fn quality_cant_degrade_below_zero() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("standard item", 2, 0));

        assert_eq!(result.sell_in, 1);
        assert_eq!(result.quality, 0);
    }

    #[test]
    fn legendary_items_stay_80_quality_and_dont_change_sellin() {
        let behavior = LegacyBehavior {};
        let result =
            behavior.update(&Item::new("Sulfuras, Hand of Ragnaros", 4, 80));

        assert_eq!(result.sell_in, 4);
        assert_eq!(result.quality, 80);
    }
}
