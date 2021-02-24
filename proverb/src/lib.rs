use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|pair| build(pair[0], pair[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect()
}

fn build(first: &str, second: &str) -> String {
    String::from(format!(
        "For want of a {} the {} was lost.\n",
        first, second
    ))
}
