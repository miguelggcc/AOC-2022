use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub fn part1(input: &str) -> u32 {
    get_energy::<8>(&input)
}

pub fn part2(input: &str) -> u32 {
    let (i1, i2) = input.split_once("  ").unwrap();
    let insert = "  #D#C#B#A#\n  #D#B#A#C#\n  ";
    get_energy::<16>(&[i1, insert, i2].join(""))
}

fn get_energy<const R: usize>(input: &str) -> u32 {
    let mut rooms = [vec![], vec![], vec![], vec![]];
    let mut cache = HashMap::new();
    let dist = get_distances();
    input[14 * 2..].lines().rev().skip(1).for_each(|line| {
        line.trim()
            .split('#')
            .filter(|s| !s.is_empty())
            .enumerate()
            .for_each(|(i, a)| rooms[i].push(a.chars().next().unwrap() as u8 - b'A' + 1))
    });
    let rooms = rooms.into_iter().flatten().collect::<Vec<_>>();
    let mut new_states = vec![];
    let mut heap: BinaryHeap<State<R>> = BinaryHeap::from([State::new(rooms.try_into().unwrap())]);

    while let Some(state) = heap.pop() {
        if state
            .rooms
            .chunks(R / 4)
            .enumerate()
            .all(|(i, room)| room.iter().all(|&ar| ar == i as u8 + 1))
        {
            return state.energy;
        }
        state.new_states(&mut new_states, &dist);
        for new_state in new_states.drain(..) {
            let c = cache.get(&new_state.get_key()).unwrap_or(&u32::MAX);
            if c > &new_state.energy {
                cache.insert(new_state.get_key(), new_state.energy);
                heap.push(new_state);
            }
        }
    }
    panic!("rearrangement not found")
}

#[derive(Clone, Eq, PartialEq)]
struct State<const R: usize> {
    rooms: [u8; R],
    hall: [u8; 7],
    energy: u32,
}

impl<const R: usize> State<R> {
    fn new(rooms: [u8; R]) -> Self {
        Self {
            rooms,
            hall: [0; 7],
            energy: 0,
        }
    }
    fn new_states(self, new_states: &mut Vec<Self>, dist: &DISTS) {
        let l = R / 4;
        for (i, a) in self.hall.iter().enumerate().filter(|(_, a)| **a != 0) {
            let index = l * (*a as usize - 1);
            if self.rooms[index..index + l]
                .iter()
                .all(|ar| ar == a || ar == &0)
            {
                if let Some(distance) = move_room(i, *a as usize, &self.hall, dist) {
                    let mut new_state = self.clone();
                    let zero = new_state.rooms[index..index + l]
                        .iter()
                        .position(|&ar| ar == 0)
                        .unwrap();
                    std::mem::swap(&mut new_state.rooms[index + zero], &mut new_state.hall[i]);
                    new_state.energy +=
                        ((l - zero - 1) as u32 + distance) * 10u32.pow(*a as u32 - 1);
                    new_states.push(new_state)
                }
            }
        }
        for (i, room) in self.rooms.chunks(l).enumerate() {
            if room.iter().any(|&a| a != 0 && a != i as u8 + 1) {
                for (pos, distance) in move_row(i, &self.hall, dist) {
                    let mut new_state = self.clone();
                    let last = room.iter().rposition(|&ar| ar != 0).unwrap();
                    let a = new_state.rooms[l * i + last];
                    std::mem::swap(&mut new_state.hall[pos], &mut new_state.rooms[l * i + last]);
                    new_state.energy +=
                        ((l - 1 - last) as u32 + distance) * 10u32.pow(a as u32 - 1);
                    new_states.push(new_state)
                }
            }
        }
    }
    fn get_key(&self) -> u64 {
        self.rooms
            .iter()
            .chain(&self.hall)
            .fold(0, |acc, &n| acc * 5 + n as u64)
    }
}

impl<const R: usize> Ord for State<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl<const R: usize> PartialOrd for State<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
type DISTS = [[u32; 7]; 4];
fn get_distances() -> DISTS {
    let rooms: [u32; 4] = [2, 4, 6, 8];
    rooms
        .iter()
        .map(|room| {
            (0..11)
                .filter(|i| rooms.binary_search(i).is_err())
                .map(|i| 1 + room.abs_diff(i))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
fn move_row<'a>(
    r: usize,
    hall: &'a [u8],
    dist: &'a DISTS,
) -> impl Iterator<Item = (usize, u32)> + 'a {
    let left = hall[..r + 2]
        .iter()
        .rposition(|&ar| ar != 0)
        .unwrap_or(usize::MAX);
    hall.iter()
        .enumerate()
        .skip(left.wrapping_add(1))
        .take_while(|(_, ar)| **ar == 0)
        .map(move |(i, _)| (i, dist[r][i]))
}
fn move_room(i0: usize, a: usize, hall: &[u8], dist: &DISTS) -> Option<u32> {
    if hall[(a + 1).min(i0 + 1)..i0.max(a + 1)]
        .iter()
        .any(|&ar| ar > 0)
    {
        return None;
    }
    Some(dist[a - 1][i0])
}

#[cfg(test)]
mod day23 {

    use super::*;

    const INPUT: &'static str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 12521);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 44169);
    }
}
