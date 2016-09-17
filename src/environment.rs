//! Keeps track of the cockroach state
//!
//! The environment is responsible for keeping track of the world the cockroach
//! lives in. The environment consists of the location of the cockroach, which
//! start out at the origin. And the location of a pile of food, which the
//! cockroach is interested in.


/// The environment the cockroach finds herself
#[derive(Debug, PartialEq, Eq)]
pub struct Environment {
    /// The location of the cockroach
    pub cockroach_location: i32,
    /// The location of the pile of food
    pub food_location: i32,
}

impl Environment {
    /// Create a new Environment with a pile of food at a certain location. The
    /// cockroach starts out at the origin
    pub fn new(food_location: i32) -> Environment {
        Environment {
            cockroach_location: 0,
            food_location: food_location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_should_be_created_with_new() {
        let expected_food_location: i32 = 37;
        let environment: Environment = Environment::new(expected_food_location);

        assert_eq!(environment, Environment {
            cockroach_location : 0,
            food_location: expected_food_location,
        });
    }
}
