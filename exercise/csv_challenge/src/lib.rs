mod core;
mod err;

pub use crate::core::read::{load_csv,write_csv};
pub use crate::core::write::{replace_column};

#[cfg(test)]
mod tests {
    use crate::load_csv;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let result=load_csv(PathBuf::from(r"src/input/input.csv"));
        assert!(result.is_ok());

    }
}
