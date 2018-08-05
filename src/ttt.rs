pub trait S {
    fn s(&mut self) -> u8;
}

pub mod it {
    use rand::{self, random};

    pub struct N;
    impl N {
        pub fn new() -> N {
            N
        }
    }

    impl super::S for N {
        fn s(&mut self) -> u8 {
            random::<u8>() * rand::random::<u8>()
        }
    }
}
