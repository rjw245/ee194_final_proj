
#![feature(link_args)]
#![feature(libc)]


extern crate libc;
use libc::c_int;

const NLOOPS: usize = 100000000;

#[link_args = "-L./ -lsimple"]
#[link(name = "simple", kind="static")]

extern {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main(){
    for i in 0..NLOOPS {
        let s = unsafe{ add(3,5) };
    }
}
