use zk_rust_io;

fn main() {
    let n: u32 = zk_rust_io::read();
    zk_rust_io::commit(&n);

    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 7919; // Modulus to prevent overflow.
        a = b;
        b = c;
    }

    zk_rust_io::commit(&a);
    zk_rust_io::commit(&b);
}

fn input() {
    let n = 1000u32;
    zk_rust_io::write(&n);
}

fn output() {
    let (a, b): (u32, u32) = zk_rust_io::out();

    println!("a: {}", a);
    println!("b: {}", b);
}
