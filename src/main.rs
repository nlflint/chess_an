use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum MoveRule {
    Absolute{ initial_rank: usize, delta: Vector },
    Relative(Vector),
    RelativeRotatable(Vector),
    RelativeRotatableScalable(Vector)
}

#[derive(Debug, PartialEq)]
struct Vector {
	x: usize,
	y: usize
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Piece {
    Pawn,
    Knight,
    Castle,
    Bishop,
    Queen,
    King
}

fn rule_set() -> HashMap<Piece,Vec<MoveRule>> {
    let mut rule_set: HashMap<Piece,Vec<MoveRule>> = HashMap::new();
    
    rule_set.insert(Piece::Pawn, vec![
        MoveRule::Absolute { initial_rank: 2, delta: Vector{ x: 0, y: 2 } },
        MoveRule::Relative( Vector { x: 0, y: 2 } )
    ]);

    rule_set.insert(Piece::Knight, vec![
        MoveRule::RelativeRotatable( Vector { x: 1, y: 2 } ),
        MoveRule::RelativeRotatable( Vector { x: 2, y: 1 } ),
    ]);

    rule_set.insert(Piece::Castle, vec![
        MoveRule::RelativeRotatableScalable( Vector { x: 0, y: 1 } ),
    ]);

    rule_set.insert(Piece::Bishop, vec![
        MoveRule::RelativeRotatableScalable( Vector { x: 1, y: 1 } ),
    ]);

    rule_set.insert(Piece::Queen, vec![
        MoveRule::RelativeRotatableScalable( Vector { x: 0, y: 1 } ),
        MoveRule::RelativeRotatableScalable( Vector { x: 1, y: 1 } )
    ]);

    rule_set.insert(Piece::King, vec![
        MoveRule::RelativeRotatable( Vector { x: 0, y: 1 } ),
        MoveRule::RelativeRotatable( Vector { x: 1, y: 1 } )
    ]);

    return rule_set;
}

#[test]
fn rule_set_contains_kings_moves() {
    let _rules = rule_set();
    assert!(_rules.contains_key(&Piece::King));
}