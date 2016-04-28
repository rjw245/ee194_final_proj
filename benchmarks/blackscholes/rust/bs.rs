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

#[link_args = "-L./ -lbs -lsniper_roi"]
#[link(name = "bs", kind="static")]
#[link(name = "sniper_roi", kind="static")]

extern {
    fn BlkSchlsEqEuroNoDiv( sptprice: c_float,
                            strike: c_float, rate: c_float, volatility: c_float,
                            time: c_float, otype: c_int, timet: c_float ) -> c_float;
}

extern {
    fn SimRoiStart_wrapper();
}

extern {
    fn SimRoiEnd_wrapper();
}

extern {
    fn SimMarker_wrapper(arg0: c_int, arg1: c_int);
}


// const NTHREADS: usize = 1;
// const testpath: &'static str = "../inputs/in_64K.txt";
const NUM_RUNS : usize = 100;

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
    // println!("Num threads: {}", NTHREADS);
    let ref testpath = args[2];

    let mut tests: Vec<TestParams> = Vec::with_capacity(100000000);
    loadTestData(&mut tests, testpath);

    let mut threads = Vec::new();
    if NTHREADS>1 { 
        unsafe{ SimRoiStart_wrapper() };
        let tests_arc = Arc::new(tests);
        for threadnum in 0..NTHREADS {
            let tests_copy = tests_arc.clone();
            let num_tests = tests_copy.len();
            let start = threadnum*(num_tests / NTHREADS);
            let mut end = (threadnum+1)*(num_tests / NTHREADS);
            if end > num_tests {
                end = num_tests;
            }
            let handle = thread::spawn(move || {
                // println!("Spawned thread");
                // println!("start: {}   end: {}\n",start,end);
                for run in 0..NUM_RUNS {    
                    for testnum in start..end {
                        let sptprice: c_float = tests_copy[testnum].sptprice;
                        let strike: c_float = tests_copy[testnum].strike;
                        let rate: c_float = tests_copy[testnum].rate;
                        let volatility: c_float = tests_copy[testnum].volatility;
                        let time: c_float = tests_copy[testnum].otime;
                        let otype: c_int = tests_copy[testnum].otype;
                        let timet: c_float = 0.0;
                        let s = unsafe{ BlkSchlsEqEuroNoDiv(sptprice, strike, rate, volatility, time, otype, timet ) };
                        // println!("Thread {} running test {}",threadnum,testnum);
                    }
                }
            });
            threads.push(handle);
        }
        for t in threads {
            t.join();
        }
        unsafe{ SimRoiEnd_wrapper() };

    } else {
        unsafe{ SimRoiStart_wrapper() };
        let start=0; let end=tests.len();
        for run in 0..NUM_RUNS {
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
        unsafe{ SimRoiEnd_wrapper() };
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


