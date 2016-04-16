// #![feature(link_args)]
// #![feature(libc)]


// extern crate libc;
// use libc::c_int;
// use libc::c_float;
use std::fs::File;
use std::io::{self, BufReader};

// #[link_args = "-L./ -lbs"]
// #[link(name = "bs", kind="static")]

// extern {
//     fn BlkSchlsEqEuroNoDiv( sptprice: c_float,
//                             strike: c_float, rate: c_float, volatility: c_float,
//                             time: c_float, otype: c_int, timet: c_float ) -> c_float;
// }

struct TestParams {
    otype: c_int,
    sptprice: c_float,
    strike: c_float,
    rate: c_float,
    volatility: c_float,
    otime: c_float,
}

fn main(){
    let testpath = "input.txt";
    let mut tests: Vec<TestParams> = Vec::new();
    loadTestData(&mut tests, testpath);
    // let sptprice: c_float = 1.0;
    // let strike: c_float = 1.0;
    // let rate: c_float = 1.0;
    // let volatility: c_float = 1.0;
    // let time: c_float = 1.0;
    // let otype: c_int = 1;
    // let timet: c_float = 1.0;
    // for i in 0..NLOOPS {
    //     let s = unsafe{ BlkSchlsEqEuroNoDiv(sptprice, strike, rate, volatility, time, otype, timet ) };
    //     // println!("{}",s);
    // }
}

fn loadTestData(tests: &mut Vec<TestParams>, testpath: String) {
    let testfile = try!(File::open(testpath));
    let testfile = BufReader::new(f);
    for test in testfile.lines() {
        let numbers: Vec<f32> = test.splite_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for num in numbers {
            println!("{}",num);
        }
    }
}


