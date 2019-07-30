use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum MoveRule {
    Absolute{ required_rank: isize, vector: Vector },
    Relative(Vector),
    RelativeRotatable(Vector),
    RelativeRotatableScalable(Vector)
}

trait Projectable {
    fn project(&self, location: &Vector) -> Vec<Vector>;
}

impl Projectable for MoveRule {
    fn project(&self, location: &Vector) -> Vec<Vector> {
        match &self {
            MoveRule::Relative(vector) => return vec![location.add(&vector)],
            MoveRule::Absolute{ required_rank, vector } => {
                if required_rank == &location.y {
                    return vec![location.add(&vector)];
                }
                return vec![];
            },
            MoveRule::RelativeRotatable(vector) => {
                let _first_move = location.add(&vector);
                return vec![_first_move,
                    vector.rotate(location, Rotate::Ninety),
                    vector.rotate(location, Rotate::OneEighty),
                    vector.rotate(location, Rotate::TwoSeventy)
                ];
            },
            MoveRule::RelativeRotatableScalable(vector) => {
                let origin = &Vector { x: 0, y: 0 };
                return [
                    &vector.multiply(location)[..],
                    &vector.rotate(origin, Rotate::Ninety).multiply(location)[..],
                    &vector.rotate(origin, Rotate::OneEighty).multiply(location)[..],
                    &vector.rotate(origin, Rotate::TwoSeventy).multiply(location)[..]
                ].concat();
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
struct Vector {
	x: isize,
	y: isize
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        return Vector { x: self.x + other.x, y: self.y + other.y };
    }

    fn rotate(&self, pivot: &Vector, rotation: Rotate) -> Vector {
        match rotation {
            Rotate::Ninety => Vector {x: pivot.x + self.y, y: pivot.y - self.x},
            Rotate::OneEighty => Vector {x: pivot.x - self.x, y: pivot.y - self.y},
            Rotate::TwoSeventy => Vector {x: pivot.x - self.y, y: pivot.y + self.x}
        }
    }
    
    fn scale(&self, center: &Vector, scaler: isize) -> Vector {
        Vector {x: center.x + (self.x * scaler), y: center.y + (self.y * scaler)}
    }

    fn multiply(&self, center: &Vector) -> Vec<Vector> {
        return (1..8)
            .into_iter()
            .map(|scaler| self.scale(center, scaler))
            .collect();
    }
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

enum Rotate {
    Ninety,
    OneEighty,
    TwoSeventy
}

struct Board {
    Size: Vector
}

impl Board {
    fn contains(&self, vector: &Vector) -> bool {
        return vector.x >= 0 &&
            vector.y >= 0 &&
            vector.x <= self.width() &&
            vector.y <= self.height();
        
    }

    fn width(&self) -> isize {
        self.Size.x
    }
    
    fn height(&self) -> isize {
        self.Size.y
    }
}

fn rule_set() -> HashMap<Piece,Vec<MoveRule>> {
    let mut rule_set: HashMap<Piece,Vec<MoveRule>> = HashMap::new();
    
    rule_set.insert(Piece::Pawn, vec![
        MoveRule::Absolute { required_rank: 2isize, vector: Vector{ x: 0, y: 2 } },
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
    let _board = standard_chess_board();
    return _rules
        .flat_map(|rule| rule.project(location))
        .filter(|vector| _board.contains(vector))
        .collect();
}

fn standard_chess_board() -> Board {
    Board{Size: Vector { x: 7, y: 7} }
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

#[test]
fn knight() {
    let possible_moves = find_moves(Piece::Knight, &Vector { x: 5, y: 3 });
    println!("{:?}",possible_moves);
    assert!(possible_moves == vec![
        Vector{ x: 6, y: 5 },
        Vector{ x: 7, y: 2 },
        Vector{ x: 4, y: 1 },
        Vector{ x: 3, y: 4 },
        Vector{ x: 7, y: 4 },
        Vector{ x: 6, y: 1 },
        Vector{ x: 3, y: 2 },
        Vector{ x: 4, y: 5 }
    ]);
}

#[test]
fn king() {
    let possible_moves = find_moves(Piece::King, &Vector { x: 1, y: 1 });
    println!("{:?}",possible_moves);
    assert!(possible_moves == vec![
        Vector{ x: 1, y: 2 },
        Vector{ x: 2, y: 1 },
        Vector{ x: 1, y: 0 },
        Vector{ x: 0, y: 1 },
        Vector{ x: 2, y: 2 },
        Vector{ x: 2, y: 0 },
        Vector{ x: 0, y: 0 },
        Vector{ x: 0, y: 2 }
    ]);
}

#[test]
fn castle() {
    let possible_moves = find_moves(Piece::Castle, &Vector { x: 3, y: 2 });
    println!("{:?}",possible_moves);
    assert!(possible_moves.contains(&Vector { x: 3, y: 3 }));
    assert!(possible_moves.contains(&Vector { x: 3, y: 7 }));
    assert!(possible_moves.contains(&Vector { x: 4, y: 2 }));
    assert!(possible_moves.contains(&Vector { x: 7, y: 2 }));
    assert!(possible_moves.contains(&Vector { x: 3, y: 1 }));
    assert!(possible_moves.contains(&Vector { x: 3, y: 0 }));
    assert!(possible_moves.contains(&Vector { x: 2, y: 2 }));
    assert!(possible_moves.contains(&Vector { x: 0, y: 2 }));
}

#[test]
fn moves_beyond_lower_bounds_of_board_not_returned() {
    let possible_moves = find_moves(Piece::Castle, &Vector { x: 3, y: 2 });
    println!("{:?}",possible_moves);
    assert!(!possible_moves.contains(&Vector { x: -1, y: 2 }));
    assert!(!possible_moves.contains(&Vector { x: 3, y: -1 }));
}

#[test]
fn moves_beyond_upper_bounds_of_board_not_returned() {
    let possible_moves = find_moves(Piece::Castle, &Vector { x: 5, y: 5 });
    println!("{:?}",possible_moves);
    assert!(!possible_moves.contains(&Vector { x: 5, y: 8 }));
    assert!(!possible_moves.contains(&Vector { x: 8, y: 5 }));
}