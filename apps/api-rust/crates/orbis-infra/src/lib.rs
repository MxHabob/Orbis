pub mod migration;
pub mod models;
pub mod repositories;
pub mod event_bus;
pub mod security;
pub mod adapters;
pub mod autopilot;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
