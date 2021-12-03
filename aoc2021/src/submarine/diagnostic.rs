#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};
fn conv_bin_int(n_bits: usize, numbers: Vec<i64>) -> i64 {
    let mut res: i64 = 0;

    for power in (0..n_bits).rev() {
        let index = n_bits-power-1;
        res += 2_i64.pow(power.try_into().unwrap())*numbers[index];
    }

    return res;
}
pub fn get_power_consumtion(path: impl AsRef<Path>) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let output = br.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let n_lines = output.len();

    let len_single_line = output[0].chars().count();
    let flattend = output.iter().flat_map(|s| s.chars()).collect::<Vec<_>>();
    let mut output_flattend = flattend.iter();
    
    let mut gamma = vec![0i64; len_single_line];
    let mut epsilon = vec![1i64; len_single_line];
    
    for i in 0..len_single_line - 1 {
        let cloned_output_flattend = output_flattend.clone();
        let bit = (cloned_output_flattend.map(|x| x.to_string().parse::<i64>().unwrap())).step_by(len_single_line);
        output_flattend.next();

        let bit_mean: f64 = (bit.sum::<i64>() as f64)/(n_lines as f64);

        if bit_mean > 0.5_f64 {
            gamma[i] = 1;
            epsilon[i] = 0;
        }
    }

    let gamma_dec = conv_bin_int(len_single_line, gamma);
    let epsilon_dec = conv_bin_int(len_single_line, epsilon);
    return Ok(gamma_dec * epsilon_dec);
}