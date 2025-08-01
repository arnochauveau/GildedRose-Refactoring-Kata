import { Item } from "@app/item";
import { getUpdateBehaviorFor } from "./behavior-resolver";
import { AgedBrieBehavior } from "./implementations/aged-brie/aged-brie-behavior";
import { DefaultBehavior } from "./implementations/default/default-behavior";
import { BackstagePassBehavior } from "./implementations/backstage-pass/backstage-pass-behavior";
import { LegendaryItemBehavior } from "./implementations/legendary-item/legendary-item-behavior";
import { ConjuredItemBehavior } from "./implementations/conjured-item/conjured-item-behavior";

describe("Behavior resolver", () => {
  it("should correctly resolve Aged Brie", () => {
    expect(getUpdateBehaviorFor(new Item("Aged Brie", 0, 0))).toBeInstanceOf(
      AgedBrieBehavior
    );
  });

  it("should correctly resolve Backstage Passes", () => {
    expect(
      getUpdateBehaviorFor(
        new Item("Backstage passes to a TAFKAL80ETC concert", 0, 0)
      )
    ).toBeInstanceOf(BackstagePassBehavior);
  });

  it("should correctly resolve Legendary Items", () => {
    expect(
      getUpdateBehaviorFor(new Item("Sulfuras, Hand of Ragnaros", 0, 0))
    ).toBeInstanceOf(LegendaryItemBehavior);
  });
  it("should correctly resolve Conjured Items", () => {
    expect(
      getUpdateBehaviorFor(new Item("Conjured Mana Cake", 0, 0))
    ).toBeInstanceOf(ConjuredItemBehavior);
  });

  it("should correctly resolve the rest to Legacy behavior", () => {
    expect(
      getUpdateBehaviorFor(new Item("some other item", 0, 0))
    ).toBeInstanceOf(DefaultBehavior);
  });
});
