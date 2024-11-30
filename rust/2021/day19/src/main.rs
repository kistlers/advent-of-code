use std::collections::HashSet;
use elf::measure;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2021/day/19
const YEAR: u32 = 2021;
const DAY: u32 = 19;

const MIN_BEACON_OVERLAP: usize = 12;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let part1_count;
    let part2_distance;
    measure!({
        let mut scanners = parse_scanners(input_lines);

        find_scanner_alignment(&mut scanners);
        part1_count = part1(&scanners);
        part2_distance = part2(&scanners);
    });
    println!("Part1: {}", part1_count);
    println!("Part2: {}", part2_distance);
}

fn part1(scanners: &[Scanner]) -> usize {
    scanners.iter().map(|s| s.beacons.clone()).flatten().unique().count()
}

fn part2(scanners: &[Scanner]) -> i32 {
    scanners
        .iter()
        .map(|s| s.pos.unwrap())
        .permutations(2)
        .map(|two| two.into_iter().collect_tuple().unwrap())
        .map(|(left, right)| manhattan_distance(&left, &right))
        .max()
        .unwrap()
}

fn manhattan_distance(left: &(i32, i32, i32), right: &(i32, i32, i32)) -> i32 {
    let offset = pairwise_minus(left, right);
    [offset.0, offset.1, offset.2].iter().map(|x| x.abs()).sum()
}

fn find_scanner_alignment(scanners: &mut Vec<Scanner>) {
    let mut aligned_scanners = HashSet::from([0]);
    let mut next_to_check_against = Vec::from([0]);

    #[allow(clippy::never_loop)]
    while has_unaligned_candidate(scanners) {
        let check_against = scanners[next_to_check_against.pop().unwrap()].clone();

        'candidate: for unaligned_candidate in scanners.iter_mut().filter(|s| s.pos.is_none()) {
            for _ in 0..4 {
                unaligned_candidate.rotate_x_90();
                for _ in 0..4 {
                    unaligned_candidate.rotate_y_90();
                    for _ in 0..4 {
                        unaligned_candidate.rotate_z_90();

                        for check_against_coord in &check_against.beacons {
                            for unaligned_candidate_coord in &unaligned_candidate.beacons {
                                let potential_scanner_offset = pairwise_minus(check_against_coord, unaligned_candidate_coord);

                                let potential_align_candidate_coordinates = unaligned_candidate.beacons
                                    .clone()
                                    .iter()
                                    .map(|check_against| pairwise_plus(check_against, &potential_scanner_offset))
                                    .collect::<HashSet<_>>();

                                if potential_align_candidate_coordinates.intersection(&check_against.beacons).count() >= MIN_BEACON_OVERLAP {
                                    unaligned_candidate.pos = Some(potential_scanner_offset);
                                    unaligned_candidate.beacons = potential_align_candidate_coordinates;
                                    aligned_scanners.insert(unaligned_candidate.id);
                                    next_to_check_against.push(unaligned_candidate.id);

                                    continue 'candidate;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn pairwise_minus(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    let mut result = *a;
    result.0 = a.0 - b.0;
    result.1 = a.1 - b.1;
    result.2 = a.2 - b.2;
    result
}

fn pairwise_plus(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    let mut result = *a;
    result.0 = a.0 + b.0;
    result.1 = a.1 + b.1;
    result.2 = a.2 + b.2;
    result
}

fn has_unaligned_candidate(scanners: &mut std::vec::Vec<Scanner>) -> bool {
    scanners.iter_mut().any(|scanner| scanner.pos.is_none())
}

fn parse_scanners(input_lines: Vec<String>) -> Vec<Scanner> {
    let mut scanners = input_lines
        .split(|l| l.is_empty())
        .enumerate()
        .map(|(scanner_id, scanner_lines)| {
            let beacons = scanner_lines[1..]
                .iter()
                .map(|line| line
                    .splitn(3, ',')
                    .map(|coord| coord.parse().unwrap())
                    .collect_tuple::<(i32, i32, i32)>()
                    .unwrap()
                ).collect::<HashSet<_>>();
            Scanner::new(scanner_id, beacons)
        }).collect::<Vec<_>>();

    scanners[0].pos = Some((0, 0, 0));
    scanners
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scanner {
    id: usize,
    beacons: HashSet<(i32, i32, i32)>,
    pos: Option<(i32, i32, i32)>,
}

impl Scanner {
    fn new(id: usize, beacons: HashSet<(i32, i32, i32)>) -> Scanner {
        Self {
            id,
            beacons,
            pos: None,
        }
    }

    fn rotate_x_90(&mut self) {
        self.beacons = self.beacons.iter().map(|c| (c.0, -c.2, c.1)).collect();
    }

    fn rotate_y_90(&mut self) {
        self.beacons = self.beacons.iter().map(|c| (c.2, c.1, -c.0)).collect();
    }

    fn rotate_z_90(&mut self) {
        self.beacons = self.beacons.iter().map(|c| (-c.1, c.0, c.2)).collect();
    }
}
