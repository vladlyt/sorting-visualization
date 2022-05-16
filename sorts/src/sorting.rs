use std::cmp::Ordering;
use std::ops::Index;

#[derive(Clone, Copy, Debug)]
pub enum SortingStateEnum {
    FREE,
    COMPARE,
    SWAP,
    CHECKING,
    COMPLETED,
}

pub type SortingState = Vec<SortingValue>;

#[derive(Debug, Clone, Copy)]
pub struct SortingValue {
    pub value: u32,
    pub state: SortingStateEnum,
}

impl Eq for SortingValue {}

impl PartialEq<Self> for SortingValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd<Self> for SortingValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for SortingValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl SortingValue {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            state: SortingStateEnum::FREE,
        }
    }
}

pub trait Sorter {
    fn sort(&mut self, values: Vec<u32>) -> SortModel;
}

pub struct SortModel {
    current_state: SortingState,
    states: Vec<SortingState>,
    boundaries: Option<(usize, usize)>,
}

impl SortModel {
    pub fn new(values: Vec<u32>) -> Self {
        let current_state = values
            .clone()
            .into_iter()
            .map(|value| SortingValue::new(value))
            .collect();

        let current_state2 = values
            .into_iter()
            .map(|value| SortingValue::new(value))
            .collect();

        Self {
            current_state,
            states: vec![current_state2],
            boundaries: None,
        }
    }

    pub fn compare<F>(&mut self, left: usize, right: usize, compare_func: F) -> bool
    where
        F: Fn(SortingValue, SortingValue) -> bool,
    {
        let state_index = self.add_new_state();
        self.states[state_index][left].state = SortingStateEnum::COMPARE;
        self.states[state_index][right].state = SortingStateEnum::COMPARE;
        compare_func(self.current_state[left], self.current_state[right])
    }

    #[allow(dead_code)]
    pub fn get_initial_state(&self) -> &SortingState {
        &self.states[0]
    }

    #[allow(dead_code)]
    pub fn get_final_state(&self) -> &SortingState {
        &self.states[self.states.len() - 1]
    }

    #[allow(dead_code)]
    pub fn get_current_state(&self) -> &SortingState {
        &self.current_state
    }

    pub fn get_states(&self) -> &Vec<SortingState> {
        &self.states
    }

    pub fn len(&self) -> usize {
        self.current_state.len()
    }

    pub fn swap(&mut self, left: usize, right: usize) {
        self.current_state.swap(left, right);
        let state_index = self.add_new_state();
        self.states[state_index][left].state = SortingStateEnum::SWAP;
        self.states[state_index][right].state = SortingStateEnum::SWAP;
    }

    pub fn set_value(&mut self, index: usize, value: SortingValue) {
        self.current_state[index] = value;
        let state_index = self.add_new_state();
        self.states[state_index][index].state = SortingStateEnum::SWAP;
    }

    fn add_new_state(&mut self) -> usize {
        self.states.push(
            self.current_state
                .iter()
                .map(|v| SortingValue::new(v.value))
                .collect(),
        );
        let new_state_index = self.states.len() - 1;

        if let Some((left, right)) = self.boundaries {
            for i in left..right + 1 {
                self.states[new_state_index][i].state = SortingStateEnum::CHECKING;
            }
        }

        self.states.len() - 1
    }

    pub fn set_boundaries(&mut self, left: usize, right: usize) {
        self.boundaries = Some((left, right));
    }

    pub fn unset_boundaries(&mut self) {
        self.boundaries = None;
    }

    pub fn complete(&mut self) {
        for i in 0..self.current_state.len() {
            let state_index = self.add_new_state();
            for j in 0..i + 1 {
                self.states[state_index][j].state = SortingStateEnum::COMPLETED;
            }
        }
    }
}

impl Index<usize> for SortModel {
    type Output = SortingValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.current_state[index]
    }
}
