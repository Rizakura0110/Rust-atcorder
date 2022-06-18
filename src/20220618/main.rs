use proconio::input;

fn main() {
    input! {
        a: i32,
    }
    let mut y = 1;
    let mut n = 1;

    loop {
        y = y*2;
        if n ==a {
            break;
        }
        n = n+1;
        
    }

    println!("{}", y);
    
}