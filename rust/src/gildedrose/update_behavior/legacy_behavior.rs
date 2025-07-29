use crate::gildedrose::{item::Item, update_behavior::UpdateBehavior};

pub struct LegacyBehavior;

impl UpdateBehavior for LegacyBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        let mut owned_item = item.clone();
        if owned_item.name != "Aged Brie"
            && owned_item.name != "Backstage passes to a TAFKAL80ETC concert"
        {
            if owned_item.quality > 0 {
                if owned_item.name != "Sulfuras, Hand of Ragnaros" {
                    owned_item.quality = owned_item.quality - 1;
                }
            }
        } else {
            if owned_item.quality < 50 {
                owned_item.quality = owned_item.quality + 1;

                if owned_item.name
                    == "Backstage passes to a TAFKAL80ETC concert"
                {
                    if owned_item.sell_in < 11 {
                        if owned_item.quality < 50 {
                            owned_item.quality = owned_item.quality + 1;
                        }
                    }

                    if owned_item.sell_in < 6 {
                        if owned_item.quality < 50 {
                            owned_item.quality = owned_item.quality + 1;
                        }
                    }
                }
            }
        }

        if owned_item.name != "Sulfuras, Hand of Ragnaros" {
            owned_item.sell_in = owned_item.sell_in - 1;
        }

        if owned_item.sell_in < 0 {
            if owned_item.name != "Aged Brie" {
                if owned_item.name
                    != "Backstage passes to a TAFKAL80ETC concert"
                {
                    if owned_item.quality > 0 {
                        if owned_item.name != "Sulfuras, Hand of Ragnaros" {
                            owned_item.quality = owned_item.quality - 1;
                        }
                    }
                } else {
                    owned_item.quality =
                        owned_item.quality - owned_item.quality;
                }
            } else {
                if owned_item.quality < 50 {
                    owned_item.quality = owned_item.quality + 1;
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
    fn aged_brie_increases_in_quality() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("Aged Brie", 2, 2));

        assert_eq!(result.sell_in, 1);
        assert_eq!(result.quality, 3);
    }

    #[test]
    fn aged_brie_double_increases_in_quality_sellin_zero() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("Aged Brie", 0, 2));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 4);
    }

    #[test]
    fn aged_brie_double_increases_in_quality_sellin_lower_than_zero() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("Aged Brie", -1, 2));

        assert_eq!(result.sell_in, -2);
        assert_eq!(result.quality, 4);
    }

    #[test]
    fn aged_brie_quality_doesnt_go_above_50() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new("Aged Brie", 4, 50));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 50);
    }

    #[test]
    fn legendary_items_stay_80_quality_and_dont_change_sellin() {
        let behavior = LegacyBehavior {};
        let result =
            behavior.update(&Item::new("Sulfuras, Hand of Ragnaros", 4, 80));

        assert_eq!(result.sell_in, 4);
        assert_eq!(result.quality, 80);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_1_when_sellin_is_larger_than_10(
    ) {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            11,
            0,
        ));

        assert_eq!(result.sell_in, 10);
        assert_eq!(result.quality, 1);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_2_when_sellin_is_10() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            10,
            0,
        ));

        assert_eq!(result.sell_in, 9);
        assert_eq!(result.quality, 2);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_2_when_sellin_is_less_than_10_but_larger_than_5(
    ) {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            7,
            0,
        ));

        assert_eq!(result.sell_in, 6);
        assert_eq!(result.quality, 2);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_3_when_sellin_is_5() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            5,
            0,
        ));

        assert_eq!(result.sell_in, 4);
        assert_eq!(result.quality, 3);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_2_when_sellin_is_less_than_5_but_larger_than_0(
    ) {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            3,
            0,
        ));

        assert_eq!(result.sell_in, 2);
        assert_eq!(result.quality, 3);
    }

    #[test]
    fn backstage_pass_quality_should_be_0_when_sellin_is_0() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            0,
            8,
        ));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 0);
    }

    #[test]
    fn backstage_pass_quality_doesnt_go_above_50() {
        let behavior = LegacyBehavior {};
        let result = behavior.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            4,
            50,
        ));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 50);
    }
}
