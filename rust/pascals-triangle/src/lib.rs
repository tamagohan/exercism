use std::iter;

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
                let pre_row = triangle.last().unwrap();
                let zip1 = iter::once(&0).chain(pre_row.iter());
                let zip2 = (pre_row.iter()).chain(iter::once(&0));
                let row = zip1.zip(zip2).map(|(m, n)| m + n).collect();
                triangle.push(row);
                triangle
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
