use std::sync::Arc;
use std::thread;

const N: usize          = 10000000;
const THREADS: usize    = 7;

fn main() {

    let mut x: Vec<f32> = vec![0.0; N];
    let mut y: Vec<f32> = vec![0.0; N];
    init_vecs(&mut x, &mut y);

    let x_arc = Arc::new(x);
    let y_arc = Arc::new(y);
    let mut handles = Vec::new();
    for t in 0..THREADS {
        let x_shared = x_arc.clone();
        let y_shared = y_arc.clone();
        let handle = thread::spawn(move || {
            println!("Spawned thread");
            let start   = t    *N / THREADS;
            let mut end = (t+1)*N / THREADS;
            if(t==THREADS-1) { end = N; }
            let partial_dotprod: f32 = dot_prod(&x_shared,&y_shared,start,end);
            partial_dotprod
        });
        handles.push(handle);
    }

    let mut dotprod: f32 = 0.0;

    for handle in handles {
        dotprod += handle.join().unwrap();
    }
    println!("{}",dotprod);
}

fn init_vecs(x: &mut Vec<f32>, y: &mut Vec<f32>) {
    let mut factor = N as f32;
    let denom_sqrd: f32 = ( 2.0 * factor * factor + 3.0 * factor + 1.0);
    factor = 1.0 / denom_sqrd.sqrt();

    for i in 0..x.len() {
        x[i] = ((i+1) as f32) * factor;
        y[i] = ((i+1) as f32) * 6.0 * factor;
    }
}

fn dot_prod(x: & Vec<f32>, y: & Vec<f32>, start: usize, end: usize) -> f32 {
    let mut sum: f32 = 0.0;
    for i in start..end {
        if(i>=0 && i<x.len()) {
            sum = sum + (x[i] * y[i]);
        }
    }
    sum
}
