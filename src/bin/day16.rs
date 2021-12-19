use std::vec::IntoIter;

const DATA: &str = include_str!("../inputs/day16.txt");

struct Result {
    version_sum: usize,
    computed_value: usize,
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let bits = parse_data(data);
    let result = parse_result(bits.into_iter().by_ref());
    result.version_sum
}

fn part_two(data: &str) -> usize {
    let bits = parse_data(data);
    let result = parse_result(bits.into_iter().by_ref());
    result.computed_value
}

fn parse_data(data: &str) -> Vec<bool> {
    data.chars()
        .flat_map(|char| {
            format!("{:04b}", char.to_digit(16).unwrap())
                .chars()
                .map(|c| c == '1')
                .collect::<Vec<_>>()
        })
        .collect()
}

fn convert_bits_to_int(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| (b as usize) << i)
        .sum()
}

fn convert_bits_iter_to_int(bits: impl Iterator<Item = bool>) -> usize {
    convert_bits_to_int(&bits.collect::<Vec<_>>())
}

fn parse_result(bits: &mut IntoIter<bool>) -> Result {
    let mut version_sum = convert_bits_iter_to_int(bits.take(3));
    let type_id = convert_bits_iter_to_int(bits.take(3));
    if type_id == 4 {
        let mut literal_bin = Vec::new();
        let mut read_next = true;
        while read_next {
            read_next = bits.next().unwrap();
            literal_bin.extend(bits.take(4));
        }
        Result {
            version_sum,
            computed_value: convert_bits_to_int(&literal_bin),
        }
    } else {
        let length_type_id = bits.next().unwrap();
        let (nb_subpacket, num_bits) = if length_type_id {
            (convert_bits_iter_to_int(bits.take(11)), usize::MAX)
        } else {
            (usize::MAX, convert_bits_iter_to_int(bits.take(15)))
        };

        let bits_left = bits.len();
        let mut sub_values = Vec::new();
        while (bits_left - bits.len()) < num_bits && sub_values.len() < nb_subpacket {
            let result = parse_result(bits);
            version_sum += result.version_sum;
            sub_values.push(result.computed_value);
        }
        let computed_value = match type_id {
            0 => sub_values.iter().sum::<usize>(),
            1 => sub_values.iter().product::<usize>(),
            2 => *sub_values.iter().min().unwrap(),
            3 => *sub_values.iter().max().unwrap(),
            5 => (sub_values[0] > sub_values[1]) as usize,
            6 => (sub_values[0] < sub_values[1]) as usize,
            7 => (sub_values[0] == sub_values[1]) as usize,
            _ => panic!(),
        };
        Result {
            version_sum,
            computed_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one("8A004A801A8002F478"), 16);
        assert_eq!(part_one("620080001611562C8802118E34"), 12);
        assert_eq!(part_one("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_one("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two("C200B40A82"), 3);
        assert_eq!(part_two("04005AC33890"), 54);
        assert_eq!(part_two("880086C3E88112"), 7);
        assert_eq!(part_two("CE00C43D881120"), 9);
        assert_eq!(part_two("D8005AC2A8F0"), 1);
        assert_eq!(part_two("F600BC2D8F"), 0);
        assert_eq!(part_two("9C005AC2F8F0"), 0);
        assert_eq!(part_two("9C0141080250320F1802104A08"), 1);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 889);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 739303923668);
    }
}
