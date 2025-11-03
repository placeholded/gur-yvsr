#[allow(dead_code)]
pub mod prelude {
    use std::collections::HashMap;
    use crate::errors::err::*;
    pub struct Acc {
        val: Option<isize>
    }
    impl Acc {
        pub(crate) fn new() -> Acc {
            Acc {
                val: None
            }
        }
        pub fn get_value(&self) -> Result<isize, ()> {
            if let Some(n) = self.val {
                return Ok(n)
            }
            Err(())
        }
        pub fn set_value(&mut self, value: isize) {
            self.val = Some(value);
        }
        pub fn is_empty(&self) -> bool {
            self.val.is_none()
        }
        pub fn is_not_empty(&self) -> bool {
            self.val.is_some()
        }
        pub fn append(&mut self, digit: isize) -> Result<(),()> {
            if let Some(n) = self.val {
                let result1 = n.overflowing_mul(10);
                let result2 = result1.0.overflowing_add(digit);
                if result1.1 || result2.1 {
                    Error::OverflowError.throw(&format!("digit {digit} caused overflow"), false);
                    return Err(())
                }
                self.val = Some(n * 10 + digit);
            } else {
                self.val = Some(digit);
            }
            Ok(())
        }
        pub(crate) fn clear(&mut self) -> Option<isize> {
            let a = self.val;
            self.val = None;
            a
        }
        pub(crate) fn get_details(&self) -> String {
            if let Some(n) = self.val {
                return format!("accumulator value: {n}")
            }
            "accumulator was empty".to_string()
        }
    }
    pub struct Tape {
        cells: HashMap<isize, isize>
    }
    impl Tape {
        pub(crate) fn new() -> Tape {
            Tape {
                cells: HashMap::new()
            }
        }

        /// Returns a [`Some`] value containing the value at the cell of the provided index, or [`None`] if the cell at the provided index is empty.
        pub(crate) fn get(&self, index: isize) -> Option<isize> {
            if let Some(n) = self.cells.get(&index) {
                return Some(*n);
            }
            None
        }

        /// Replaces the value of the cell at the provided index with another value.
        pub(crate) fn set(&mut self, index: isize, value: isize) {
            self.cells.insert(index, value);
        }

        /// Clears the cell at the provided index.
        /// If the cell was originally empty, [`None`] is returned. Otherwise, a [`Some`] value containing the cleared value is returned.
        pub(crate) fn clear(&mut self, index: isize) -> Option<isize> {
            if !self.cells.contains_key(&index) {
                return None
            }
            self.cells.remove(&index)
        }

        pub(crate) fn cell_is_full(&self, index: isize) -> bool {
            if let Some(_) = self.cells.get(&index) {
                return true;
            }
            false
        }

        pub(crate) fn cell_is_empty(&self, index: isize) -> bool {
            if let Some(_) = self.cells.get(&index) {
                return false;
            }
            true
        }

        /// Returns an [`Ok`] if the cell at `index - 1` exists, and an [`Err`] if not.
        /// If said cell is empty, a [`None`] is contained within the returned [`Ok`].
        /// Otherwise, a [`Some`] value containing the cell's value is contained within the returned [`Ok`].
        pub(crate) fn left_of(&self, index: isize) -> Result<Option<isize>, ()> {
            if let Some(_) = index.checked_sub(1) {
                if let Some(n) = self.get(index - 1) {
                    Ok(Some(n))
                } else {
                    Ok(None)
                }
            } else {
                Err(())
            }
        }
    }
}