use std::io;

fn outer(x: i32, y: i32) {}

fn trace(v: &Vec<i64>, size: usize) {
    print!("[");

    for i in 0..size {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v[i]);
    }
    print!("]");
}

fn u_size() -> usize {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect(
        "ERROR Cant Read array size",
    );
    let u_size: usize = buf.trim().parse().unwrap();
    return u_size;
}

fn i32_vector() -> Vec<i64> {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect(
        "ERROR Cant Read input array",
    );
    let v: Vec<i64> = buf.split_whitespace().map(|n| n.parse().unwrap()).collect();
    return v;
}

fn main() {
    println!("input x size:");
    let x_size: usize = u_size();
    println!("x_size: {}", x_size);

    let x: Vec<i64> = i32_vector();
    trace(&x, x_size);
    print!("\n");

    println!("input y size:");
    let y_size: usize = u_size();
    println!("y_size: {}", y_size);

    let y: Vec<i64> = i32_vector();
    trace(&y, y_size);

}
