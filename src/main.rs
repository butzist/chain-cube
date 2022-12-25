mod chain;

use chain::CHAIN;
use ndarray::{arr1, Array1, Array3};
use std::{array::IntoIter, fmt::Debug};

fn main() {
    let state = SearchState {
        chain: CHAIN.clone().into_iter(),
        cube: Array3::zeros([4, 4, 4]),
        index: 0,
        position: arr1(&[-1, 1, 0]), // place first element
        solution: vec![],
    };

    let state = place_element(state, Direction::Xp).unwrap();

    println!("{:?}", place_remaining(state));
}

fn place_remaining(state: SearchState) -> Option<SearchState> {
    if state.chain.clone().next().is_none() {
        return Some(state);
    }

    for dir in state.solution.last().unwrap().orthogonal() {
        match place_element(state.clone(), dir).and_then(|next_state| place_remaining(next_state)) {
            Some(solved_state) => return Some(solved_state),
            None => (),
        }
    }

    None
}

fn place_element(mut new_state: SearchState, direction: Direction) -> Option<SearchState> {
    //let mut new_state = state.clone();
    let next = new_state.chain.next().unwrap();
    new_state.solution.push(direction);

    let direction_vec = direction.to_vec();
    for _i in 0..next {
        new_state.index += 1;
        new_state.position += &direction_vec;

        let cube_index = (
            new_state.position[0] as usize,
            new_state.position[1] as usize,
            new_state.position[2] as usize,
        );

        if let Some(cube_value) = new_state.cube.get_mut(cube_index) {
            if *cube_value != 0 {
                return None;
            }

            *cube_value = new_state.index;
        } else {
            return None;
        };
    }

    Some(new_state)
}

#[derive(Debug, Clone)]
struct SearchState {
    chain: IntoIter<u8, 39>,
    cube: Array3<u8>,
    position: Array1<i8>,
    index: u8,
    solution: Vec<Direction>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Xp,
    Xn,
    Yp,
    Yn,
    Zp,
    Zn,
}

impl Direction {
    fn to_vec(&self) -> Array1<i8> {
        match self {
            Direction::Xp => arr1(&[1, 0, 0]),
            Direction::Xn => arr1(&[-1, 0, 0]),
            Direction::Yp => arr1(&[0, 1, 0]),
            Direction::Yn => arr1(&[0, -1, 0]),
            Direction::Zp => arr1(&[0, 0, 1]),
            Direction::Zn => arr1(&[0, 0, -1]),
        }
    }

    fn orthogonal(&self) -> [Self; 4] {
        match self {
            Direction::Xp | Direction::Xn => {
                [Direction::Yp, Direction::Yn, Direction::Zp, Direction::Zn]
            }
            Direction::Yp | Direction::Yn => {
                [Direction::Xp, Direction::Xn, Direction::Zp, Direction::Zn]
            }
            Direction::Zp | Direction::Zn => {
                [Direction::Xp, Direction::Xn, Direction::Yp, Direction::Yn]
            }
        }
    }
}
