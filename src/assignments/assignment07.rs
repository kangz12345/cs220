//! Assignment 7: Mastering advanced types (2/2).
//!
//! The primary goal of this assignment is to understand generics, traits, and lifetimes.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-07.sh` works fine.
//! See `assignment07_grade.rs` and `/scripts/grade-07.sh` for the test script.

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let init = &self.query[0];
        for (index, item) in self.base.iter().enumerate().skip(self.curr) {
            if item == init {
                let mut new_query = self.query[1..].iter();
                let mut new_base = self.base[index + 1..].iter();
                let mut matched = true;
                for q in new_query {
                    if let Some(b) = new_base.next() {
                        if *q != *b {
                            matched = false;
                            break;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    self.curr = index + 1;
                    return Some(index);
                }
            }
        }
        self.curr = self.base.len();
        None
    }
}

/// Returns an iterator over substring query indexes in the base.
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}
