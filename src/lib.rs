mod bindings;
pub mod error;
pub mod geoip;
pub mod socket;
pub mod tor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
