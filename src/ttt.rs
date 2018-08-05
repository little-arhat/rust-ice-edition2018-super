use rand;

pub trait S {
    fn s(&mut self) -> u8;
}

pub mod it {
    use super::rand;
    use rand::random;

    pub struct N;
    impl N {
        pub fn new() -> N {
            N
        }
    }

    impl super::S for N {
        fn s(&mut self) -> u8 {
            random() * rand::random()
        }
    }
}
