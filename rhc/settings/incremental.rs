use crate::core::charset::Table;

use super::validator::IncrementalValues;

pub struct IncrementalSettings {
    max_length: usize,
    min_length: usize,
    range: &'static [u8],
    table: Table,
}

impl IncrementalSettings {
    pub fn from(values: IncrementalValues, count: usize) -> Self {
        Self {
            max_length: values.max_length(),
            min_length: values.min_length(),
            range: Self::get_range(values.charset(), count, values.thread_count()),
            table: values.charset(),
        }
    }

    fn get_range(table: Table, count: usize, num_cores: usize) -> &'static [u8] {
        let chunk_size = table.len() / num_cores;

        let start = chunk_size * count;
        let end = if count == num_cores {
            table.len()
        } else {
            chunk_size * (count + 1)
        };
        &table[start..end]
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn min_length(&self) -> usize {
        self.min_length
    }

    pub fn range(&self) -> &'static [u8] {
        self.range
    }

    pub fn table(&self) -> &'static [u8] {
        self.table
    }
}