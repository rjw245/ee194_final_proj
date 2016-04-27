use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
use std::thread::JoinHandle;
const N: usize          = 1048576;

fn main() {
    let args: Vec<String> = env::args().collect();
    let THREADS: usize = args[1].parse::<usize>().unwrap();

    let mut x: Vec<f64> = vec![0.0; N];
    let mut y: Vec<f64> = vec![0.0; N];
    init_vecs(&mut x, &mut y);
    
    if THREADS==1 {
        let dotprod = dot_prod(&x, &y, 0, N);
        println!("{}",dotprod);
    } else { 
        let x_arc = Arc::new(x);
        let y_arc = Arc::new(y);
        let mut handles : Vec<JoinHandle<(f64)>> = Vec::new();

        for t in 0..THREADS {
            let x_shared = x_arc.clone();
            let y_shared = y_arc.clone();

            let handle = thread::spawn(move || {
                let start   = t    *N / THREADS;
                let mut end = (t+1)*N / THREADS;
                if t==THREADS-1 { end = N; }
                let local_sum = dot_prod(&x_shared,&y_shared,start,end);
                local_sum
            });

            handles.push(handle);
        }

        let mut global_sum : f64 = 0.0;
        for handle in handles {
            global_sum += handle.join().unwrap();
        }

        println!("{}", global_sum);
    }
}

fn init_vecs(x: &mut Vec<f64>, y: &mut Vec<f64>) {
    //Initialize so that dot product = N
    for i in 0..x.len() {
        x[i] = 4.0;
        y[i] = 0.25;
    }
}

fn dot_prod(x: & Vec<f64>, y: & Vec<f64>, start: usize, end: usize) -> f64 {
    let mut dotprod: f64 = 0.0;
    for i in start..end {
        if i < x.len() {
            dotprod = dotprod + (x[i] * y[i]);
        }
    }
    dotprod
}
