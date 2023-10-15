#![allow(non_snake_case, unused)]

use std::f64::consts::{E, PI};

use rand::Rng;

#[derive(Debug)]
struct MyMatrix {
    rows: usize,
    cols: usize,
    data: Vec<u32>,
}

impl MyMatrix {
    fn from_string(input: String) -> MyMatrix {
        let mut lines = input.lines();
        let (rows, cols): (usize, usize) = {
            let sizes: Vec<usize> = lines.next()
                .expect("invalid G structure")
                .split_whitespace()
                .map(|num| usize::from_str_radix(num, 10)
                .expect("bad gen matrix format or dim > 32"))
                .collect();
            (sizes[0], sizes[1])
        };
        let data: Vec<u32> = lines
            .map(|line| {
                // println!("line is => {}", line); // DEBUG!();
                u32::from_str_radix(line, 2).expect("bad gen matrix format or dim > 32")
        })
            .collect();
        MyMatrix {
            rows,
            cols,
            data,
        }
    }

    fn row_echelon(&mut self) {
        let mut rows_done = 0;
        let mut current_column = 32-1;
        while current_column > 0 {
            let mut row_is_empty = true;
            for row in rows_done..self.cols {
                if is_bit_set(self.data[row], current_column) {
                    row_is_empty = false;
                    break;
                }
            }
            if row_is_empty {
                current_column -= 1;
                continue;
            }
            let mut first_row_with_1 = rows_done;
            while !is_bit_set(self.data[first_row_with_1], current_column) {
                first_row_with_1 += 1;
            }
            self.data.swap(rows_done, first_row_with_1);
            for row in (rows_done + 1)..self.cols {
                if is_bit_set(self.data[row], current_column) {
                    self.data[row] ^= self.data[rows_done];
                }
            }
            rows_done += 1;
            current_column -= 1;
        }
    }
    fn superpose_rows(&mut self, v: Vec<u8>) -> Vec<u8>{
        let mut result = 0;
        let v = &v[..self.cols];
        // println!("{:?}", v);
        for (i, bit) in v.iter().enumerate() {
            if *bit == 1 {
                result ^= self.data[i];
            }
        }
        vec_of_u8_from_u32(result)
    }
    fn swap_cols(&mut self, i: usize, j: usize) {
        for row in &mut self.data {
            let i_bit = is_bit_set(*row, i);
            let j_bit = is_bit_set(*row, j);
            if i_bit != j_bit {
                *row ^= 1 << i;
                *row ^= 1 << j;
            }
        }
    }
}

