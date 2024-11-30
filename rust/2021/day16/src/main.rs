use std::cmp::{max, min};
use std::fmt::Error;
use std::ops::Range;
use std::str::FromStr;

// Code for the problem https://adventofcode.com/2021/day/16
const YEAR: u32 = 2021;
const DAY: u32 = 16;

const VERSION_LEN: usize = 3;
const TYPE_LEN: usize = 3;

const SUB_PACKET_BITS_ID: &str = "0";
const IS_SUB_PACKET_BITS: bool = true;
const SUB_PACKET_BITS_LEN: usize = 15;

// const NUM_SUB_PACKETS_ID: &str = "1";
const IS_NUM_SUB_PACKETS: bool = false;
const NUM_SUB_PACKETS_LEN: usize = 11;

const LITERAL_CHUNK_LEN: usize = 5;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let input = input_lines[0].chars().map(|x| format!("{:04b}", x.to_digit(16).unwrap())).collect::<Vec<_>>().join("");

    let mut input_index = 0usize;
    let packet = parse_to_packet(&input, &mut input_index);
    println!("Part1: {}", packet.get_version_sum());
    println!("Part2: {}", packet.get_result());
}

fn binary_to_int(bin: &str) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

#[derive(Debug, Clone, Copy)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

impl FromStr for PacketType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match binary_to_int(s) {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::Min),
            3 => Ok(PacketType::Max),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GreaterThan),
            6 => Ok(PacketType::LessThan),
            7 => Ok(PacketType::Equal),
            o => panic!("unexpected packet type '{}'", o)
        }
    }
}

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    packet_type: PacketType,
    literal_value: usize,
    is_sub_packet_bits: bool,
    len: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub(crate) fn get_result(&self) -> usize {
        match self.packet_type {
            PacketType::Literal => self.literal_value,
            PacketType::Sum =>
                self.sub_packets.iter().fold(0, |sum, p| sum + p.get_result()),
            PacketType::Product =>
                self.sub_packets.iter().fold(1, |prod, p| prod * p.get_result()),
            PacketType::Min =>
                self.sub_packets.iter().fold(usize::MAX, |curr, p| min(curr, p.get_result())),
            PacketType::Max =>
                self.sub_packets.iter().fold(usize::MIN, |curr, p| max(curr, p.get_result())),
            PacketType::GreaterThan =>
                if self.sub_packets[0].get_result() > self.sub_packets[1].get_result() { 1 } else { 0 },
            PacketType::LessThan =>
                if self.sub_packets[0].get_result() < self.sub_packets[1].get_result() { 1 } else { 0 },
            PacketType::Equal =>
                if self.sub_packets[0].get_result() == self.sub_packets[1].get_result() { 1 } else { 0 }
        }
    }
}

impl Packet {
    pub(crate) fn get_version_sum(&self) -> usize {
        self.version + self.sub_packets
            .iter()
            .fold(0, |sum, p| sum + p.get_version_sum())
    }
}

fn parse_to_packet(input: &str, input_index: &mut usize) -> Packet {
    let version = binary_to_int(&input[range_and_increment(input_index, VERSION_LEN)]);
    let packet_type: PacketType = input[range_and_increment(input_index, TYPE_LEN)].parse().unwrap();
    match packet_type {
        PacketType::Literal => {
            let literal_value = parse_literal_value(input, input_index);
            Packet {
                version,
                packet_type,
                literal_value,
                is_sub_packet_bits: false,
                len: 0,
                sub_packets: vec![],
            }
        }
        _ => {
            let (is_sub_packet_bits, len) = parse_operator_packet(input, input_index);
            Packet {
                version,
                packet_type,
                literal_value: 0,
                is_sub_packet_bits,
                len,
                sub_packets: parse_sub_packets(input, input_index, is_sub_packet_bits, len),
            }
        }
    }
}

fn parse_sub_packets(input: &str, input_index: &mut usize, is_sub_packet_bits: bool, len: usize) -> Vec<Packet> {
    let mut packets = vec![];
    let start_index = *input_index;
    if is_sub_packet_bits {
        loop {
            packets.push(parse_to_packet(input, input_index));
            if *input_index - start_index == len {
                break;
            } else if *input_index - start_index > len {
                panic!();
            }
        }
    } else {
        for _ in 0..len {
            packets.push(parse_to_packet(input, input_index));
        }
    }
    packets
}

fn parse_operator_packet(input: &str, input_index: &mut usize) -> (bool, usize) {
    if &input[range_and_increment(input_index, 1)] == SUB_PACKET_BITS_ID {
        let len_sub_packet_bits = binary_to_int(&input[range_and_increment(input_index, SUB_PACKET_BITS_LEN)]);
        (IS_SUB_PACKET_BITS, len_sub_packet_bits)
    } else {
        let num_sub_packets = binary_to_int(&input[range_and_increment(input_index, NUM_SUB_PACKETS_LEN)]);
        (IS_NUM_SUB_PACKETS, num_sub_packets)
    }
}

fn range_and_increment(input_index: &mut usize, num_bits: usize) -> Range<usize> {
    let range = *input_index..(*input_index + num_bits);
    *input_index += num_bits;
    range
}

fn parse_literal_value(input: &str, input_index: &mut usize) -> usize {
    let mut literal_value = "".to_string();
    loop {
        let five = &input[range_and_increment(input_index, LITERAL_CHUNK_LEN)];
        literal_value.push_str(&five[1..5]);
        if five.starts_with('0') {
            break;
        }
    }
    binary_to_int(&literal_value)
}

