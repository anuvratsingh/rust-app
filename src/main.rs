use rand::prelude::*;
use std::env;
///This function generates a float number using a number generator passed intp the function.
///
/// # Arguments
/// * generator (&mut ThreadRng): the random number
/// generator to generate the random number
///
/// # Returns
/// (f64): random number between 0 -> 10
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    placeholder * 10.0
}

/// This tarit defines the struct to be a user.
trait IsUser {
    /// This function proclaims that the struct is a user.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (bool) true if user, false if not
    fn is_user() -> bool {
        true
    }
}

/// This struct deines a user
///
/// # Attributes
/// * name (String): the name of the user
/// * age (i8): the age of the user
struct User {
    name: String,
    age: i8,
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    let random_number = generate_float(&mut rng);

    let args: Vec<String> = env::args().collect();
    let path: &str = &args[0];

    if path.contains("/debug/") {
        println!("The development server is running.");
    } else if path.contains("/release/") {
        println!("The production server is running.")
    } else {
        panic!("the seeting is neither debug or release");
    }

    println!("Random number is: {}", random_number);
}
