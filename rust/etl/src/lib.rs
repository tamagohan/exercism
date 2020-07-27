use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .flat_map(|(k, vs)| {
            vs.into_iter()
                .map(move |v| (v.to_lowercase().next().unwrap(), *k))
        })
        .collect()
}
