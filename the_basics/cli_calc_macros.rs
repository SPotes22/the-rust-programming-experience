macro_rules! calculator {
    () => {{
        use std::io;

        println!("Bienvenido a la calculadora!");
        println!("Elige una operación: +, -, *, /");

        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();
        let op = op.trim();

        let mut a = String::new();
        let mut b = String::new();

        println!("Ingresa el primer número:");
        io::stdin().read_line(&mut a).unwrap();
        let a: f64 = a.trim().parse().expect("Numero invalido.");

        println!("Ingresa el segundo número:");
        io::stdin().read_line(&mut b).unwrap();
        let b: f64 = b.trim().parse().expect("Numero invalido.");

        if op == "+" {
            println!("Resultado: {}", a + b);
        } else if op == "-" {
            println!("Resultado: {}", a - b);
        } else if op == "*" {
            println!("Resultado: {}", a * b);
        } else if op == "/" {
            if b == 0.0 {
                println!("Error: división por cero");
            } else {
                println!("Resultado: {}", a / b);
            }
        } else {
            println!("Operación no válida");
        }
    }};
}

fn main() {
loop {
    calculator!();

    println!("Hacer otro calculo? (s/n)");
    let mut again = String::new();
    
    std::io::stdin().read_line(&mut again).unwrap();

    if again.trim().to_lowercase() != "s" {
        break;
    }
}}
