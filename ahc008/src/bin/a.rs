#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use whiteread::parse_line;

pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

struct Human {
    i: usize,
    j: usize,
}

struct Pet {
    i: usize,
    j: usize,
    t: PetType,
}

enum PetType {
    Ushi,
    Buta,
    Usagi,
    Inu,
    Neko,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum CellType {
    None,
    Stop,
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
    h_size: usize,
    v_size: usize,
    cells: Vec<Vec<CellType>>,
}

impl Solver {
    fn new(pets: Vec<(usize, usize, usize)>, humans: Vec<(usize, usize)>) -> Self {
        let mut cells = vec![vec![CellType::None; 30]; 30];
        for pet in &pets {
            cells[pet.0][pet.1] = CellType::HumanOrPet(0, 1);
        }
        for human in &humans {
            cells[human.0][human.1] = CellType::HumanOrPet(1, 0);
        }
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
            h_size: 30,
            v_size: 30,
            cells,
        }
    }

    fn solve(&mut self) {
        for turn in 1..=self.all_turn {
            self.cur_turn = turn;

            let human_actions = self.move_humans();
            println!("{}", human_actions.iter().join(""));
            let pet_actions: Vec<Vec<char>> = read_line()
                .split(" ")
                .map(|s| s.chars().into_iter().collect::<Vec<char>>())
                .collect();
            self.move_pets_by_actions(&pet_actions);
        }
    }

    fn move_humans(&mut self) -> Vec<char> {
        if !self.done_move_to_initial_positon {
            return self.move_to_initial_position();
        }

        let mut actions = Vec::new();
        for i in 0..self.m {
            let human = &self.humans[i];
            let act = match self.cells[human.i][human.j - 1] {
                CellType::Stop if human.i < self.v_size - 1 => 'D',
                CellType::None if self.can_stop_cell(human.i, human.j - 1) => 'l',
                _ => '.',
            };
            actions.push(act);
        }
        self.move_humans_by_actions(&actions);
        actions
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
        self.move_humans_by_actions(&actions);
        actions
    }

    fn move_humans_by_actions(&mut self, action: &Vec<char>) {
        for i in 0..self.m {
            let human = &mut self.humans[i];
            let mut new_i = human.i;
            let mut new_j = human.j;
            match action[i] {
                'U' => new_i -= 1,
                'D' => new_i += 1,
                'L' => new_j -= 1,
                'R' => new_j += 1,
                'u' => self.cells[human.i - 1][human.j] = CellType::Stop,
                'd' => self.cells[human.i + 1][human.j] = CellType::Stop,
                'l' => self.cells[human.i][human.j - 1] = CellType::Stop,
                'r' => self.cells[human.i][human.j + 1] = CellType::Stop,
                _ => {}
            }

            // セルの状態更新
            if let CellType::HumanOrPet(c, c2) = self.cells[human.i][human.j] {
                // 移動元
                if c == 1 && c2 == 0 {
                    self.cells[human.i][human.j] = CellType::None;
                } else {
                    self.cells[human.i][human.j] = CellType::HumanOrPet(c - 1, c2);
                }
                // 移動先
                if let CellType::HumanOrPet(c, c2) = self.cells[new_i][new_j] {
                    self.cells[new_i][new_j] = CellType::HumanOrPet(c + 1, c2);
                } else {
                    self.cells[new_i][new_j] = CellType::HumanOrPet(1, 0);
                }
            } else {
                unreachable!();
            }

            human.i = new_i;
            human.j = new_j;
        }
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
            if let CellType::HumanOrPet(c, c2) = self.cells[pet.i][pet.j] {
                // 移動元
                if c == 0 && c2 == 1 {
                    self.cells[pet.i][pet.j] = CellType::None;
                } else {
                    self.cells[pet.i][pet.j] = CellType::HumanOrPet(c, c2 - 1);
                }
                // 移動先
                if let CellType::HumanOrPet(c, c2) = self.cells[new_i][new_j] {
                    self.cells[new_i][new_j] = CellType::HumanOrPet(c, c2 + 1);
                } else {
                    self.cells[new_i][new_j] = CellType::HumanOrPet(0, 1);
                }
            } else {
                unreachable!();
            }

            pet.i = new_i;
            pet.j = new_j;
        }
    }

    fn can_stop_cell(&self, i: usize, j: usize) -> bool {
        let cell = &self.cells[i][j];
        if cell != &CellType::None && cell != &CellType::Stop {
            return false;
        }
        // U
        if i > 0 {
            let cell = &self.cells[i - 1][j];
            let ok = match cell {
                CellType::None | CellType::Stop => true,
                CellType::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        // D
        if i < self.v_size - 1 {
            let cell = &self.cells[i + 1][j];
            let ok = match cell {
                CellType::None | CellType::Stop => true,
                CellType::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        // L
        if j > 0 {
            let cell = &self.cells[i][j - 1];
            let ok = match cell {
                CellType::None | CellType::Stop => true,
                CellType::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        // R
        if j < self.h_size - 1 {
            let cell = &self.cells[i][j + 1];
            let ok = match cell {
                CellType::None | CellType::Stop => true,
                CellType::HumanOrPet(_, c) => c == &0,
            };
            if !ok {
                return false;
            }
        }
        true
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
