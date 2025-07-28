use crate::item::Item;

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if self.items[i].name != "Aged Brie"
                && self.items[i].name
                    != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name
                        == "Backstage passes to a TAFKAL80ETC concert"
                    {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality =
                                    self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality =
                                    self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != "Aged Brie" {
                    if self.items[i].name
                        != "Backstage passes to a TAFKAL80ETC concert"
                    {
                        if self.items[i].quality > 0 {
                            if self.items[i].name
                                != "Sulfuras, Hand of Ragnaros"
                            {
                                self.items[i].quality =
                                    self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality =
                            self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn quality_and_sellin_decrease_for_standard_items() {
        let mut rose = GildedRose::new(vec![Item::new("standard item", 4, 4)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 3);
        assert_eq!(rose.items[0].quality, 3);
    }

    #[test]
    pub fn double_quality_degradation_sellin_equal_to_zero() {
        let mut rose = GildedRose::new(vec![Item::new("standard item", 0, 4)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, -1);
        assert_eq!(rose.items[0].quality, 2);
    }

    #[test]
    pub fn double_quality_degradation_sellin_lower_than_zero() {
        let mut rose = GildedRose::new(vec![Item::new("standard item", -1, 4)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, -2);
        assert_eq!(rose.items[0].quality, 2);
    }

    #[test]
    fn quality_cant_degrade_below_zero() {
        let mut rose = GildedRose::new(vec![Item::new("standard item", 2, 0)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 1);
        assert_eq!(rose.items[0].quality, 0);
    }

    #[test]
    fn aged_brie_increases_in_quality() {
        let mut rose = GildedRose::new(vec![Item::new("Aged Brie", 2, 2)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 1);
        assert_eq!(rose.items[0].quality, 3);
    }

    #[test]
    fn aged_brie_double_increases_in_quality_sellin_zero() {
        let mut rose = GildedRose::new(vec![Item::new("Aged Brie", 0, 2)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, -1);
        assert_eq!(rose.items[0].quality, 4);
    }

    #[test]
    fn aged_brie_double_increases_in_quality_sellin_lower_than_zero() {
        let mut rose = GildedRose::new(vec![Item::new("Aged Brie", -1, 2)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, -2);
        assert_eq!(rose.items[0].quality, 4);
    }

    #[test]
    fn aged_brie_quality_doesnt_go_above_50() {
        let mut rose = GildedRose::new(vec![Item::new("Aged Brie", 4, 50)]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 3);
        assert_eq!(rose.items[0].quality, 50);
    }

    #[test]
    fn legendary_items_stay_80_quality_and_dont_change_sellin() {
        let mut rose = GildedRose::new(vec![Item::new(
            "Sulfuras, Hand of Ragnaros",
            4,
            80,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 4);
        assert_eq!(rose.items[0].quality, 80);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_1_when_sellin_is_larger_than_10(
    ) {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            11,
            0,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 10);
        assert_eq!(rose.items[0].quality, 1);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_2_when_sellin_is_10() {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            10,
            0,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 9);
        assert_eq!(rose.items[0].quality, 2);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_2_when_sellin_is_less_than_10_but_larger_than_5(
    ) {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            7,
            0,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 6);
        assert_eq!(rose.items[0].quality, 2);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_3_when_sellin_is_5() {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            5,
            0,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 4);
        assert_eq!(rose.items[0].quality, 3);
    }

    #[test]
    fn backstage_pass_quality_should_increase_by_2_when_sellin_is_less_than_5_but_larger_than_0(
    ) {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            3,
            0,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 2);
        assert_eq!(rose.items[0].quality, 3);
    }

    #[test]
    fn backstage_pass_quality_should_be_0_when_sellin_is_0() {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            0,
            8,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, -1);
        assert_eq!(rose.items[0].quality, 0);
    }

    #[test]
    fn backstage_pass_quality_doesnt_go_above_50() {
        let mut rose = GildedRose::new(vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            4,
            50,
        )]);
        rose.update_quality();

        assert_eq!(rose.items[0].sell_in, 3);
        assert_eq!(rose.items[0].quality, 50);
    }
}
