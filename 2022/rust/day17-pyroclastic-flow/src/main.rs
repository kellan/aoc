use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    part1(input, 6);
}

fn part1(jets: String, cnt: usize) {
    let rock_forms = rocks();
    let jets: Vec<char> = jets.chars().collect();

    for i in 0..cnt {
        // drop rock
        
        let j = i;
        let jet = jets[ j % jets.len()];
        
        let mut current = Rock {
            shape: rock_forms[ i % rock_forms.len() ],
            left: 3,
            y: 0
        };
        current.push(jet);
        
        println!("{} {}", jet, &current.left);
    }
}

fn x_in_range(x: usize) -> usize {
    if x > 7 {
        return 7;
    } else if x < 0 {
        return 0;
    } else {
        return x;
    }
}

struct Cave {
    rocks: HashSet<(usize,usize)>,
}

impl Cave {
    fn is_blocked(self, rock: Rock) {

    }
}

#[derive(Debug)]
struct Rock {
    shape: RockForm,
    left: usize,
    y: usize
}

impl Rock {
    fn push(&mut self, jet: char) {
        match jet {
            '>' => self.left_incr(),
            '<' => self.left_decr(),
            _ => println!("Unknown jet")
        }
    }

    fn left_incr(&mut self) {
        self.left = std::cmp::min(self.left + 1, 7);
    }

    fn left_decr(&mut self) {
        self.left = std::cmp::max(self.left - 1, 0);
    }

    fn points(self) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = vec![];

        for i in 0..self.shape.len() {
            for j in 0..self.shape[i].len() {
                if self.shape[i][j] == 1 {
                    points.push(
                        (self.left+i, self.y+j)
                    );
                }
            }
        }

        points
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
    
//     let pieces: [Piece; 4] = [
//      [[1,1,1,1],
//      [0,0,0,0],
//      [0,0,0,0],
//      [0,0,0,0]],
     
    
//       [[0,0,1,0],
//       [0,0,1,0],
//       [1,1,1,0],
//       [0,0,0,0]],
     
//      [[1,0,0,0],
//       [1,0,0,0],
//       [1,0,0,0],
//       [1,0,0,0]],
 
//      [[1,1,0,0],
//       [1,1,0,0],
//       [0,0,0,0],
//       [0,0,0,0]],
//     ];

//     pieces
// }
fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}
