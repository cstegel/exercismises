/// ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]
/// produces:
/// For want of a nail the shoe was lost.
/// For want of a shoe the horse was lost.
/// For want of a horse the rider was lost.
/// For want of a rider the message was lost.
/// For want of a message the battle was lost.
/// For want of a battle the kingdom was lost.
/// And all for the want of a nail.
use std::iter::once;

pub fn build_proverb(list: Vec<&str>) -> String {
    match list.len() {
        0 => String::new(),
        _ => list[..list.len() - 1]
            .iter()
            .zip(list.iter().cycle().skip(1))
            .map(|(first, second)| format!("For want of a {} the {} was lost.\n", first, second))
            .chain(once(format!(
                "And all for the want of a {}.",
                list.get(0).unwrap()
            ))).collect(),
    }
}
