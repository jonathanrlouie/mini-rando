use rand::{
    thread_rng, Rng,
    distributions::Uniform
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const ID_LENGTH: usize = 10;

pub struct SeedId(pub String);

pub struct IntSeed(pub u64);

pub struct Seed {
    pub id: SeedId,
    pub int_seed: IntSeed
}

impl Seed {
    pub fn generate_seed() -> Self {
        let range = Uniform::new_inclusive(0, 35);

        let id = thread_rng()
            .sample_iter(&range)
            .take(ID_LENGTH)
            .map(|c: u8| map_to_id_char(c))
            .collect::<String>();

        let int_seed = hash_seed_id(&id);

        Seed {
            id: SeedId(id),
            int_seed
        }
    }
}

fn hash_seed_id<T: Hash>(id: &T) -> IntSeed {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    IntSeed(hasher.finish())
}

// TODO: return result here instead
fn map_to_id_char(num: u8) -> char {
    debug_assert!(num <= 35);
    match num {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        26 => '0',
        27 => '1',
        28 => '2',
        29 => '3',
        30 => '4',
        31 => '5',
        32 => '6',
        33 => '7',
        34 => '8',
        35 => '9',
        _ => 'A'
    }
}