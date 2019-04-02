pub use self::valid_seed_char::ValidSeedChar;
pub use self::seed_id::SeedId;
pub use self::int_seed::IntSeed;
pub use self::seed::Seed;

const ID_LENGTH: usize = 10;

pub mod valid_seed_char {
    pub struct ValidSeedChar(char);

    impl ValidSeedChar {
        pub fn new(num: u8) -> Option<Self> {
            ValidSeedChar::convert_to_char(num).map(|c| ValidSeedChar(c))
        }

        pub fn get(self) -> char {
            self.0
        }

        fn convert_to_char(num: u8) -> Option<char> {
            match num {
                0 => Some('A'),
                1 => Some('B'),
                2 => Some('C'),
                3 => Some('D'),
                4 => Some('E'),
                5 => Some('F'),
                6 => Some('G'),
                7 => Some('H'),
                8 => Some('I'),
                9 => Some('J'),
                10 => Some('K'),
                11 => Some('L'),
                12 => Some('M'),
                13 => Some('N'),
                14 => Some('O'),
                15 => Some('P'),
                16 => Some('Q'),
                17 => Some('R'),
                18 => Some('S'),
                19 => Some('T'),
                20 => Some('U'),
                21 => Some('V'),
                22 => Some('W'),
                23 => Some('X'),
                24 => Some('Y'),
                25 => Some('Z'),
                26 => Some('0'),
                27 => Some('1'),
                28 => Some('2'),
                29 => Some('3'),
                30 => Some('4'),
                31 => Some('5'),
                32 => Some('6'),
                33 => Some('7'),
                34 => Some('8'),
                35 => Some('9'),
                _ => None
            }
        }
    }
}

pub mod seed_id {
    use super::ID_LENGTH;

    #[derive(Clone)]
    pub struct SeedId(String);

    impl SeedId {
        pub fn new(seed_string: String) -> Option<Self> {
            if seed_string.len() == ID_LENGTH {
                Some(SeedId(seed_string))
            } else {
                None
            }
        }

        pub fn get(self) -> String {
            self.0
        }

        pub fn get_clone(&self) -> String {
            self.0.clone()
        }
    }
}

pub mod int_seed {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use super::SeedId;

    #[derive(Copy, Clone)]
    pub struct IntSeed(u64);

    impl IntSeed {
        pub fn get_hashed_seed(seed_id: &SeedId) -> IntSeed {
            let string_id: String = seed_id.get_clone();
            IntSeed::hash_seed_id::<String>(string_id)
        }

        fn hash_seed_id<T: Hash>(id: T) -> IntSeed {
            let mut hasher = DefaultHasher::new();
            id.hash(&mut hasher);
            IntSeed(hasher.finish())
        }

        pub fn get(self) -> u64 {
            self.0
        }

        pub fn get_clone(&self) -> u64 {
            self.0.clone()
        }
    }
}

pub mod seed {
    use rand::{
        thread_rng, Rng,
        distributions::Uniform
    };

    use super::{SeedId, IntSeed, ID_LENGTH, ValidSeedChar};

    pub struct Seed {
        id: SeedId,
        int_seed: IntSeed
    }

    impl Seed {
        pub fn generate_seed() -> Option<Self> {
            let range = Uniform::new_inclusive(0, 35);

            let id_string = thread_rng()
                .sample_iter(&range)
                .take(ID_LENGTH)
                .map(|c: u8| ValidSeedChar::new(c)
                    .map(|v| v.get()))
                .collect::<Option<String>>()?;

            let seed_id = SeedId::new(id_string)?;

            let int_seed = IntSeed::get_hashed_seed(&seed_id);

            Some(Seed {
                id: seed_id,
                int_seed
            })
        }

        pub fn get_id_clone(&self) -> String {
            self.id.get_clone()
        }

        pub fn get_int_seed_clone(&self) -> u64 {
            self.int_seed.get_clone()
        }
    }
}