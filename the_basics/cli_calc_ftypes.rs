/// This is the main declarative macro that sets up our calculator.
/// When invoked, it defines a set of other declarative macros for basic arithmetic operations.
/// These inner macros will be available in the scope where `calculator!()` is called.
macro_rules! calculator {
    () => {
        /// Defines an `add!` macro for floating-point addition.
        /// It takes two expressions (`$a` and `$b`), casts them to `f64` to ensure
        /// floating-point arithmetic, and returns their sum.
        macro_rules! add {
            ($a:expr, $b:expr) => {
                $a as f64 + $b as f64
            };
        }

        /// Defines a `subtract!` macro for floating-point subtraction.
        /// It takes two expressions (`$a` and `$b`), casts them to `f64`,
        /// and returns their difference.
        macro_rules! subtract {
            ($a:expr, $b:expr) => {
                $a as f64 - $b as f64
            };
        }

        /// Defines a `multiply!` macro for floating-point multiplication.
        /// It takes two expressions (`$a` and `$b`), casts them to `f64`,
        /// and returns their product.
        macro_rules! multiply {
            ($a:expr, $b:expr) => {
                $a as f64 * $b as f64
            };
        }

        /// Defines a `divide!` macro for floating-point division.
        /// It takes two expressions (`$a` and `$b`), casts them to `f64`,
        /// and returns their quotient.
        ///
        /// For floating-point numbers (`f64`), division by zero is handled
        /// by the IEEE 754 standard, resulting in `Infinity` or `NaN` (Not a Number)
        /// depending on the operands. This is considered graceful handling as per
        /// the requirements, so no explicit `Option` or `Result` is needed here.
        macro_rules! divide {
            ($a:expr, $b:expr) => {
                $a as f64 / $b as f64
            };
        }
    };
}

// Invoke the `calculator!` macro to define the `add!`, `subtract!`, `multiply!`, and `divide!` macros.
// These macros will now be available for use in the rest of the code, including `main`.
calculator!();

/// The main function demonstrates the usage of the calculator macros.
fn main() {
    // Define some example floating-point values for calculations.
    let val1 = 25.5;
    let val2 = 5.0;
    let zero_val = 0.0;

    println!("--- Simple Macro Calculator Demonstrations ---");
    println!("Values used: val1 = {}, val2 = {}, zero_val = {}", val1, val2, zero_val);
    println!("--------------------------------------------");

    // Demonstrate the `add!` macro.
    let sum = add!(val1, val2);
    println!("Addition: {} + {} = {}", val1, val2, sum); // Expected: 25.5 + 5.0 = 30.5

    // Demonstrate the `subtract!` macro.
    let difference = subtract!(val1, val2);
    println!("Subtraction: {} - {} = {}", val1, val2, difference); // Expected: 25.5 - 5.0 = 20.5

    // Demonstrate the `multiply!` macro.
    let product = multiply!(val1, val2);
    println!("Multiplication: {} * {} = {}", val1, val2, product); // Expected: 25.5 * 5.0 = 127.5

    // Demonstrate the `divide!` macro with normal values.
    let quotient = divide!(val1, val2);
    println!("Division: {} / {} = {}", val1, val2, quotient); // Expected: 25.5 / 5.0 = 5.1

    // Demonstrate the `divide!` macro with division by zero.
    // For floats, this results in `Infinity` (positive or negative) or `NaN`.
    let div_by_zero_pos = divide!(val1, zero_val);
    println!("Division by zero (positive): {} / {} = {}", val1, zero_val, div_by_zero_pos); // Expected: Infinity

    let div_by_zero_neg = divide!(-val1, zero_val);
    println!("Division by zero (negative): {} / {} = {}", -val1, zero_val, div_by_zero_neg); // Expected: -Infinity

    let zero_div_zero = divide!(zero_val, zero_val);
    println!("Division by zero (0/0): {} / {} = {}", zero_val, zero_val, zero_div_zero); // Expected: NaN
}
