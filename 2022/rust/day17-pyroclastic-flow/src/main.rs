use std::io::{self, Read};

fn main() {
    //println!("Hello, world!");
    let input = read_stdin();
    //let pieces = pieces();
    //dbg!(pieces);
    //dbg!(piece);
    part1(input, 6);
}

fn part1(jets: String, cnt: usize) {
    let pieces = pieces();
    let jets: Vec<char> = jets.chars().collect();

    for i in 0..cnt {
        // drop rock
        let rock = pieces[ i % pieces.len()];
        let jet = jets[ i % jets.len()];
        dbg!(rock, jet);
    }
}




type Piece = [[u8;4];4];

fn pieces() -> [Piece; 5] {
    let pieces: [Piece; 5] = [
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

    return pieces;
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
