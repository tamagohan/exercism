use std::iter;

pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        _ => {
            let hoge1 = list.iter().zip((list[1..].iter()).chain(iter::once(&"")));
            return hoge1
                .map(|(x, y)| {
                    if y != &"" {
                        format!("For want of a {} the {} was lost.", x, y)
                    } else {
                        format!("And all for the want of a {}.", list[0])
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
        }
    }
}
