pub fn raindrops(n: u32) -> String {
    match n {
        n if n % 105 == 0 => "PlingPlangPlong".to_string(),
        n if n % 35 == 0 => "PlangPlong".to_string(),
        n if n % 21 == 0 => "PlingPlong".to_string(),
        n if n % 15 == 0 => "PlingPlang".to_string(),
        n if n % 7 == 0 => "Plong".to_string(),
        n if n % 5 == 0 => "Plang".to_string(),
        n if n % 3 == 0 => "Pling".to_string(),
        _ => n.to_string(),
    }
}
