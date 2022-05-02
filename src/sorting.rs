use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum SortingStateEnum {
    FREE,
    COMPARE,
    SWAP,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy)]
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

pub type SortingState = Vec<SortingValue>;

pub trait Sorter {
    fn sort(&mut self) -> &mut Vec<SortingState>;
}

pub struct SortModel {
    pub current_state: SortingState,
    pub states: Vec<SortingState>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl SortModel {
    pub fn new(v: Vec<u32>) -> Self {
        let current_state = v
            .iter()
            .map(|value| SortingValue::new(*value))
            .collect();

        Self {
            current_state,
            states: Vec::new(),
            left: None,
            right: None,
        }
    }

    pub fn value_is_greater(&mut self, left: usize, right: usize) -> bool {
        let state_index = self.add_new_state();
        self.states[state_index][left].state = SortingStateEnum::COMPARE;
        self.states[state_index][right].state = SortingStateEnum::COMPARE;
        self.current_state[left] > self.current_state[right]
    }

    pub fn value_is_greater_or_equal(&mut self, left: usize, right: usize) -> bool {
        let state_index = self.add_new_state();
        self.states[state_index][left].state = SortingStateEnum::COMPARE;
        self.states[state_index][right].state = SortingStateEnum::COMPARE;
        self.current_state[left] >= self.current_state[right]
    }

    pub fn swap_values(&mut self, left: usize, right: usize) {
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
                .collect()
        );
        let new_state_index = self.states.len() - 1;

        if let Some(left) = self.left {
            self.states[new_state_index][left].state = SortingStateEnum::LEFT;
        }
        if let Some(right) = self.right {
            self.states[new_state_index][right].state = SortingStateEnum::RIGHT;
        }

        self.states.len() - 1
    }

    pub fn compare_index(&mut self, index: usize) {
        let state_index = self.add_new_state();
        self.states[state_index][index].state = SortingStateEnum::COMPARE;
    }

    pub fn set_left_right(&mut self, left: usize, right: usize) {
        self.left = Some(left);
        self.right = Some(right);
    }

    pub fn unset_left_right(&mut self) {
        self.left = None;
        self.right = None;
    }

    pub fn complete(&mut self) {
        for i in 0..self.current_state.len() {
            let state_index = self.add_new_state();
            for j in 0..i + 1 {
                self.states[state_index][j].state = SortingStateEnum::COMPARE;
            }
        }
    }
}

