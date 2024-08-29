use std::ops::{Bound, Range, RangeBounds};

use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Bounds {
    /// The lower bound of the range (inclusive).
    pub start: u64,
    /// The upper bound of the range (exclusive).
    pub end: u64,
}

impl Bounds {
    pub fn contains(&self, value: u64) -> bool {
        self.start <= value && value < self.end
    }
}

impl RangeBounds<u64> for Bounds {
    fn start_bound(&self) -> Bound<&u64> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&u64> {
        Bound::Excluded(&self.end)
    }
}

impl From<Bounds> for Range<u64> {
    fn from(value: Bounds) -> Self {
        Self { start: value.start, end: value.end }
    }
}

impl From<Range<u64>> for Bounds {
    fn from(value: Range<u64>) -> Self {
        Self { start: value.start, end: value.end }
    }
}
