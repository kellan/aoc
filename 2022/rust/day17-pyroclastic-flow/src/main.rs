use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    part1(input, 2022);
}

fn part1(jets: String, cnt: usize) {
    let mut cave = Cave::new();
    let rock_forms = rocks();
    let jets: Vec<char> = jets.chars().collect();
    
    let mut j = 0;
    for _ in 0..cnt {
        let current = rock_forms[ j % rock_forms.len() ];
        let mut rock = cave.rock(current);
        while !cave.is_blocked(&rock) {
            let jet = jets[ j % jets.len()];
            rock.push(jet);
            if !cave.is_blocked(&rock) {
                rock.bottom = rock.bottom.saturating_sub(1);
            }
            j += 1;
        }
        cave.insert_rock(&rock);
    }
    dbg!(cave.floor);

}

#[derive(Debug)]
struct Cave {
    rocks: HashSet<(usize,usize)>,
    floor: usize
}

impl Cave {
    fn new() -> Cave {
        Cave {
            rocks: HashSet::new(),
            floor: 0  // all rocks appear three units above the highest rock/floor
        }
    }

    fn rock(&self, shape: RockForm) -> Rock {
        Rock::new(self.floor, shape)
    }

    fn is_blocked(&self, rock: &Rock) -> bool {
        if rock.bottom <= 0 {
            return true;
        }

        let points = rock.points();
        for p in points.iter() {
            if self.rocks.contains( &(p.0,p.1.saturating_sub(1)) ) {
                return true;
            }
        }

        false
    }

    fn insert_rock(&mut self, rock: &Rock) {
        for p in rock.points() {
            self.rocks.insert(p);
            if p.1 > self.floor {
                self.floor = p.1;
            }
        }
    }
}

#[derive(Debug)]
struct Rock {
    shape: RockForm,
    left: usize,
    bottom: usize,
    width: usize,
    height: usize,
}

impl Rock {
    fn new(floor: usize, shape: RockForm) -> Rock {
        let width = Self::width_from_shape(shape);
        let height = Self::height_from_shape(shape);
        let bottom = floor+3;
        let left = 2;

        Rock {
            shape,
            left,
            bottom,
            width,
            height
        }
    }

    fn points(&self) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = vec![];

        for (y, row) in self.shape.iter().filter(|row| row.iter().any(|u| *u==1) ).enumerate() {
            for x in 0..row.len() {
                if self.shape[y][x] == 1 {
                    points.push(
                        (self.left+x, self.bottom + (self.height-y) -1)
                    )
                }
            }
        }

        points
    }

    fn push(&mut self, jet: char) {
        match jet {
            '>' => {
                if self.left + self.width < 7 {
                    self.left += 1;
                }
            },
            '<' => self.left = self.left.saturating_sub(1),
           _ => panic!("Unknown jet {}", jet)
        };
    }

    fn height_from_shape(shape: RockForm) -> usize {
        shape.iter().filter(|row| row.contains(&1)).count()
    }

    fn width_from_shape(shape: RockForm) -> usize {
        let mut width = 1;
        for row in shape.iter() {
            for (x,u) in row.iter().enumerate() {
                if *u == 1 {
                    width = std::cmp::max(width, x+1);
                }
            }
        }

        width
    }
}

type RockForm = [[u8;4];4];

fn rocks() -> [RockForm; 5] {
    let rocks: [RockForm; 5] = [
        [[1,1,1,1],
         [0,0,0,0],
         [0,0,0,0],
         [0,0,0,0]],

        [[0,1,0,0],
         [1,1,1,0],
         [0,1,0,0],
         [0,0,0,0]],

        [[0,0,1,0],
         [0,0,1,0],
         [1,1,1,0],
         [0,0,0,0]],

        [[1,0,0,0],
         [1,0,0,0],
         [1,0,0,0],
         [1,0,0,0]],

         [[1,1,0,0],
         [1,1,0,0],
         [0,0,0,0],
         [0,0,0,0]],
    ];

    return rocks;
}



fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop() {
        let rock_forms = rocks();
        let mut cave = Cave::new();
        let mut rock = cave.rock(rock_forms[0]);
        while(!cave.is_blocked(&rock)) {
            rock.bottom = rock.bottom.saturating_sub(1);
        }
        assert_eq!(rock.bottom, 0);
        cave.insert_rock(&rock);
        assert!(cave.rocks.contains(&(2,0)));
        assert!(cave.rocks.contains(&(3,0)));
        assert!(cave.rocks.contains(&(4,0)));
        assert!(cave.rocks.contains(&(5,0)));
        assert_eq!(cave.rocks.len(), 4);
        

        let mut rock = cave.rock(rock_forms[0]);
        while(!cave.is_blocked(&rock)) {
            rock.bottom = rock.bottom.saturating_sub(1);
        }
        assert_eq!(rock.bottom, 1);
        cave.insert_rock(&rock);
        assert!(cave.rocks.contains(&(2,1)));
        assert!(cave.rocks.contains(&(3,1)));
        assert!(cave.rocks.contains(&(4,1)));
        assert!(cave.rocks.contains(&(5,1)));
        assert_eq!(cave.rocks.len(), 8);

        let mut rock = cave.rock(rock_forms[1]);
        while(!cave.is_blocked(&rock)) {
            rock.bottom = rock.bottom.saturating_sub(1);
        }
        assert_eq!(rock.bottom, 2);
        cave.insert_rock(&rock);

        assert!(cave.rocks.contains(&(3,2)));
        assert!(cave.rocks.contains(&(3,3)));
        assert!(cave.rocks.contains(&(3,4)));
        assert!(cave.rocks.contains(&(2,3)));
        assert!(cave.rocks.contains(&(4,3)));
        assert_eq!(cave.rocks.len(), 13);

        let mut rock = cave.rock(rock_forms[2]);
        while(!cave.is_blocked(&rock)) {
            rock.bottom = rock.bottom.saturating_sub(1);
        }
        assert_eq!(rock.bottom, 5);
        cave.insert_rock(&rock);

        assert!(cave.rocks.contains(&(2,5)));
        assert!(cave.rocks.contains(&(3,5)));
        assert!(cave.rocks.contains(&(4,5)));
        assert!(cave.rocks.contains(&(4,6)));
        assert!(cave.rocks.contains(&(4,7)));

        assert_eq!(cave.rocks.len(), 18);



    }
    #[test]
    fn test_shapes() {
        let rock_forms = rocks();

        assert_eq!(Rock::height_from_shape(rock_forms[0]), 1);
        assert_eq!(Rock::height_from_shape(rock_forms[1]), 3);
        assert_eq!(Rock::height_from_shape(rock_forms[2]), 3);
        assert_eq!(Rock::height_from_shape(rock_forms[3]), 4);
        assert_eq!(Rock::height_from_shape(rock_forms[4]), 2);

        assert_eq!(Rock::width_from_shape(rock_forms[0]), 4);
        assert_eq!(Rock::width_from_shape(rock_forms[1]), 3);
        assert_eq!(Rock::width_from_shape(rock_forms[2]), 3);
        assert_eq!(Rock::width_from_shape(rock_forms[3]), 1);
        assert_eq!(Rock::width_from_shape(rock_forms[4]), 2);

    }

    #[test]
    fn test_new_cave_rock() {
        let rock_forms = rocks();

        let cave = Cave::new();
        assert_eq!(cave.floor, 0);

        let rock = cave.rock(rock_forms[0]);
        assert_eq!(rock.height, 1);
        assert_eq!(rock.width, 4);
        assert_eq!(rock.bottom, 3);
    }

    fn assert_points(rock: &Rock, test_points: &[(usize,usize)]) {
        let points = rock.points();
        for p in test_points.iter() {
            assert!(points.contains(p));
        }
    }

    #[test]
    fn test_points() {
        let rock_forms = rocks();
        let mut rock = Rock::new(0, rock_forms[0]); // -
        assert_points(&rock, &[(2,3),(3,3),(4,3),(5,3)]);
        let mut rock = Rock::new(0, rock_forms[1]); // +
        assert_points(&rock, &[(3,5),(2,4),(3,4),(4,4),(3,3)]);
        let mut rock = Rock::new(0, rock_forms[2]); // backwards L
        assert_eq!(rock.points().len(), 5);
        assert_points(&rock, &[(2,3),(3,3),(4,3),(4,4),(4,5)]);
        let mut rock = Rock::new(0, rock_forms[3]); // |
        assert_eq!(rock.points().len(), 4);
        assert_points(&rock, &[(2,3),(2,4),(2,5),(2,6)]);
        let mut rock = Rock::new(0, rock_forms[4]); // square
        assert_eq!(rock.points().len(), 4);
        assert_points(&rock, &[(2,3),(3,3),(3,3),(3,4)]);
    }

    #[test]
    fn test_push() {
        let rock_forms = rocks();

        let mut rock = Rock::new(0, rock_forms[0]);
        rock.push('>');
        assert_rock(&rock, 1, 4, 3, 3);
        rock.push('>');
        assert_rock(&rock, 1, 4, 3, 3);
        rock.push('<');
        assert_rock(&rock, 1, 4, 3, 2);
        rock.push('<');
        assert_rock(&rock, 1, 4, 3, 1);
        rock.push('<');
        assert_rock(&rock, 1, 4, 3, 0);
        rock.push('<');
        assert_rock(&rock, 1, 4, 3, 0);

        let mut rock = Rock::new(0, rock_forms[1]);
        assert_rock(&rock, 3, 3, 3, 2);
        rock.push('>');
        assert_rock(&rock, 3, 3, 3, 3);
        rock.push('>');
        assert_rock(&rock, 3, 3, 3, 4);
        rock.push('>');
        assert_rock(&rock, 3, 3, 3, 4);
        rock.push('<');
        assert_rock(&rock, 3, 3, 3, 3);

        

    }

    fn assert_rock(rock: &Rock, height: usize, width: usize, bottom: usize, left: usize) {
        assert_eq!(rock.height, height, "Failed height check for {:?}", rock);
        assert_eq!(rock.width, width, "Failed width check for {:?}", rock);
        assert_eq!(rock.bottom, bottom, "Failed bottom check for {:?}", rock);
        assert_eq!(rock.left, left, "Failed left check for {:?}", rock);
    }

}
