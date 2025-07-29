use crate::gildedrose::{item::Item, update_behavior::UpdateBehavior};

pub static LEGENDARY_ITEM_BEHAVIOR: LegendaryItemBehavior =
    LegendaryItemBehavior {};
pub struct LegendaryItemBehavior;

impl UpdateBehavior for LegendaryItemBehavior {
    fn update(
        &self,
        item: &Item,
    ) -> Item {
        item.to_owned()
    }
}

#[cfg(test)]
mod legendary_item_tests {
    use super::*;

    #[test]
    fn stay_80_quality_and_dont_change_sellin() {
        let result = LEGENDARY_ITEM_BEHAVIOR.update(&Item::new(
            "Sulfuras, Hand of Ragnaros",
            4,
            80,
        ));

        assert_eq!(result.sell_in, 4);
        assert_eq!(result.quality, 80);
    }
}
