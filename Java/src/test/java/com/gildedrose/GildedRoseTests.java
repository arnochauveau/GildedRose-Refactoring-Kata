package com.gildedrose;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Nested;

import static org.junit.jupiter.api.Assertions.assertEquals;

class GildedRoseTests {

    @Nested()
    class NormalItems {

        @Test
        void shouldDecreaseQualityBy1() {
            Item[] items = new Item[] { new Item("foo", 3, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(2, app.items[0].quality);
        }

        @Test
        void shouldDecreaseSellInBy1() {
            Item[] items = new Item[] { new Item("foo", 3, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(2, app.items[0].sellIn);
        }

        @Test
        void shouldDecreaseQualityBy2WhenSellInIs0() {
            Item[] items = new Item[] { new Item("foo", 0, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(1, app.items[0].quality);
        }

        @Test
        void shouldNotDecreaseQualityUnder0() {
            Item[] items = new Item[] { new Item("foo", 3, 0) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(0, app.items[0].quality);
        }

        @Test
        void shouldNotDecreaseQualityUnder0WhenSellInIs0() {
            Item[] items = new Item[] { new Item("foo", 0, 0) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(0, app.items[0].quality);
        }

        @Test
        void shouldNotDecreaseQualityUnder0WhenSellInIsUnder0() {
            Item[] items = new Item[] { new Item("foo", -3, 0) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(0, app.items[0].quality);
        }

    }

    @Nested
    class AgedBrie {
        @Test
        void shouldIncreaseQualityBy1() {
            Item[] items = new Item[] { new Item("Aged Brie", 2, 0) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(1, app.items[0].quality);
        }

        @Test
        void shouldIncreaseQualityBy2WhenSellInIs0() {
            Item[] items = new Item[] { new Item("Aged Brie", 0, 0) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(2, app.items[0].quality);
        }

        @Test
        void shouldIncreaseQualityBy2WhenSellInIsUnder0() {
            Item[] items = new Item[] { new Item("Aged Brie", -3, 0) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(2, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50() {
            Item[] items = new Item[] { new Item("Aged Brie", 3, 50) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellInIs0() {
            Item[] items = new Item[] { new Item("Aged Brie", 0, 50) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellInIsUnder0() {
            Item[] items = new Item[] { new Item("Aged Brie", -3, 50) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenQualityIs49AndSellInIsUnder0() {
            Item[] items = new Item[] { new Item("Aged Brie", -3, 49) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }
    }

    @Nested
    class LegendaryItems {
        @Test
        void shouldNotChangeQuality() {
            Item[] items = new Item[] { new Item("Sulfuras, Hand of Ragnaros", 0, 80) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(80, app.items[0].quality);
        }
    }

    @Nested
    class BackstagePasses {
        @Test
        void shouldIncreaseQualityBy1WhenSellinIsMoreThan10() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 11, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(4, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellinIsMoreThan10() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 11, 50) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldIncreaseQualityBy2WhenSellinIs10() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 10, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(5, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellinIs10() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 10, 49) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldIncreaseQualityBy2WhenSellinIsLessThan10ButMoreThan5() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 7, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(5, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellinIsLessThan10ButMoreThan5() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 7, 49) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldIncreaseQualityBy3WhenSellInIs5() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 5, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(6, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellinIs5() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 7, 48) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldIncreaseQualityBy3WhenSellInIsLessThan5ButMoreThan0() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 3, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(6, app.items[0].quality);
        }

        @Test
        void shouldNotIncreaseQualityOver50WhenSellinIsLessThan5ButMoreThan0() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 3, 48) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(50, app.items[0].quality);
        }

        @Test
        void shouldSetQualityTo0WhenSellInIs0() {
            Item[] items = new Item[] { new Item("Backstage passes to a TAFKAL80ETC concert", 0, 3) };
            GildedRose app = new GildedRose(items);
            app.updateQuality();
            assertEquals(0, app.items[0].quality);
        }
    }
}
