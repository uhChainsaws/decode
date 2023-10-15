#[derive(Debug)]
pub struct MyMatrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<bool>>,
}

impl MyMatrix {
    pub fn from_string(input: String) -> MyMatrix {
        let mut lines = input.lines();
        let (cols, rows): (usize, usize) = {
            let sizes: Vec<usize> = lines.next()
                .expect("invalid G structure")
                .split_whitespace()
                .map(|num| usize::from_str_radix(num, 10)
                .expect("bad gen matrix format"))
                .collect();
            (sizes[0], sizes[1])
        };
        let data: Vec<Vec<bool>> = lines
            .map(|line| {
                // dbg!(&line);
                line.chars().map(|c| c != '0').collect()
        }).collect();
        MyMatrix {
            rows,
            cols,
            data,
        }
    }
    pub fn xor_rows(&mut self, i: usize, j: usize) {
        for k in 0..self.cols {
            self.data[i][k] ^= self.data[j][k];
        }
    }
    pub fn row_echelon(&mut self) {
        let mut rows_done = 0;
        for current_column in 0..self.cols {
            let mut row_is_empty = true;
            for row in rows_done..self.rows {
                if self.data[row][current_column] {
                    row_is_empty = false;
                    break;
                }
            }
            if row_is_empty {
                continue;
            }
            let mut first_row_with_1 = rows_done;
            while !self.data[first_row_with_1][current_column] {
                first_row_with_1 += 1;
            }
            self.data.swap(rows_done, first_row_with_1);
            for row in (rows_done + 1)..self.rows {
                if self.data[row][current_column] {
                    self.xor_rows(row, rows_done);
                }
            }
            rows_done += 1;
        }
    }
    pub fn superpose_rows(&self, v: &Vec<bool>) -> Vec<bool>{
        let mut result = vec![false; self.cols];
        let v = &v[..self.rows];
        for (i, bit) in v.iter().enumerate() {
            if *bit {
                for j in 0..self.cols {
                    result[j] ^= self.data[i][j];
                }
            }
        }
        result
    }
    pub fn swap_cols(&mut self, i: usize, j: usize) {
        for row in &mut self.data {
            let i_bit = row[i];
            let j_bit = row[j];
            if i_bit != j_bit {
                row[i] ^= true;
                row[j] ^= true;
            }
        }
    }
    pub fn rearrange_cols(&mut self, new_order: &Vec<usize>) {
        let mut new_data = vec![vec![false; self.cols]; self.rows];
        for (i, row) in self.data.iter().enumerate() {
            for (j, bit) in row.iter().enumerate() {
                new_data[i][new_order[j]] = *bit;
            }
        }
        self.data = new_data;
    }
    pub fn get_square_span_dimention(&self) -> usize {
        let mut span = 0;
        let min_dim = std::cmp::min(self.rows, self.cols);
        for i in 0..min_dim {
            for j in 0..min_dim {
                if self.data[i][j] {
                    span += 1;
                    break;
                }
            }
        }
        span
    }
}

impl std::fmt::Display for MyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for bit in row {
                if *bit {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
