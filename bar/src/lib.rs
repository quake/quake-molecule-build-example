use foo::foo as foo;
pub mod bar;

#[cfg(test)]
mod tests {
    use crate::bar::Bar;
    use molecule::prelude::*;

    #[test]
    fn it_works() {
        let bar = Bar::new_builder().build();
        assert_eq!(bar.as_slice(), vec![0u8; 32 + 42]);
    }
}
