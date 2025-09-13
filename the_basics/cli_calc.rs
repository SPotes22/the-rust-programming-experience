// The `calculator!` macro serves as a macro-generating macro.
// When invoked, it defines a set of arithmetic operation macros
// that can then be used throughout the code.
macro_rules! calculator {
    () => {
        // Defines the `add!` macro for performing addition.
        // It takes two expressions ($a and $b) and expands directly to their sum.
        macro_rules! add {
            ($a:expr, $b:expr) => {
                $a + $b
            };
        }

        // Defines the `subtract!` macro for performing subtraction.
        // It takes two expressions ($a and $b) and expands directly to their difference.
        macro_rules! subtract {
            ($a:expr, $b:expr) => {
                $a - $b
            };
        }

        // Defines the `multiply!` macro for performing multiplication.
        // It takes two expressions ($a and $b) and expands directly to their product.
        macro_rules! multiply {
            ($a:expr, $b:expr) => {
                $a * $b
            };
        }

        // Defines the `divide!` macro for performing division.
        // This macro is designed to handle division by zero gracefully for integer types
        // by returning an `Option<T>`.
        // If the divisor ($b) is zero, it expands to `None`.
        // Otherwise, it expands to `Some($a / $b)`.
        // This approach works for both integer and floating-point types,
        // consistently returning `None` for division by zero.
        macro_rules! divide_int {
            ($a:expr, $b:expr) => {{
                // for integers
                if $b == 0 {
                    None //dafuk? no ';' this is getting veru flexible...
                } else {
                    Some( $a/ $b )
                }
            }};
        }
            // float operator
        macro_rules! divide_float {($a:expr, $b:expr) => {{
                if $b == 0.0 {
                    None 
                }
            }}  else {
                Some($a / $b)    
            }
            };
        }
}

// Invoke the `calculator!` macro to define `add!`, `subtract!`, `multiply!`, and `divide!`.
// This must be called before any of the operation macros are used.
calculator!();

fn main() {
    println!("--- Calculator Macro Demonstrations ---");

    // Demonstrate the `add!` macro with integer values.
    let sum_int = add!(10, 5);
    println!("10 + 5 = {}", sum_int); // Expected: 15

    // Demonstrate the `add!` macro with floating-point values.
    let sum_float = add!(10.5, 2.3);
    println!("10.5 + 2.3 = {}", sum_float); // Expected: 12.8

    // Demonstrate the `subtract!` macro with integer values.
    let difference_int = subtract!(10, 5);
    println!("10 - 5 = {}", difference_int); // Expected: 5

    // Demonstrate the `subtract!` macro with floating-point values.
    let difference_float = subtract!(10.5, 2.3);
    println!("10.5 - 2.3 = {}", difference_float); // Expected: 8.2

    // Demonstrate the `multiply!` macro with integer values.
    let product_int = multiply!(10, 5);
    println!("10 * 5 = {}", product_int); // Expected: 50

    // Demonstrate the `multiply!` macro with floating-point values.
    let product_float = multiply!(10.5, 2.0);
    println!("10.5 * 2.0 = {}", product_float); // Expected: 21.0

    // Demonstrate the `divide!` macro with valid integer division.
    let quotient_int_ok = divide_int!(10, 5);
    println!("10 / 5 = {:?}", quotient_int_ok); // Expected: Some(2)

    // Demonstrate the `divide!` macro with integer division by zero.
    // This will return `None` as per the macro's graceful handling.
    let quotient_int_err = divide_int!(10, 0);
    println!("10 / 0 = {:?}", quotient_int_err); // Expected: None

    // Demonstrate the `divide!` macro with valid floating-point division.
    let quotient_float_ok = divide_float!(10.0, 4.0);
    println!("10.0 / 4.0 = {:?}", quotient_float_ok); // Expected: Some(2.5)

    // Demonstrate the `divide!` macro with floating-point division by zero.
    // This will also return `None`, providing consistent error handling.
    let quotient_float_err = divide_float!(10.0, 0.0);
    println!("10.0 / 0.0 = {:?}", quotient_float_err); // Expected: None

    // Example with integer division that truncates.
    let quotient_trunc = divide_int!(11, 3);
    println!("11 / 3 = {:?}", quotient_trunc); // Expected: Some(3)
}
