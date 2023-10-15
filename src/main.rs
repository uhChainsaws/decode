#![allow(non_snake_case, unused)]

use rand::Rng;
use rand_distr::Normal;

mod decode;
mod my_matrix;

fn main() {
    println!("decode (by paramour & uhChainsaws)");
    let mut rng = rand::thread_rng();
    let gen_raw = std::fs::read_to_string("resources/RM_16_11_4.gen")
        .expect("please put the .gen file in ../resources/ folder");
    let mut G = my_matrix::MyMatrix::from_string(gen_raw);
    G.row_echelon();
    println!("G is: \n{}", G);
    let m: Vec<bool> = vec![true, false, true, false];
    // let mess_full: Vec<Vec<bool>> = m.chunks(G.rows).collect();
    // mess_full.last_mut().expect("empty message").
    let c = G.superpose_rows(&m);
    println!("c is: {:?}", c);
    let mut c_fuzzed: Vec<(bool, f64)> = c.iter().map(|b| {
        let sigma = 0.2_f64;
        let e: f64 = rng.sample::<f64,_>(Normal::new(0., sigma).unwrap());
        let y = *b as u8 as f64 + e;
        decode::soft_decode(y, sigma)
    }).collect();

    dbg!(&c_fuzzed);

    c_fuzzed.iter_mut().for_each(|(b, _)| {
        if *b {
            print!("1");
        }
        else {
            print!("0");
        }
    });
    println!();

    let mut soft_abs = c_fuzzed.iter().map(|(_, soft)| soft.abs()).collect::<Vec<f64>>();
    let mut soft_abs_enumerated = soft_abs.iter_mut().enumerate().collect::<Vec<(usize, &mut f64)>>();
    // dbg!(&soft_abs_enumerated);
    soft_abs_enumerated.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    // soft_abs_enumerated.reverse();
    let new_order = soft_abs_enumerated.iter().map(|(i, _)| *i).collect();

    dbg!(&soft_abs_enumerated);
    G.rearrange_cols(&new_order);
    G.row_echelon();
    if G.get_square_span_dimention() < G.rows {
        println!("{}", G.get_square_span_dimention());
        G.swap_cols(G.rows-1, G.rows);
        soft_abs_enumerated.swap(G.rows-1, G.rows);
        println!("now it is{}", G.get_square_span_dimention());
    }
    println!("G is now: \n{}", G);
    let c_fuzz_sure = soft_abs_enumerated.iter().map(|(i, _)| c[*i]).collect::<Vec<bool>>();
    let good_guess = G.superpose_rows(&c_fuzz_sure[..G.rows].to_vec());
    let new_order: Vec<usize> = soft_abs_enumerated.iter().map(|(i, _)| *i).collect();
    // let mut test = new_order.clone();
    // dbg!(&test);
    let good_guess_original_order = decode::reorder_reverse(good_guess, &new_order);
    // let test = decode::reorder(test, &new_order);
    // dbg!(&test);
    print!("if sorted by reliability: ");
    good_guess_original_order.iter().for_each(|b| {
        if *b {
            print!("1");
        }
        else {
            print!("0");
        }
    });
    println!();

}
