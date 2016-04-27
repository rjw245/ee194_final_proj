//Concurrency modules
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::env;

extern crate crossbeam;
use crossbeam::Scope;

const TOTAL_SIZE:       usize = 1024;

fn main(){
    let args: Vec<String> = env::args().collect();
    let NTHREADS: usize = args[1].parse::<usize>().unwrap();
        
    let mut thread_list: Vec<JoinHandle<()>> = Vec::new();

    let mut a : Vec<Vec<f32>> = Vec::new();
    let mut b : Vec<Vec<f32>> = Vec::new();
    let mut shared_c = vec![0.0; TOTAL_SIZE * TOTAL_SIZE];

    init_matrix(&mut a, TOTAL_SIZE, 1.0);
    init_matrix(&mut b, TOTAL_SIZE, 1.0);

    // split into separate scoped threads
    let mut t = 0;
    if NTHREADS==1 {
        let from = 0;
        let to   = TOTAL_SIZE;
        let mut count = 0;
        for i in from..to {
            for j in 0..TOTAL_SIZE {
                for k in 0..TOTAL_SIZE {
                    shared_c[count] += a[i][k] * b[k][j];
                }
                count += 1;
            }
        }
    } else {
        let shared_a = Arc::new(a);
        let shared_b = Arc::new(b);

        crossbeam::scope(|scope| {
            for chunk in shared_c.chunks_mut(TOTAL_SIZE * TOTAL_SIZE / NTHREADS) {
                let a = shared_a.clone();
                let b = shared_b.clone();
                // let c = shared_c.clone();
                scope.spawn(move ||{
                    println!("Spawned thread {}", t);
                    let from = (t * TOTAL_SIZE) / NTHREADS;
                    let to   = ((t+1) * TOTAL_SIZE) / NTHREADS;
                    let mut count = 0;
                    for i in from..to {
                        for j in 0..TOTAL_SIZE {
                            for k in 0..TOTAL_SIZE {
                                chunk[count] += a[i][k] * b[k][j];
                            }
                            count += 1;
                        }
                    }
                });
                t += 1;
            }
        });
    }


    // print_matrix(&shared_a);
    // println!("*");
    // print_matrix(&shared_b);
    // println!("=");
    // print_res(&shared_c);
}

fn init_matrix(m: &mut Vec<Vec<f32>>, size: usize, increment: f32) {
    let mut val : f32 = 0.0;
    for r in 0..size{
        m.push(Vec::new());
        for c in 0..size{
            m[r].push(val);
            val=val+increment;
        }
    }
}

fn print_res(m: & Vec<f32>) {
    let mut i = 0;
    for item in m {
        print!("{} ",item);
        i += 1;
        if (i % TOTAL_SIZE == 0) {
            println!("");
        }
    }
}

fn print_matrix(m: & Vec<Vec<f32>>) {
    for r in m {
        for item in r {
            print!("{} ",item);
        }
        println!("");
    }
}
