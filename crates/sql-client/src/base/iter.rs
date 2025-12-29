use crate::base::{bindings::wasmledger::sql::query_types::QueryResults, row::RowPointer};

pub struct RowPointerIter<'a> {
    query_results: &'a QueryResults,
    total_rows: u64,
    current: u64,
}

impl<'a> Iterator for RowPointerIter<'a> {
    type Item = RowPointer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.total_rows {
            let pos = RowPointer {
                query_results: self.query_results,
                row_index: self.current,
            };
            self.current += 1;
            Some(pos)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.total_rows - self.current) as usize;
        (remaining, Some(remaining))
    }
}

impl<'a> IntoIterator for &'a QueryResults {
    type Item = RowPointer<'a>;
    type IntoIter = RowPointerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RowPointerIter {
            query_results: &self,
            total_rows: self.row_count(),
            current: 0,
        }
    }
}

impl QueryResults {
    pub fn get_row<'a>(&'a self, index: u64) -> RowPointer<'a> {
        RowPointer {
            query_results: &self,
            row_index: index,
        }
    }
}
