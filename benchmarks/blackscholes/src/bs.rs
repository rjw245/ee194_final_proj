#![feature(link_args)]
#![feature(libc)]

extern crate libc;
use libc::c_int;
use libc::c_float;

const NLOOPS: usize = 100000000;

#[link_args = "-L./ -lbs"]
#[link(name = "bs", kind="static")]

extern {
    fn BlkSchlsEqEuroNoDiv( sptprice: c_float,
                            strike: c_float, rate: c_float, volatility: c_float,
                            time: c_float, otype: c_int, timet: c_float ) -> c_float;
}

fn main(){
    let sptprice: c_float = 1.0;
    let strike: c_float = 1.0;
    let rate: c_float = 1.0;
    let volatility: c_float = 1.0;
    let time: c_float = 1.0;
    let otype: c_int = 1;
    let timet: c_float = 1.0;
    for i in 0..NLOOPS {
        let s = unsafe{ BlkSchlsEqEuroNoDiv(sptprice, strike, rate, volatility, time, otype, timet ) };
        // println!("{}",s);
    }
}

