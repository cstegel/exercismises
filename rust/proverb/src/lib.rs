/// ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]
/// produces:
/// For want of a nail the shoe was lost.
/// For want of a shoe the horse was lost.
/// For want of a horse the rider was lost.
/// For want of a rider the message was lost.
/// For want of a message the battle was lost.
/// For want of a battle the kingdom was lost.
/// And all for the want of a nail.
extern crate itertools;

use itertools::Itertools;
use std::iter::once;

pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
        .chain(once(format!(
            "And all for the want of a {}.",
            list.get(0).unwrap()
        ))).join("\n")
}
