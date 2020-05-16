use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut g = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    g.reverse();
    g.into_iter().collect()
}
