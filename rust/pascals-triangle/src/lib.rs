pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = Self::new_imp(row_count);
        Self { triangle: rows }
    }
    fn new_imp(row_count: u32) -> Vec<Vec<u32>> {
        match row_count {
            0 => vec![],
            1 => vec![vec![1]],
            _ => {
                let mut triangle = Self::new_imp(row_count - 1);
                let pre_row = triangle.get((row_count - 2) as usize).unwrap();
                let row = (0..row_count)
                    .map(|x| {
                        let l = if x > 0 {
                            pre_row.get((x - 1) as usize).unwrap_or(&0)
                        } else {
                            &0
                        };
                        let r = pre_row.get(x as usize).unwrap_or(&0);
                        l + r
                    })
                    .collect::<Vec<u32>>();
                triangle.push(row);
                triangle
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
