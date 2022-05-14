use crate::sorting::{SortModel, Sorter};

pub struct InsertionSort {}

impl InsertionSort {
    pub fn new() -> Self {
        Self {}
    }

    fn insertion_sort(&self, sorter: &mut SortModel) {
        for mut i in 1..sorter.len() {
            while i > 0 && sorter.compare(i - 1, i, |a, b| a > b) {
                sorter.swap(i, i - 1);
                i -= 1;
            }
        }
    }
}

impl Sorter for InsertionSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        self.insertion_sort(&mut sorter);
        sorter.complete();

        sorter
    }
}