impl std::fmt::Display for MyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // dbg!(&self);
        let longest_row_len: usize = self.data.iter()
            .max_by_key(|r| r.count_ones())
            .unwrap()
            .count_ones() 
            as usize;
        for row in &self.data {
            for i in (0..longest_row_len).rev() {
                if is_bit_set(*row, i) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
use rand_distr::Normal;

fn main() {
    println!("decode (by paramour & uhChainsaws)");
    let mut rng = rand::thread_rng();
    // let gen_raw = std::fs::read_to_string("resources/test.gen")
    let gen_raw = std::fs::read_to_string("resources/RM_16_11_4.gen")
        .expect("please put the .gen file in ../resources/ folder");

    let mut G = MyMatrix::from_string(gen_raw);
    dbg!(&G.cols);
    println!("G is: \n{}", G);
    G.row_echelon();
    // row_echelon(&mut G.data);
    println!("G is now: \n{}", G);

    let m: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0];
    let c = G.superpose_rows(m);
    // dbg!(&c);
    let mut c_fuzz: Vec<(u8, f64)> = c.iter().map(|b| {
            let sigma = 0.1_f64;
            let e: f64 = rng.sample::<f64,_>(Normal::new(0., sigma).unwrap());
            // let e = 0.;
            // let e: f64 = rng.sample::<f64,_>(StandardNormal);
            let y = *b as f64 + e;
            // dbg!(&e);
            // dbg!(&y);
            soft_decode(y, sigma)
        }
    ).collect();
    // dbg!(&c_fuzz);
    c_fuzz.iter().for_each(|(b, _)| print!("{}", b));
    println!();
    let mut to_be_inverted: Vec<u8> = (0..G.rows as u8).collect();
    // sort by absolute soft value (asc) with quicksort while swapping columns in G and in to_be_inverted
    fn quicksort<T: PartialOrd>(v: &mut [T], v2: &mut [u8]) {
        if v.len() <= 1 {
            return;
        }
        let pivot = v.len() - 1;
        let mut i = 0;
        for j in 0..pivot {
            if v[j] < v[pivot] {
                v.swap(i, j);
                v2.swap(i, j);
                i += 1;
            }
        }
        v.swap(i, pivot);
        v2.swap(i, pivot);
        quicksort(&mut v[0..i], &mut v2[0..i]);
        quicksort(&mut v[i+1..], &mut v2[i+1..]);
    }
    quicksort(&mut c_fuzz, &mut to_be_inverted);
    // dbg!(&c_fuzz);
    // dbg!(&to_be_inverted);
    for (i, j) in to_be_inverted.iter().enumerate() {
        if i == *j as usize {
            continue;
        }
        G.swap_cols(i, *j as usize);
    }
    println!("G is now: \n{}", G);
    println!("to_be_inverted: {:?}", to_be_inverted);
    G.row_echelon();
    println!("G is now: \n{}", G);
    let i_guess = G.superpose_rows(c_fuzz.iter().map(|(b, _)| *b).collect::<Vec<u8>>()[..G.rows].to_vec());
    println!("i_guess: {:?}", i_guess);
    
}

fn is_bit_set(num: u32, i: usize) -> bool {
    ((num >> i)) & 1 == 1
}

// fn row_echelon(G: &mut Vec<u32>) {
//     let mut rows_done = 0;
//     let mut current_column = 32-1;
//     while current_column > 0 {
//         let mut row_is_empty = true;
//         for row in rows_done..G.len() {
//             if is_bit_set(G[row], current_column) {
//                 row_is_empty = false;
//                 break;
//             }
//         }
//         if row_is_empty {
//             current_column -= 1;
//             continue;
//         }
//         let mut first_row_with_1 = rows_done;
//         while !is_bit_set(G[first_row_with_1], current_column) {
//             first_row_with_1 += 1;
//         }
//         G.swap(rows_done, first_row_with_1);
//         for row in (rows_done + 1)..G.len() {
//             if is_bit_set(G[row], current_column) {
//                 G[row] ^= G[rows_done];
//             }
//         }
//         rows_done += 1;
//         current_column -= 1;
//     }
// }

// fn prettyprint_matrix(G: &Vec<u32>) {
//     let longest_row_len: usize = G.iter()
//         .max_by_key(|r| r.count_ones())
//         .unwrap()
//         .count_ones() 
//         as usize;
//     for row in G {
//         for i in (0..longest_row_len).rev() {
//             if is_bit_set(*row, i) {
//                 print!("1 ");
//             } else {
//                 print!("0 ");
//             }
//         }
//         println!();
//     }
// }

fn L(y: f64, sigma: f64) -> f64 {
    (P(y, 1., sigma)/P(y, 0., sigma)).ln()
}
fn P(y: f64, x: f64, sigma: f64) -> f64 {
    E.powf(-(y-x)*(y-x)/(2.*sigma*sigma))/(sigma*sigma*PI*2.).sqrt()
}
fn soft_decode(y: f64, sigma: f64) -> (u8, f64) {
    let soft = L(y, sigma);
    println!("{y}:{soft}");
    let hard = if soft > 0. {
        1
    } else {
        0
    };
    (hard, soft)
}

fn vec_of_u8_from_u32(num: u32) -> Vec<u8> {
    let mut v = Vec::new();
    for i in (0..32).rev() {
        if is_bit_set(num, i) {
            v.push(1);
        } else {
            v.push(0);
        }
    }
    v
}
