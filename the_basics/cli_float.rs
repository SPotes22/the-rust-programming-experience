macro_rules! calculator {
    () => {
        macro_rules! add { ($a:expr, $b:expr) => { $a as f64 + $b as f64 }; }
        macro_rules! subtract { ($a:expr, $b:expr) => { $a as f64 - $b as f64 }; }
        macro_rules! multiply { ($a:expr, $b:expr) => { $a as f64 * $b as f64 }; }
        macro_rules! divide { ($a:expr, $b:expr) => { $a as f64 / $b as f64 }; }
    };
}
calculator!();

fn main() {
    println!("{}", add!(5, 2));
    println!("{}", subtract!(5, 2));
    println!("{}", multiply!(5, 2));
    println!("{}", divide!(5, 2));
}

