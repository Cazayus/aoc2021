use std::collections::HashMap;

use crate::Amphipod::*;

const DATA: &str = include_str!("../inputs/day23.txt");
const DATA_B: &str = include_str!("../inputs/day23b.txt");
const PART_ONE_ROOM_SIZE: usize = 2;
const PART_TWO_ROOM_SIZE: usize = 4;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost_multiplier(self) -> usize {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn room_id(self) -> usize {
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
}

type Room = Vec<Amphipod>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Option<Amphipod>; 7],
    rooms: [Room; 4],
}

impl State {
    fn room(&self, room_type: Amphipod) -> &Room {
        &self.rooms[room_type.room_id()]
    }

    fn room_mut(&mut self, room_type: Amphipod) -> &mut Room {
        &mut self.rooms[room_type.room_id()]
    }

    fn room_ready(&self, room_type: Amphipod) -> bool {
        self.room(room_type).iter().all(|&a| a == room_type)
    }

    fn steps_between(
        &self,
        hall_id: usize,
        room_type: Amphipod,
        room_size: usize,
    ) -> Option<usize> {
        let room_id = room_type.room_id();
        let mut steps_taken = 1;
        for step in (hall_id + 1..room_id + 2).chain(room_id + 2..hall_id) {
            if self.hallway[step].is_some() {
                return None;
            }
            steps_taken += 2;
        }
        steps_taken += room_size - self.room(room_type).len();
        steps_taken += self.hallway[hall_id].is_none() as usize;
        steps_taken -= matches!(hall_id, 0 | 6) as usize;
        Some(steps_taken)
    }

    fn move_between(
        &self,
        hall_id: usize,
        room_type: Amphipod,
        room_size: usize,
    ) -> Option<(Self, usize)> {
        match self.hallway[hall_id] {
            Some(a) => {
                if a != room_type
                    || self.room(room_type).len() == room_size
                    || !self.room_ready(room_type)
                {
                    None
                } else {
                    let steps_taken = self.steps_between(hall_id, room_type, room_size)?;
                    let mut new_state = self.clone();
                    new_state.hallway[hall_id] = None;
                    new_state.room_mut(room_type).push(room_type);
                    Some((new_state, steps_taken * room_type.cost_multiplier()))
                }
            }
            None => {
                if self.room(room_type).is_empty() || self.room_ready(room_type) {
                    None
                } else {
                    let steps_taken = self.steps_between(hall_id, room_type, room_size)?;
                    let mut new_state = self.clone();
                    let move_amphipod = new_state.room_mut(room_type).pop().unwrap();
                    new_state.hallway[hall_id] = Some(move_amphipod);
                    Some((new_state, steps_taken * move_amphipod.cost_multiplier()))
                }
            }
        }
    }
}

fn total_cost(state: State, memo: &mut HashMap<State, usize>, room_size: usize) -> usize {
    if let Some(cost) = memo.get(&state) {
        return *cost;
    }
    if [A, B, C, D]
        .into_iter()
        .all(|room_type| state.room(room_type).len() == room_size && state.room_ready(room_type))
    {
        return 0;
    }
    let cost = (0..7)
        .into_iter()
        .flat_map(|hall_id| {
            [A, B, C, D]
                .into_iter()
                .map(move |room_type| (hall_id, room_type))
        })
        .filter_map(|(hall_id, room_type)| state.move_between(hall_id, room_type, room_size))
        .map(|(new_state, move_cost)| {
            move_cost.saturating_add(total_cost(new_state, memo, room_size))
        })
        .min()
        .unwrap_or(usize::MAX);
    memo.insert(state, cost);
    cost
}

fn parse(data: &str) -> State {
    let lns = data.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let hallway = [None; 7];
    let parse_amphipod = |c| match c {
        b'A' => A,
        b'B' => B,
        b'C' => C,
        b'D' => D,
        _ => panic!(),
    };
    if lns.len() > 6 {
        let rooms = [
            [lns[5][3], lns[4][3], lns[3][3], lns[2][3]],
            [lns[5][5], lns[4][5], lns[3][5], lns[2][5]],
            [lns[5][7], lns[4][7], lns[3][7], lns[2][7]],
            [lns[5][9], lns[4][9], lns[3][9], lns[2][9]],
        ]
        .map(|byte_room| byte_room.map(parse_amphipod).into());
        State { hallway, rooms }
    } else {
        let rooms = [
            [lns[3][3], lns[2][3]],
            [lns[3][5], lns[2][5]],
            [lns[3][7], lns[2][7]],
            [lns[3][9], lns[2][9]],
        ]
        .map(|byte_room| byte_room.map(parse_amphipod).into());
        State { hallway, rooms }
    }
}

fn main() {
    println!("part 1: {}", solve(DATA, PART_ONE_ROOM_SIZE));
    println!("part 2: {}", solve(DATA_B, PART_TWO_ROOM_SIZE));
}

fn solve(data: &str, room_size: usize) -> usize {
    total_cost(parse(data), &mut HashMap::new(), room_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day23_sample.txt");
    const SAMPLE_DATA_B: &str = include_str!("../inputs/day23b_sample.txt");

    #[test]
    fn sample() {
        let state = parse(SAMPLE_DATA);
        let (state, cost) = state.move_between(2, C, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 40);
        let (state, cost) = state.move_between(3, B, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 200);
        let (state, cost) = state.move_between(3, C, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 200);
        let (state, cost) = state.move_between(3, B, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 3000);
        let (state, cost) = state.move_between(2, B, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 30);
        let (state, cost) = state.move_between(2, A, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 20);
        let (state, cost) = state.move_between(2, B, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 20);
        let (state, cost) = state.move_between(4, D, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 2000);
        let (state, cost) = state.move_between(5, D, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 3);
        let (state, cost) = state.move_between(4, D, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 3000);
        let (state, cost) = state.move_between(3, D, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 4000);
        let (solved_state, cost) = state.move_between(5, A, PART_ONE_ROOM_SIZE).unwrap();
        assert_eq!(cost, 8);
        let total_cost = total_cost(solved_state, &mut HashMap::new(), PART_ONE_ROOM_SIZE);
        assert_eq!(total_cost, 0);
        assert_eq!(solve(SAMPLE_DATA, PART_ONE_ROOM_SIZE), 12521);
        assert_eq!(solve(SAMPLE_DATA_B, PART_TWO_ROOM_SIZE), 44169);
    }

    #[test]
    fn case() {
        assert_eq!(solve(DATA, PART_ONE_ROOM_SIZE), 15412);
        assert_eq!(solve(DATA_B, PART_TWO_ROOM_SIZE), 52358);
    }
}
