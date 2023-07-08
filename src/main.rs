fn main() {
    let _idx: u8 = 2;
    let tup: (f32, char, f64, &str) = (10.5, 'c', 4.5, "less");
    // prefixing with _ lets clippy know that its intentionally unused
    let mut some_name_here: [i32; 4] = [0; 4];
    some_name_here[0] = 1;
    ls();
    println!("{}", tup.0);
}

fn ls(){
    println!("Ls");
}
