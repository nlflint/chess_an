use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum MoveRule {
    Absolute{ required_rank: usize, vector: Vector },
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
        MoveRule::Absolute { required_rank: 2, vector: Vector{ x: 0, y: 2 } },
        MoveRule::Relative( Vector { x: 0, y: 1 } )
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

fn find_moves(piece: Piece, location: &Vector) -> Vec<Vector> {
    let _rule_set = rule_set();
    let _rules = _rule_set.get(&piece).unwrap().into_iter();

    return _rules.flat_map(|rule| {
        match rule {
            MoveRule::Relative(vector) => return vec![location.add(&vector)],
            MoveRule::Absolute{ required_rank, vector} => {
                if required_rank == &location.y {
                    return vec![location.add(&vector)];
                }
                return vec![];
            },
            _ => vec![]
        }
    }).collect();
    
    
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        return Vector { x: self.x + other.x, y: self.y + other.y };
    }
}

#[test]
fn rule_set_contains_kings_moves() {
    let _rules = rule_set();
    assert!(_rules.contains_key(&Piece::King));
}

#[test]
fn pawn_moves_one_forward() {
    let possible_moves = find_moves(Piece::Pawn, &Vector { x: 3, y: 3 });
    println!("{:?}",possible_moves);
    assert!(possible_moves == vec![Vector{ x: 3, y: 4 }]);
}

#[test]
fn second_rank_pawn_can_make_two_moves() {
    let possible_moves = find_moves(Piece::Pawn, &Vector { x: 7, y: 2 });
    println!("{:?}",possible_moves);
    assert!(possible_moves == vec![
        Vector{ x: 7, y: 4 },
        Vector{ x: 7, y: 3 }
    ]);
}