#![feature(link_args)]
#![feature(libc)]

extern crate libc;
use libc::c_int;
use libc::c_float;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::sync::Arc;
use std::thread;
use std::env;

#[link_args = "-L./ -lbs"]
#[link(name = "bs", kind="static")]

extern {
    fn BlkSchlsEqEuroNoDiv( sptprice: c_float,
                            strike: c_float, rate: c_float, volatility: c_float,
                            time: c_float, otype: c_int, timet: c_float ) -> c_float;
}

// const NTHREADS: usize = 1;
// const testpath: &'static str = "../inputs/in_64K.txt";

struct TestParams {
    otype: c_int,
    sptprice: c_float,
    strike: c_float,
    rate: c_float,
    volatility: c_float,
    otime: c_float,
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let NTHREADS: usize = args[1].parse::<usize>().unwrap();
    let ref testpath = args[2];

    let mut tests: Vec<TestParams> = Vec::new();
    loadTestData(&mut tests, testpath);
    if NTHREADS>1 { 
        let tests_arc = Arc::new(tests);
        for threadnum in 0..NTHREADS {
            let tests_copy = tests_arc.clone();
            let start = threadnum*tests_copy.len();
            let mut end = (threadnum+1)*tests_copy.len();
            if end > tests_copy.len() {
                end = tests_copy.len();
            }
            thread::spawn(move || {
                for testnum in start..end {
                    let sptprice: c_float = tests_copy[testnum].sptprice;
                    let strike: c_float = tests_copy[testnum].strike;
                    let rate: c_float = tests_copy[testnum].rate;
                    let volatility: c_float = tests_copy[testnum].volatility;
                    let time: c_float = tests_copy[testnum].otime;
                    let otype: c_int = tests_copy[testnum].otype;
                    let timet: c_float = 0.0;
                    let s = unsafe{ BlkSchlsEqEuroNoDiv(sptprice, strike, rate, volatility, time, otype, timet ) };
                    // println!("{}",s);
                }
            });
        }
    } else {
        let start=0; let end=tests.len();
        for testnum in start..end {
            let sptprice: c_float = tests[testnum].sptprice;
            let strike: c_float = tests[testnum].strike;
            let rate: c_float = tests[testnum].rate;
            let volatility: c_float = tests[testnum].volatility;
            let time: c_float = tests[testnum].otime;
            let otype: c_int = tests[testnum].otype;
            let timet: c_float = 0.0;
            let s = unsafe{ BlkSchlsEqEuroNoDiv(sptprice, strike, rate, volatility,   time, otype, timet ) };
            // println!("{}",s);
        }
    }
}

fn loadTestData(tests: &mut Vec<TestParams>, path_to_test: & String ) {
    let testfile = BufReader::new(File::open(path_to_test).unwrap());
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


