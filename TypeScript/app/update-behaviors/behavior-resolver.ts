import { Item } from "@app/item";
import { IUpdateBehavior } from "./update-behavior.interface";
import { DefaultBehavior } from "./implementations/default/default-behavior";
import { AgedBrieBehavior } from "./implementations/aged-brie/aged-brie-behavior";
import { BackstagePassBehavior } from "./implementations/backstage-pass/backstage-pass-behavior";
import { LegendaryItemBehavior } from "./implementations/legendary-item/legendary-item-behavior";
import { ConjuredItemBehavior } from "./implementations/conjured-item/conjured-item-behavior";

export function getUpdateBehaviorFor(item: Item): IUpdateBehavior {
  switch (item.name) {
    case "Aged Brie":
      return new AgedBrieBehavior(item);
    case "Backstage passes to a TAFKAL80ETC concert":
      return new BackstagePassBehavior(item);
    case "Sulfuras, Hand of Ragnaros":
      return new LegendaryItemBehavior(item);
    case "Conjured Mana Cake":
      return new ConjuredItemBehavior(item);
    default:
      return new DefaultBehavior(item);
  }
}
