use std::{ process, time::{ Duration, Instant }, error::Error };
use rayon::prelude::*;
use csv::WriterBuilder;

fn csv_run(bsn_vec: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
                                    .delimiter(b'\n')
                                    .from_path("./all_bsns.csv")?;
    wtr.write_field("bsn")?;
    wtr.write_record(bsn_vec)?;
    wtr.flush()?;
    Ok(())
}

fn to_digits_vec(v: u32) -> Vec<u8> {
    format!("{:09}", v).as_str()
        .bytes()
        .map(|b: u8| b - b'0')
        .collect::<Vec<u8>>()
}

fn is_valid_bsn(bsn: u32) -> bool {
    let bsn_vec:Vec<u8> = to_digits_vec(bsn);
    let multiply_list: [i8; 9] = [9, 8, 7, 6, 5, 4, 3, 2, -1];

    let bsn_sum: i32 = bsn_vec.iter()
        .zip(multiply_list.iter())
        .map(|(bsn_digit, factor)| *bsn_digit as i32 * *factor as i32)
        .sum();

    bsn_sum % 11 == 0
}

fn gen_valid_bsns() -> Vec<String> {
    let max_n: u32 = 999_999_999;
    let bsns_range = 1..=max_n;
    println!("Generating using {} threads ...", rayon::current_num_threads());

    let valid_bsns: Vec<String> = bsns_range
        .into_par_iter()
        .filter(|&bsn| is_valid_bsn(bsn))
        .map(|bsn| format!("{:0>9}", bsn.to_string()))
        .collect();

    valid_bsns
}


fn main() {
    let start_time = Instant::now();
    let data = gen_valid_bsns();

    println!("Writing to CSV ...");

    if let Err(err) = csv_run(data) {
        println!("{}", err);
        process::exit(1);
    }

    let elapsed_time = start_time.elapsed();

    println!("\nDone ({elapsed_time:.1?}).\nSaved to ./all_bsns.csv")
}
