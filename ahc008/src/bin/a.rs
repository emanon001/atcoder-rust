#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Duration, Instant};
use whiteread::parse_line;

pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Human {
    i: usize,
    j: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pet {
    i: usize,
    j: usize,
    t: PetType,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum PetType {
    Ushi,
    Buta,
    Usagi,
    Inu,
    Neko,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cell {
    None,
    Blocked,
    HumanOrPet(usize, usize),
}

impl From<usize> for PetType {
    fn from(t: usize) -> Self {
        match t {
            1 => Self::Ushi,
            2 => Self::Buta,
            3 => Self::Usagi,
            4 => Self::Inu,
            5 => Self::Neko,
            _ => unreachable!(),
        }
    }
}

struct Solver {
    n: usize,
    pets: Vec<Pet>,
    m: usize,
    humans: Vec<Human>,
    all_turn: usize,
    cur_turn: usize,
    done_move_to_initial_positon: bool,
    done_move_and_block_cells: bool,
    random_turn_cout: usize,
    h_size: usize,
    v_size: usize,
    cells: Vec<Vec<Cell>>,
    rng: ThreadRng,
    start_time: Instant,
    random_start_time: Instant,
    random_duration_per_turn: Duration,
    max_random_turn: usize,
}

impl Solver {
    fn new(pets: Vec<(usize, usize, usize)>, humans: Vec<(usize, usize)>) -> Self {
        let mut cells = vec![vec![Cell::None; 30]; 30];
        for pet in &pets {
            cells[pet.0][pet.1] = Cell::HumanOrPet(0, 1);
        }
        for human in &humans {
            cells[human.0][human.1] = Cell::HumanOrPet(1, 0);
        }
        let start_time = Instant::now();
        Solver {
            n: pets.len(),
            pets: pets
                .into_iter()
                .map(|p| Pet {
                    i: p.0,
                    j: p.1,
                    t: p.2.into(),
                })
                .collect(),
            m: humans.len(),
            humans: humans
                .into_iter()
                .map(|h| Human { i: h.0, j: h.1 })
                .collect(),
            all_turn: 300,
            cur_turn: 1,
            done_move_to_initial_positon: false,
            done_move_and_block_cells: false,
            random_turn_cout: 0,
            h_size: 30,
            v_size: 30,
            cells,
            rng: rand::thread_rng(),
            start_time,
            random_start_time: Instant::now(),
            random_duration_per_turn: Duration::from_millis(0),
            max_random_turn: 10,
        }
    }

    fn solve(&mut self) {
        for turn in 1..=self.all_turn {
            self.cur_turn = turn;

            let human_actions = if !self.done_move_to_initial_positon {
                self.move_to_initial_position()
            } else if !self.done_move_and_block_cells {
                self.move_and_block_cells()
            } else if self.random_turn_cout < self.max_random_turn {
                if self.random_turn_cout == 0 {
                    self.random_start_time = Instant::now();
                    let duration =
                        self.start_time + Duration::from_millis(2850) - self.random_start_time;
                    self.random_duration_per_turn = duration / self.max_random_turn as u32;
                }
                self.random_actions()
            } else {
                vec!['.'; self.m]
            };
            println!("{}", human_actions.iter().join(""));
            let pet_actions: Vec<Vec<char>> = read_line()
                .split(" ")
                .map(|s| s.chars().into_iter().collect::<Vec<char>>())
                .collect();
            self.move_pets_by_actions(&pet_actions);
        }
    }

    fn move_to_initial_position(&mut self) -> Vec<char> {
        let h_span = self.h_size / self.m;
        let mut actions = Vec::new();
        let mut done_move = true;
        for i in 0..self.m {
            let human = &self.humans[i];
            let act = if human.i > 0 {
                'U'
            } else if human.j != i * h_span + h_span - 1 {
                if human.j < i * h_span + h_span - 1 {
                    'R'
                } else {
                    'L'
                }
            } else {
                '.'
            };
            if act != '.' {
                done_move = false;
            }
            actions.push(act);
        }
        if done_move {
            self.done_move_to_initial_positon = true;
        }
        self.apply_humans_actions(&actions);
        actions
    }

    fn move_and_block_cells(&mut self) -> Vec<char> {
        let mut actions = Vec::new();
        for i in 0..self.m {
            let human = &self.humans[i];
            if human.j < 3 || self.h_size - human.j < 3 {
                actions.push('.');
                continue;
            }
            let act = match self.cells[human.i][human.j - 1] {
                Cell::Blocked if human.i < self.v_size - 1 => 'D',
                Cell::None
                    if human.i < self.v_size - 1
                        && self.can_block_cell(human.i, human.j - 1, &self.cells) =>
                {
                    'l'
                }
                _ => '.',
            };
            actions.push(act);
        }
        self.apply_humans_actions(&actions);
        let mut done = true;
        for i in 0..self.m {
            let human = &self.humans[i];
            if human.j < 3 || self.h_size - human.j < 3 {
                continue;
            }
            if human.i < self.v_size - 1 {
                done = false;
            }
        }
        if done {
            self.done_move_and_block_cells = true;
        }
        actions
    }

    fn random_actions(&mut self) -> Vec<char> {
        self.random_turn_cout += 1;
        let depth = 4;
        let best_actions = vec!['.'; self.m];
        let (mut best_score, mut best_actions, bk_humans, bk_cells) =
            self.calc_score(&best_actions);
        let action_list = vec!['L', 'R', 'u', 'l', 'r', '.'];
        let mut trial_count = 0;
        let mut now = Instant::now();
        let end_time =
            self.random_start_time + self.random_duration_per_turn * self.random_turn_cout as u32;
        while now < end_time {
            trial_count += 1;
            let mut first_actions = Vec::new();
            for _ in 0..self.m {
                first_actions.push(action_list[self.rng.gen::<usize>() % action_list.len()]);
            }
            let mut bscore = 0_f64;
            let (score, first_actions, _, _) = self.calc_score(&first_actions);
            if score > bscore {
                bscore = score;
            }
            for _ in 0..depth - 1 {
                let mut actions = Vec::new();
                for _ in 0..self.m {
                    actions.push(action_list[self.rng.gen::<usize>() % action_list.len()]);
                }
                let (score, _actions, _, _) = self.calc_score(&actions);
                if score > bscore {
                    bscore = score;
                }
            }
            if bscore > best_score {
                best_score = bscore;
                best_actions = first_actions;
            }
            self.humans = bk_humans.clone();
            self.cells = bk_cells.clone();
            if trial_count % 50 == 0 {
                now = Instant::now();
            }
        }
        self.apply_humans_actions(&best_actions);
        best_actions
    }

    fn apply_humans_actions(&mut self, actions: &Vec<char>) {
        for i in 0..self.m {
            self.move_human_by_action(i, actions[i]);
        }
    }

    fn move_human_by_action(&mut self, h_i: usize, action: char) {
        let human = &mut self.humans[h_i];
        let mut new_i = human.i;
        let mut new_j = human.j;
        match action {
            'U' => new_i -= 1,
            'D' => new_i += 1,
            'L' => new_j -= 1,
            'R' => new_j += 1,
            'u' => self.cells[human.i - 1][human.j] = Cell::Blocked,
            'd' => self.cells[human.i + 1][human.j] = Cell::Blocked,
            'l' => self.cells[human.i][human.j - 1] = Cell::Blocked,
            'r' => self.cells[human.i][human.j + 1] = Cell::Blocked,
            _ => {}
        }

        // セルの状態更新
        if let Cell::HumanOrPet(c, c2) = self.cells[human.i][human.j] {
            // 移動元
            if c == 1 && c2 == 0 {
                self.cells[human.i][human.j] = Cell::None;
            } else {
                self.cells[human.i][human.j] = Cell::HumanOrPet(c - 1, c2);
            }
            // 移動先
            if let Cell::HumanOrPet(c, c2) = self.cells[new_i][new_j] {
                self.cells[new_i][new_j] = Cell::HumanOrPet(c + 1, c2);
            } else {
                self.cells[new_i][new_j] = Cell::HumanOrPet(1, 0);
            }
        } else {
            unreachable!();
        }

        human.i = new_i;
        human.j = new_j;
    }

    fn move_pets_by_actions(&mut self, actions: &Vec<Vec<char>>) {
        for i in 0..self.n {
            let pet = &mut self.pets[i];
            let pet_acts = &actions[i];
            let mut new_i = pet.i;
            let mut new_j = pet.j;
            for act in pet_acts {
                match act {
                    'U' => new_i -= 1,
                    'D' => new_i += 1,
                    'L' => new_j -= 1,
                    'R' => new_j += 1,
                    _ => {}
                };
            }
            if let Cell::HumanOrPet(c, c2) = self.cells[pet.i][pet.j] {
                // 移動元
                if c == 0 && c2 == 1 {
                    self.cells[pet.i][pet.j] = Cell::None;
                } else {
                    self.cells[pet.i][pet.j] = Cell::HumanOrPet(c, c2 - 1);
                }
                // 移動先
                if let Cell::HumanOrPet(c, c2) = self.cells[new_i][new_j] {
                    self.cells[new_i][new_j] = Cell::HumanOrPet(c, c2 + 1);
                } else {
                    self.cells[new_i][new_j] = Cell::HumanOrPet(0, 1);
                }
            } else {
                unreachable!();
            }

            pet.i = new_i;
            pet.j = new_j;
        }
    }

    fn can_action(&self, h: &Human, act: char, cells: &Vec<Vec<Cell>>) -> bool {
        match act {
            'U' => h.i > 0 && self.can_move(h.i - 1, h.j, cells),
            'D' => h.i < self.v_size - 1 && self.can_move(h.i + 1, h.j, cells),
            'L' => h.j > 0 && self.can_move(h.i, h.j - 1, cells),
            'R' => h.j < self.h_size - 1 && self.can_move(h.i, h.j + 1, cells),
            'u' => h.i > 0 && self.can_block_cell(h.i - 1, h.j, cells),
            'd' => h.i < self.v_size - 1 && self.can_block_cell(h.i + 1, h.j, cells),
            'l' => h.j > 0 && self.can_block_cell(h.i, h.j - 1, cells),
            'r' => h.j < self.h_size - 1 && self.can_block_cell(h.i, h.j + 1, cells),
            _ => false,
        }
    }

    fn can_move(&self, i: usize, j: usize, cells: &Vec<Vec<Cell>>) -> bool {
        match cells[i][j] {
            Cell::None | Cell::HumanOrPet(_, _) => true,
            _ => false,
        }
    }

    fn can_block_cell(&self, i: usize, j: usize, cells: &Vec<Vec<Cell>>) -> bool {
        let cell = &cells[i][j];
        if cell != &Cell::None && cell != &Cell::Blocked {
            return false;
        }
        // U
        if i > 0 {
            let cell = &cells[i - 1][j];
            let ok = match cell {
                Cell::None | Cell::Blocked => true,
                Cell::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        // D
        if i < self.v_size - 1 {
            let cell = &cells[i + 1][j];
            let ok = match cell {
                Cell::None | Cell::Blocked => true,
                Cell::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        // L
        if j > 0 {
            let cell = &cells[i][j - 1];
            let ok = match cell {
                Cell::None | Cell::Blocked => true,
                Cell::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        // R
        if j < self.h_size - 1 {
            let cell = &cells[i][j + 1];
            let ok = match cell {
                Cell::None | Cell::Blocked => true,
                Cell::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        true
    }

    fn calc_score(&mut self, actions: &Vec<char>) -> (f64, Vec<char>, Vec<Human>, Vec<Vec<Cell>>) {
        let bk_humans = self.humans.clone();
        let bk_cells = self.cells.clone();
        let mut fixed_actions = Vec::new();
        for i in 0..actions.len() {
            let act = actions[i];
            let act = if self.can_action(&self.humans[i], act, &bk_cells)
                && self.can_action(&self.humans[i], act, &self.cells)
            {
                act
            } else {
                '.'
            };
            fixed_actions.push(act);
            self.move_human_by_action(i, act);
        }

        let mut visited = HashSet::new();
        let mut score = 0_f64;
        for h in &self.humans {
            if visited.contains(&(h.i, h.j)) {
                continue;
            }
            let mut cell_count = 0_usize;
            let mut human_count = 0_usize;
            let mut pet_count = 0_usize;
            let mut que = VecDeque::new();
            que.push_back((h.i, h.j));
            visited.insert((h.i, h.j));
            while let Some((i, j)) = que.pop_front() {
                // 人間と動物の数を加算
                cell_count += 1;
                match self.cells[i][j] {
                    Cell::HumanOrPet(hc, pc) => {
                        human_count += hc;
                        pet_count += pc;
                    }
                    _ => {}
                }

                // 上下左右に移動
                let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (di, dj) in dirs {
                    let new_i = i as isize + di;
                    let new_j = j as isize + dj;
                    if new_i < 0
                        || new_i >= self.v_size as isize
                        || new_j < 0
                        || new_j >= self.h_size as isize
                    {
                        continue;
                    }
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    if visited.contains(&(new_i, new_j)) {
                        continue;
                    }
                    visited.insert((new_i, new_j));
                    match self.cells[new_i][new_j] {
                        Cell::None => {
                            que.push_back((new_i, new_j));
                        }
                        Cell::HumanOrPet(_, _) => {
                            que.push_back((new_i, new_j));
                        }
                        _ => {}
                    }
                }
            }
            score += (cell_count as f64 / (self.h_size * self.v_size) as f64)
                * 2_f64.powi(-(pet_count as i32))
                * human_count as f64;
        }
        score = 10.pow(8) as f64 * (1.0 as f64 / self.m as f64) * score;
        (score, fixed_actions, bk_humans, bk_cells)
    }
}

fn main() {
    let n: usize = parse_line().unwrap();
    let mut pets = Vec::new();
    for _ in 0..n {
        let (x, y, t): (usize, usize, usize) = parse_line().unwrap();
        pets.push((x - 1, y - 1, t));
    }
    let m: usize = parse_line().unwrap();
    let mut humans = Vec::new();
    for _ in 0..m {
        let (x, y): (usize, usize) = parse_line().unwrap();
        humans.push((x - 1, y - 1));
    }

    let mut solver = Solver::new(pets, humans);
    solver.solve();
}
