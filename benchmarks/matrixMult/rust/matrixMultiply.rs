//Concurrency modules
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

const TOTAL_SIZE:       usize = 16;
const NTHREADS:         usize = 1;

fn main(){
    let mut thread_list: Vec<JoinHandle<i64>> = Vec::new();




    let mut a : Vec<Vec<f32>> = Vec::new();
    let mut b : Vec<Vec<f32>> = Vec::new();
    let mut c : Vec<Vec<f32>> = Vec::new();

    init_matrix(&mut a, TOTAL_SIZE);
    init_matrix(&mut b, TOTAL_SIZE);
    init_matrix(&mut c, TOTAL_SIZE);

    //split into separate threads

    for t in 0..NTHREADS {
        multiply(&a, &b, &mut c, t);
    }

    //rejoin

    print_matrix(&a);
    println!("*");
    print_matrix(&b);
    println!("=");
    print_matrix(&c);
}

fn multiply(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>, c: &mut Vec<Vec<f32>>, slice: usize){
    let s = slice;
    let from = (s * TOTAL_SIZE) / NTHREADS;
    let to   = ((s+1) * TOTAL_SIZE) / NTHREADS;
    for i in from..to {
        for j in 0..TOTAL_SIZE {
            c[i][j] = 0.0;
            for k in 0..TOTAL_SIZE {
                c[i][j]+=a[i][k] * b[k][j];
            }
        }
    }
}

fn init_matrix(m: &mut Vec<Vec<f32>>, size: usize) {
    let mut val : f32 = 0.0;
    for r in 0..size{
        m.push(Vec::new());
        for c in 0..size{
            m[r].push(val);
            val=val+1.0;
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
