
//! A sorted array of arbitrary values, with a secondary index that allows linear access over the
//! value distribution of the array.
//! Shows `rand` crate and sorting of partial ordered values.

extern crate rand;

use rand::{thread_rng, Rng};
use std::cmp::{Ordering};

/// Rust tries to be truthful about sorting floats -- not all floats can be strictly ordered (e.g.
/// is NaN before or after zero?). This function can be passed to `Vec::sort_by` to ignore that and
/// get an ordering like in more loose languages.
fn float_order<T:std::cmp::PartialOrd>(a:&T,b:&T) -> Ordering { a.partial_cmp(b).unwrap_or(Ordering::Equal) }

fn main() {
    let mut rng = thread_rng();
    let mut data = rng.gen_iter::<f64>().take(20).collect::<Vec<f64>>();

    data.sort_by(float_order);

    let lvec = lindex(&data);

    graph_print(&data, &lvec);
}

/// return a vector of indexes that best match a linear progression
fn lindex (inp: &Vec<f64>) -> Vec<usize> {
    let upper = *inp.last().unwrap();
    let lower = *inp.first().unwrap();
    let range = upper - lower;
    let lerp : f64 = range / (inp.len() as f64);
    let mut p : f64 = lower;

    // the vec we will fill
    let mut outp: Vec<usize> = Vec::with_capacity(inp.len());
    // current index through input
    let mut j = 0;

    for _ in 0..inp.len() {
        p += lerp;
        while (j < inp.len()-1) && (inp[j+1] <= p) {j += 1;}
        outp.push(j);
    }

    return outp;
}

fn graph_print(vals: &Vec<f64>, lin: &Vec<usize>) {
    let mut i = 0usize;
    for v in vals {
        println!("{}", dots(v));
        println!("{}", lines(&vals[lin[i]]));
        i+=1;
    }
}
fn dots(v:&f64) -> String {
    let n = (v * 80f64).floor() as usize;
    return String::from_utf8(vec![b'*'; n]).unwrap();
}
fn lines(v:&f64) -> String {
    let n = (v * 80f64).floor() as usize;
    return String::from_utf8(vec![b'-'; n]).unwrap();
}
