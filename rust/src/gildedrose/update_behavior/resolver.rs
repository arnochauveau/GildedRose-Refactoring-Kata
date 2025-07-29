use crate::gildedrose::update_behavior::{
    legacy_behavior::LegacyBehavior, UpdateBehavior,
};

use super::Item;

pub fn resolve_update_behavior(item: &Item) -> impl UpdateBehavior {
    match item.name {
        _ => LegacyBehavior {},
    }
}
