use std::{slice, vec::IntoIter};

use crate::client::bindings::wasmledger::sql::{
    codecs::{ColumnIndex, ValuePosition},
    query::QueryResults,
};

pub mod bindings {
    #[cfg(feature = "client-postgres")]
    wit_bindgen::generate!({
        path: ["../../wit/sql", "../../wit/sql/postgres", "./wit/client-postgres"],
        world: "wasmledger:sql-postgres-client/client-postgres",
        generate_all
    });
}

pub struct LazyValuePosition {
    row_index: u64,
}

pub struct QueryResultsIter {
    row_count: u64,
    current: u64,
}

impl LazyValuePosition {
    pub fn column_index(&self, i: u64) -> ValuePosition {
        (self.row_index, ColumnIndex::Index(i))
    }

    pub fn column_name(&self, name: String) -> ValuePosition {
        (self.row_index, ColumnIndex::ColumnName(name))
    }
}

impl Iterator for QueryResultsIter {
    type Item = LazyValuePosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.row_count {
            let pos = LazyValuePosition {
                row_index: self.current,
            };
            self.current += 1;
            Some(pos)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.row_count - self.current) as usize;
        (remaining, Some(remaining))
    }
}

impl<'a> IntoIterator for &'a QueryResults {
    type Item = LazyValuePosition;
    type IntoIter = QueryResultsIter;

    fn into_iter(self) -> Self::IntoIter {
        QueryResultsIter {
            row_count: self.row_count(),
            current: 0,
        }
    }
}
