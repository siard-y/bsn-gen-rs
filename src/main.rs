use std::error::Error;
use std::process;
use rayon::prelude::*;
use indicatif::ProgressBar;
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

fn to_digits(v: i32) -> Vec<u8> {
    format!("{:09}", v)
        .as_str()
        .bytes()
        .map(|b: u8| (b - b'0') as u8)
        .collect::<Vec<u8>>()
}

fn is_valid_bsn(bsn: i32) -> bool {
    let bsn_vec:Vec<u8> = to_digits(bsn);
    let multiply_list: [i8; 9] = [9, 8, 7, 6, 5, 4, 3, 2, -1];

    let mut bsn_sum: i32 = 0;

    for (idx, digit) in bsn_vec.iter().enumerate() {
        bsn_sum += *digit as i32 * multiply_list[idx] as i32;
    }

    bsn_sum % 11 == 0
}

fn gen_valid_bsns() -> Vec<String> {
    let max_n: i32 = 999999999;
    let bsns_range = 1..=max_n;
    let pb = ProgressBar::new(max_n as u64);

    let mut valid_bsns: Vec::<String> = Vec::new();

    for bsn in bsns_range {
        if is_valid_bsn(bsn) {
            valid_bsns.push(format!("{:0>9}", bsn.to_string()));
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");

    valid_bsns
}


fn main() {
    let data = gen_valid_bsns();

    if let Err(err) = csv_run(data) {
        println!("{}", err);
        process::exit(1);
    }
}
