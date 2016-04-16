#![feature(link_args)]
#![feature(libc)]

extern crate libc;
use libc::c_int;
use libc::c_float;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[link_args = "-L./ -lbs"]
#[link(name = "bs", kind="static")]

extern {
    fn BlkSchlsEqEuroNoDiv( sptprice: c_float,
                            strike: c_float, rate: c_float, volatility: c_float,
                            time: c_float, otype: c_int, timet: c_float ) -> c_float;
}

struct TestParams {
    otype: c_int,
    sptprice: c_float,
    strike: c_float,
    rate: c_float,
    volatility: c_float,
    otime: c_float,
}

fn main(){
    let testpath = "../inputs/in_4.txt";
    let mut tests: Vec<TestParams> = Vec::new();
    loadTestData(&mut tests, testpath);
    for test in tests {
        let sptprice: c_float = test.sptprice;
        let strike: c_float = test.strike;
        let rate: c_float = test.rate;
        let volatility: c_float = test.volatility;
        let time: c_float = test.otime;
        let otype: c_int = test.otype;
        let timet: c_float = 0.0;
        let s = unsafe{ BlkSchlsEqEuroNoDiv(sptprice, strike, rate, volatility, time, otype, timet ) };
        println!("{}",s);
    }
}

fn loadTestData(tests: &mut Vec<TestParams>, testpath: & str) {
    let testfile = BufReader::new(File::open(testpath).unwrap());
    for test2 in testfile.lines() {
        let test = test2.unwrap();
        let params = test.split_whitespace();
        let params: Vec<&str> = params.collect();
        if params.len()>1 {
            let mut otype_val = 0;
            if params[6]=="P" {
                otype_val = 1;
            }
            let test_params = TestParams{
                otype: otype_val as c_int,
                sptprice: params[0].parse::<f32>().unwrap(),
                strike: params[1].parse::<f32>().unwrap(),
                rate: params[2].parse::<f32>().unwrap(),
                volatility: params[4].parse::<f32>().unwrap(),
                otime: params[5].parse::<f32>().unwrap(),
            };
            tests.push(test_params);

        }
    }
}


