pub mod angular;
pub mod ast_math;
pub mod distance;
pub mod temperature;
pub mod traits;
// simple re-usable helpers for unit tests
mod unit_test_helpers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
