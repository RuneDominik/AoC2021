#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};
fn conv_bin_int(n_bits: usize, numbers: Vec<i64>) -> i64 {
    // convert vec of binary ints to decimal value, from_str_radix would do the trick
    let mut res: i64 = 0;

    for power in (0..n_bits).rev() {
        let index = n_bits - power - 1;
        res += 2_i64.pow(power.try_into().unwrap()) * numbers[index];
    }

    return res;
}
pub fn get_power_consumtion(path: impl AsRef<Path>) -> Result<i64, Error> {
    // read file and get line-length and count
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let output = br.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let n_lines = output.len();

    let len_single_line = output[0].chars().count();
    let flattend = output.iter().flat_map(|s| s.chars()).collect::<Vec<_>>();

    // set some internal variables
    let mut output_flattend = flattend.iter();

    let mut gamma = vec![0i64; len_single_line];
    let mut epsilon = vec![1i64; len_single_line];

    for i in 0..len_single_line - 1 {
        // copy input and conv to int
        let cloned_output_flattend = output_flattend.clone();
        // get needed bit for each line
        let bit = (cloned_output_flattend.map(|x| x.to_string().parse::<i64>().unwrap()))
            .step_by(len_single_line);

        // rotate each round
        output_flattend.next();

        // compute mean, should be >0.5 if 1 appears more than 0
        let bit_mean: f64 = (bit.sum::<i64>() as f64) / (n_lines as f64);

        if bit_mean > 0.5_f64 {
            gamma[i] = 1;
            epsilon[i] = 0;
        }
    }

    // conv to dec done by hand, from_str_radix would do
    let gamma_dec = conv_bin_int(len_single_line, gamma);
    let epsilon_dec = conv_bin_int(len_single_line, epsilon);
    return Ok(gamma_dec * epsilon_dec);
}
pub fn get_life_support(path: impl AsRef<Path>) -> Result<i64, Error> {
    // read file and get line-length and count
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let output = br.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let n_lines = output.len();
    let len_single_line = output[0].chars().count();
    let flattend = output.iter().collect::<Vec<_>>();

    // set some internal variables
    let mut common_bit: i64;
    let mut cloned_output = flattend.clone();
    let mut n_lines_int = n_lines;

    // iterate over all bits in one line
    for i in 0..len_single_line {
        // restore input and conv to int
        let cloned_output_flattend = cloned_output
            .iter()
            .flat_map(|s| s.chars())
            .collect::<Vec<_>>();

        let mut bit_unwraped = cloned_output_flattend
            .iter()
            .map(|x| x.to_string().parse::<i64>().unwrap());

        // rotate input for each run of the loop that has happend
        for _j in 0..i {
            bit_unwraped.next();
        }

        // get needed bit for each line
        let bit = bit_unwraped.step_by(len_single_line);

        // compute mean, should be >0.5 if 1 appears more than 0
        let bit_mean: f64 = (bit.sum::<i64>() as f64) / (n_lines_int as f64);

        if bit_mean >= 0.5_f64 {
            common_bit = 1;
        } else {
            common_bit = 0;
        }

        // recompute bits instead of copy, needed to do some kind of boolean masking
        let mut bit_unwraped_copy = cloned_output_flattend
            .iter()
            .map(|x| x.to_string().parse::<i64>().unwrap());
        for _j in 0..i {
            bit_unwraped_copy.next();
        }
        let mut bit_copy = bit_unwraped_copy.step_by(len_single_line);

        // remove those lines that are no longer needed
        cloned_output.retain(|_| bit_copy.next() == Some(common_bit));

        // update linelength and check if only one line is left
        n_lines_int = cloned_output.len();
        if n_lines_int == 1 {
            break;
        }
    }

    let oxy = i64::from_str_radix(cloned_output[0], 2).unwrap();

    // repeat for carbon

    let mut common_bit: i64;
    let mut cloned_output = flattend.clone();
    let mut n_lines_int = n_lines;
    for i in 0..len_single_line {
        let cloned_output_flattend = cloned_output
            .iter()
            .flat_map(|s| s.chars())
            .collect::<Vec<_>>();

        let mut bit_unwraped = cloned_output_flattend
            .iter()
            .map(|x| x.to_string().parse::<i64>().unwrap());
        for _j in 0..i {
            bit_unwraped.next();
        }
        let bit = bit_unwraped.step_by(len_single_line);
        let bit_mean: f64 = (bit.sum::<i64>() as f64) / (n_lines_int as f64);

        if bit_mean >= 0.5_f64 {
            common_bit = 0;
        } else {
            common_bit = 1;
        }

        let mut bit_unwraped_copy = cloned_output_flattend
            .iter()
            .map(|x| x.to_string().parse::<i64>().unwrap());
        for _j in 0..i {
            bit_unwraped_copy.next();
        }
        let mut bit_copy = bit_unwraped_copy.step_by(len_single_line);
        cloned_output.retain(|_| bit_copy.next() == Some(common_bit));

        n_lines_int = cloned_output.len();
        if n_lines_int == 1 {
            break;
        }
    }

    let carbon = i64::from_str_radix(cloned_output[0], 2).unwrap();
    
    // return product
    return Ok(oxy * carbon);
}
