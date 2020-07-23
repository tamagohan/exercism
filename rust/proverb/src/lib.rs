use std::iter;

pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        _ => {
            return list
                .iter()
                .zip((list[1..].iter()).chain(iter::once(&"")))
                .map(|(w1, w2)| {
                    if w2 != &"" {
                        format!("For want of a {} the {} was lost.", w1, w2)
                    } else {
                        format!("And all for the want of a {}.", list[0])
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
        }
    }
}
