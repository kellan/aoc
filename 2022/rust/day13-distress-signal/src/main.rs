use serde::Deserialize;
use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Int(u32),
    List(Vec<Node>),
}

fn main() {
    let input = read_stdin();
    let sum = solve1(input);
    dbg!(sum);
}

fn solve1(input: String) -> usize {
    let mut sum = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let pair: Vec<Node> = pair
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        match packet_order(&pair[0], &pair[1]) {
            Ordering::Greater => {}
            Ordering => {
                sum += i + 1;
            }
        }
    }

    sum
}

fn packet_order(a: &Node, b: &Node) -> Ordering {
    match (a, b) {
        (Node::Int(a), Node::Int(b)) => a.cmp(b),
        (Node::List(a), Node::List(b)) => {
            //dbg!(&a, &b);
            let length = std::cmp::max(a.len(), b.len());

            for i in 0..length {
                if i >= b.len() {
                    // a is longer than b, wrong order
                    return Ordering::Greater;
                }
                if i >= a.len() {
                    // b is longer, right order
                    return Ordering::Less;
                }

                let order = packet_order(&a[i], &b[i]);
                if order == Ordering::Equal {
                    continue;
                } else {
                    return order;
                }
            }
            return Ordering::Equal;
        }
        (Node::List(a), Node::Int(b)) => {
            let list = Node::List(vec![Node::Int(*b)]);
            return packet_order(&Node::List(a.to_vec()), &list);
        }
        (Node::Int(a), Node::List(b)) => {
            let list = Node::List(vec![Node::Int(*a)]);
            return packet_order(&list, &Node::List(b.to_vec()));
        }
        _ => Ordering::Equal,
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_test_order(packet1: &str, packet2: &str, expected_order: Ordering) {
        let a = serde_json::from_str(packet1).unwrap();
        let b = serde_json::from_str(packet2).unwrap();
        let order: Ordering = packet_order(&a, &b);
        assert_eq!(order, expected_order, "{} cmp {}", &packet1, &packet2);
    }

    #[test]
    fn test_packet_order_simple() {
        assert_test_order("3", "4", Ordering::Less);
        assert_test_order("4", "4", Ordering::Equal);
        assert_test_order("[4]", "[4]", Ordering::Equal);
        assert_test_order("3", "4", Ordering::Less);
        assert_test_order("4", "3", Ordering::Greater);
        assert_test_order("[4]", "[3]", Ordering::Greater);
    }

    #[test]
    fn test_packet_order_lists() {
        assert_test_order("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less);
        assert_test_order("[1,1,5,1,1]", "[1,1,3,1,1]", Ordering::Less);
        assert_test_order("[9]", "[8,7,6]", Ordering::Greater);
    }

    #[test]
    fn test_packet_order_staggered() {
        assert_test_order("[[3]]", "4", Ordering::Less);
        assert_test_order("[[4]]", "3", Ordering::Greater);
        assert_test_order("[[3]]", "[4]", Ordering::Less);
        assert_test_order("3", "[[4]]", Ordering::Less);
    }

    #[test]
    fn test_packet_order_nested() {
        assert_test_order("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less);
        assert_test_order("[[4,4],4,4,4]", "[[4,4],4,4]", Ordering::Greater);
        assert_test_order(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            Ordering::Greater,
        );
        assert_test_order("[]", "[3]", Ordering::Less);
        assert_test_order("[[[]]]", "[[]]", Ordering::Greater);
    }

    #[test]
    fn test_edge_case() {
        assert_test_order("[[1],[2,3,4]]", "[[1],4]", Ordering::Less);
    }
    #[test]
    fn test_packet_order_test_data_ok() {
        let data = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[4,4],4,4]
[[4,4],4,4,4]

[]
[3]"#;

        for (i, input) in data.split("\n\n").enumerate() {
            let pair: Vec<Node> = input
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();

            assert_eq!(
                Ordering::Less,
                packet_order(&pair[0], &pair[1]),
                "testing {}",
                input
            );
        }
    }

    #[test]
    fn test_packet_order_test_data_not_ok() {
        let data = r#"[9]
[[8,7,6]]

[7,7,7,7]
[7,7,7]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        for (i, input) in data.split("\n\n").enumerate() {
            let pair: Vec<&str> = input.lines().collect();
            assert_test_order(&pair[0], &pair[1], Ordering::Greater);
        }
    }
}
