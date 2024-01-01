// Chapter 1. Category the escense of composition

// 1.4.1
fn identity<T>(x: T) ->T{
    x
}

// 1.4.2
fn compose<A,B,C>(f: impl Fn(A)->B, g: impl Fn(B)->C) -> impl Fn(A)->C{
    move |a: A| g(f(a))
}

// 1.4.3
// for testing
fn add2(n: i32) -> i32{
    n+2
}

fn times10(n: i32) ->i32{
    n*10
}

// Chapter 2: Types and functions

//2.7.1

use std::collections::HashMap;
use std::hash::Hash;


// not sure if this does anything (Clojures how do they work?)
fn memoize<A,B>(f: impl Fn(A)->B)->impl Fn(A)->B where A: PartialEq + Eq +Hash, B: Copy {
    let cache: HashMap<A, B> = HashMap::new();
    move |a: A|->B{
        match cache.get(&a) {
            Some(b) => return *b,
            None => return f(a),
        }
    }
}

fn fib(n: u32) ->u32 {
    if n<2{
        return 1
    }else{
        return fib(n-1)+fib(n-2)
    }
}
fn main() {
    let x = 2;
    let y = identity(x);
    let add2times10 = compose(add2, times10);
    let z = add2times10(x);
    let a = compose(add2, identity)(x);
    let b = compose(identity, add2)(x);
    let m = (2,3,"asdf");
    let _n = identity(m);
    let xs = x.to_string();
    let ys = y.to_string();
    println!("la identidad de {xs} es {ys}");
    println!("add2times10 de {x} es igual a {z}");
    println!("add2 ° identity de {x} es {a}");
    println!("identity ° add 2 de {x} es {b}");
    let na=100;
    // let fibna = fib(na);
    let mfib = memoize(fib)(na);
    // println!("fib de {na} es {fibna}");
    println!("fib de {na} es {mfib}");
}
