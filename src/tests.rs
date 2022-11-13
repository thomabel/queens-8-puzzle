#[test]
fn crossover() {
    use crate::gene::*;
    use crate::vector::*;
    use bracket_lib::random::RandomNumberGenerator;

    let dimension = Vector2::new(8, 8);
    let mut gene = Gene::new(&dimension);
    let mut gene2 = Gene::new(&dimension);
    let mut rng = RandomNumberGenerator::new();
    gene.randomize(&dimension, &mut rng);
    gene2.randomize(&dimension, &mut rng);
    let genecross = gene.crossover(&gene2, &mut rng);

    println!(" gene1: {}", gene.to_string());
    println!(" gene2: {}\n", gene2.to_string());
    println!("cross1: {}", genecross.0.to_string());
    println!("cross2: {}", genecross.1.to_string());
}

#[test]
fn fitness() {
    use crate::gene::*;
    use crate::vector::*;

    let dimension = Vector2::new(8, 8);
    let gene = Gene::new(&dimension);

    assert_eq!(gene.fitness(), 0);
}

#[test]
fn mutate() {
    use crate::gene::*;
    use crate::vector::*;
    use bracket_lib::random::RandomNumberGenerator;

    let dimension = Vector2::new(8, 8);
    let mut gene = Gene::new(&dimension);
    println!("original: {}", gene.to_string());

    let mut rng = RandomNumberGenerator::new();
    gene.mutate(8, &mut rng);
    println!("  mutate: {}", gene.to_string());
}
