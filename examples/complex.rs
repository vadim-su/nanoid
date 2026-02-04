use nanoid::nanoid;

use rand::Rng;

fn random(size: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut result = vec![0u8; size];

    rng.fill(&mut result[..]);

    result
}

fn main() {
    nanoid!(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
