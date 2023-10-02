use std::convert::From;

const CHIP_COL_MAP: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
const CHIP_ROW_MAP: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Point can be compared with Point, the result is the same
// as the both result of x and y
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<std::cmp::Ordering> {
        let x = self.x.cmp(&other.x);
        let y = self.y.cmp(&other.y);
        use std::cmp::Ordering as O;
        match (x, y) {
            (O::Equal, O::Equal) => Some(O::Equal),
            (O::Greater, O::Greater) => Some(O::Greater),
            (O::Less, O::Less) => Some(O::Less),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Chip {
    id: String,
}

// convert Point to Chip based on CHIP_ROW_MAP and CHIP_COL_MAP
impl From<Point> for Chip {
    fn from(p: Point) -> Self {
        let x = p.x;
        let y = p.y;
        let id = format!("{}{}", CHIP_COL_MAP[x as usize], CHIP_ROW_MAP[y as usize]);
        Chip { id }
    }
}

// convert ref Point to Chip
impl From<&Point> for Chip {
    fn from(p: &Point) -> Self {
        let x = p.x;
        let y = p.y;
        let id = format!("{}{}", CHIP_COL_MAP[x as usize], CHIP_ROW_MAP[y as usize]);
        Chip { id }
    }
}

// Chip is comparable and result is the same as Point
impl PartialOrd for Chip {
    fn partial_cmp(&self, other: &Chip) -> Option<std::cmp::Ordering> {
        let chip_1: Point = self.into();
        let chip_2: Point = other.into();
        chip_1.partial_cmp(&chip_2)
    }
}

// convert Chip to Point based on CHIP_ROW_MAP and CHIP_COL_MAP
impl From<Chip> for Point {
    fn from(c: Chip) -> Self {
        let id = c.id;
        let x = CHIP_COL_MAP
            .iter()
            .position(|&r| r == id.chars().nth(0).unwrap())
            .unwrap();
        let y = CHIP_ROW_MAP
            .iter()
            .position(|&r| r == id.chars().nth(1).unwrap())
            .unwrap();
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

// convert ref Chip to Point
impl From<&Chip> for Point {
    fn from(c: &Chip) -> Self {
        let id = &c.id;
        let x = CHIP_COL_MAP
            .iter()
            .position(|&r| r == id.chars().nth(0).unwrap())
            .unwrap();
        let y = CHIP_ROW_MAP
            .iter()
            .position(|&r| r == id.chars().nth(1).unwrap())
            .unwrap();
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let c: Chip = p.into();
    println!("{:?}", c);
    let p: Point = (&c).into();
    println!("{:?}", p);
    println!("{:?}", c);

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 3 };
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p1 < p2);

    let c1: Chip = p1.into();
    let c2: Chip = p2.into();
    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", c1 < c2);
}
