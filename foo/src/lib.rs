pub mod foo;

pub static SCHEMA: &[u8] = include_bytes!("../schemas/foo.mol");

#[cfg(test)]
mod tests {
    use crate::foo::F32;
    use molecule::prelude::*;

    #[test]
    fn it_works() {
        let f32 = F32::new_builder().build();
        assert_eq!(f32.as_slice(), vec![0u8; 32]);
    }
}
