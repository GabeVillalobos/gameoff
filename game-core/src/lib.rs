extern crate amethyst;
extern crate rand;

mod component;
mod map;
pub mod system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
