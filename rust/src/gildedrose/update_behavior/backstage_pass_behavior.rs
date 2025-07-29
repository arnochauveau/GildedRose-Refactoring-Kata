use std::cmp::min;

use crate::gildedrose::{
    config::CONFIG, item::Item, update_behavior::UpdateBehavior,
};

pub static BACKSTAGE_PASS_BEHAVIOR: BackstagePassBehavior =
    BackstagePassBehavior {};
pub struct BackstagePassBehavior;

impl UpdateBehavior for BackstagePassBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        let mut owned_item = item.clone();

        let amount_to_increase_option = match owned_item.sell_in {
            ..=0 => None,
            1..=5 => Some(3),
            6..=10 => Some(2),
            11.. => Some(1),
        };

        owned_item.quality = match amount_to_increase_option {
            Some(amount_to_increase) => {
                min(owned_item.quality + amount_to_increase, CONFIG.max_quality)
            }
            None => 0,
        };

        owned_item.sell_in -= 1;
        owned_item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_should_increase_by_1_when_sellin_is_larger_than_10() {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            11,
            0,
        ));

        assert_eq!(result.sell_in, 10);
        assert_eq!(result.quality, 1);
    }

    #[test]
    fn quality_should_increase_by_2_when_sellin_is_10() {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            10,
            0,
        ));

        assert_eq!(result.sell_in, 9);
        assert_eq!(result.quality, 2);
    }

    #[test]
    fn quality_should_increase_by_2_when_sellin_is_less_than_10_but_larger_than_5(
    ) {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            7,
            0,
        ));

        assert_eq!(result.sell_in, 6);
        assert_eq!(result.quality, 2);
    }

    #[test]
    fn quality_should_increase_by_3_when_sellin_is_5() {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            5,
            0,
        ));

        assert_eq!(result.sell_in, 4);
        assert_eq!(result.quality, 3);
    }

    #[test]
    fn quality_should_increase_by_2_when_sellin_is_less_than_5_but_larger_than_0(
    ) {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            3,
            0,
        ));

        assert_eq!(result.sell_in, 2);
        assert_eq!(result.quality, 3);
    }

    #[test]
    fn quality_should_be_0_when_sellin_is_0() {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            0,
            8,
        ));

        assert_eq!(result.sell_in, -1);
        assert_eq!(result.quality, 0);
    }

    #[test]
    fn quality_doesnt_go_above_50() {
        let result = BACKSTAGE_PASS_BEHAVIOR.update(&Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            4,
            50,
        ));

        assert_eq!(result.sell_in, 3);
        assert_eq!(result.quality, 50);
    }
}
