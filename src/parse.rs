use crate::app::App;
use core::panic;
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::Display,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SortField {
    idx_start: u16,
    idx_end: u16,
    ignore_leading_blank: bool,
    ignore_non_printable: bool,
    cmp_order_reverse: bool,
    cmp_only_blank_and_alphanumeric: bool,
    cmp_only_initial_numeric: bool,
    cmp_lowercase_as_uppercase: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LineMeta {
    InitialNumber { initial_number: f64 },
    Fields { start: SortField, end: SortField },
}

#[derive(Debug, Clone)]
pub struct SortLine {
    content: String,
    meta: Option<LineMeta>,
}

impl Display for SortLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Eq for SortLine {}

#[allow(unused_variables)]
impl PartialEq for SortLine {
    fn eq(&self, other: &Self) -> bool {
        match (&self.meta, &other.meta) {
            (None, None) => self.content == other.content,
            (
                Some(LineMeta::InitialNumber { initial_number: a }),
                Some(LineMeta::InitialNumber { initial_number: b }),
            ) => {
                todo!("compare initial number")
            }
            (
                Some(LineMeta::Fields { start: a, end: b }),
                Some(LineMeta::Fields { start: c, end: d }),
            ) => {
                todo!("compare fields")
            }
            _ => panic!("invalid meta"),
        }
    }
}

impl PartialOrd for SortLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused_variables)]
impl Ord for SortLine {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.meta, &other.meta) {
            (None, None) => self.content.cmp(&other.content),
            (
                Some(LineMeta::InitialNumber { initial_number: a }),
                Some(LineMeta::InitialNumber { initial_number: b }),
            ) => {
                todo!("compare initial number")
            }
            (
                Some(LineMeta::Fields { start: a, end: b }),
                Some(LineMeta::Fields { start: c, end: d }),
            ) => {
                todo!("compare fields")
            }
            _ => panic!("invalid meta"),
        }
    }
}

pub fn tokenize_into_lines(app: &App, input: &str) {
    let sep = if app.settings.nul_term { "\0" } else { "\n" };
    app.line_accumulator
        .borrow_mut()
        .extend(input.split(sep).map(|token| {
            if app.settings.sort_by_keydef {
                todo!("sort by keydef")
            } else if app.settings.sort_numeric {
                todo!("sort numeric")
            } else {
                SortLine {
                    content: token.to_string(),
                    meta: None,
                }
            }
        }))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_tokenize_into_lines() {

//     }
// }
