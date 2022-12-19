use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    part1(input, 6);
}

fn part1(jets: String, cnt: usize) {
    let rock_forms = rocks();
    let jets: Vec<char> = jets.chars().collect();
    let current = rock_forms[ 0 ];
    let (mut left, y) = (2, 0); // 3-1
    let mut rock = Rock::new(left, current);

    for i in 0..cnt {
        // drop rock
        //let current = rock_forms[ i % rock_forms.len() ];

        let jet = jets[ i % jets.len()];
        print!("{} {} {} ", i, rock.left, jet);
        //left = push(left, current, jet);
        rock.push(jet);
        println!("{}", rock.left);

    }
}

struct Cave {
    rocks: HashSet<(usize,usize)>,
}

// impl Cave {
//     fn drop(&self, left, y, rock_shape)
// }

struct Rock {
    shape: RockForm,
    left: usize,
    width: usize,
}

impl Rock {
    fn new(left: usize, shape: RockForm) -> Rock {
        let width = shape[0].iter().filter(|n| **n != 0).collect::<Vec<_>>().len();
        Rock {
            shape,
            left,
            width
        }
    }
    fn push(&mut self, jet: char) {
        match jet {
            '>' => {
                if self.left + self.width > 6 {
                    self.left = 7 - self.width;
                } else {
                    self.left += 1;
                }
            },
            '<' => self.left = self.left.saturating_sub(1),
           _ => panic!("Unknown jet {}", jet)
        };
    }

}

// fn x_in_range(x: usize) -> usize {
//     if x > 7 {
//         return 7;
//     } else if x < 0 {
//         return 0;
//     } else {
//         return x;
//     }
// }

// struct Cave {
//     rocks: HashSet<(usize,usize)>,
// }

// impl Cave {
//     fn is_blocked(self, rock: Rock) {

//     }
// }

// #[derive(Debug)]
// struct Rock {
//     shape: RockForm,
//     left: usize,
//     y: usize
// }

// impl Rock {
//     fn push(&mut self, jet: char) {
//         atch jet {
//             '>' => self.left_incr(),
//             '<' => self.left_decr(),
//             _ => println!("Unknown jet")
//         }
//     }
// // this doesn't work because I'm only looking at the intersection of left
//     fn left_incr(&mut self) {
//         self.left = std::cmp::min(self.left + 1, 7);
//     }

//     fn left_decr(&mut self) {
//         self.left = std::cmp::max(self.left - 1, 0);
//     }

//     fn points(self) -> Vec<(usize, usize)> {
//         let mut points: Vec<(usize, usize)> = vec![];

//         for i in 0..self.shape.len() {
//             for j in 0..self.shape[i].len() {
//                 if self.shape[i][j] == 1 {
//                     points.push(
//                         (self.left+i, self.y+j)
//                     );
//                 }
//             }
//         }

//         points
//     }


// }

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
