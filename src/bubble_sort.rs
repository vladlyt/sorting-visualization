use crate::sorting::{SortModel, SortingState, Sorter};

pub struct BubbleSort {
    sorter: SortModel,
}

impl BubbleSort {
    pub fn new(v: Vec<u32>) -> Self {
        Self {
            sorter: SortModel::new(v),
        }
    }
}

impl Sorter for BubbleSort {
    fn sort(&mut self) -> &mut Vec<SortingState> {
        for i in 0..self.sorter.current_state.len() {
            for j in 0..(self.sorter.current_state.len() - i - 1) {
                if self.sorter.value_is_greater(j, j + 1) {
                    self.sorter.swap_values(j, j + 1);
                }
            }
        }
        self.sorter.complete();
        &mut self.sorter.states
    }
}
