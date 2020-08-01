#![no_std]

extern crate alloc;
use alloc::fmt;
use alloc::vec::Vec;

pub struct Cursor<'a, T: 'a> {
    index: usize,
    vec: &'a Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for Cursor<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Cursor")
            .field(&self.vec)
            .field(&self.index())
            .finish()
    }
}

pub struct CursorMut<'a, T: 'a> {
    index: usize,
    vec: &'a mut Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for CursorMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CursorMut")
            .field(&self.vec)
            .field(&self.index())
            .finish()
    }
}

impl<'a, T> Cursor<'a, T> {
    fn check_current(&self) -> Option<()> {
        if self.index < self.vec.len() {
            Some(())
        } else {
            None
        }
    }
    pub fn index(&self) -> Option<usize> {
        let _ = self.check_current()?;
        Some(self.index)
    }
    pub fn move_next(&mut self) {
        match self.check_current() {
            None => {
                self.index = 0;
            }
            Some(_) => {
                self.index += 1;
            }
        }
    }
    pub fn move_prev(&mut self) {
        match self.check_current() {
            None if self.index == 0 => {
                self.index = 0;
            }
            _ => self.index = self.index.checked_sub(1).unwrap_or_else(|| self.vec.len()),
        }
    }
    pub fn current(&self) -> Option<&'a T> {
        let _ = self.check_current()?;
        unsafe { Some(self.vec.get_unchecked(self.index)) }
    }
    pub fn peek_next(&self) -> Option<&'a T> {
        let next_index = match self.check_current() {
            None => 0,
            Some(_) => self.index + 1,
        };
        self.vec.get(next_index)
    }
    pub fn peek_prev(&mut self) -> Option<&'a T> {
        let prev_index = match self.check_current() {
            None if self.index == 0 => 0,
            _ => self.index - 1,
        };
        self.vec.get(prev_index)
    }
}

impl<'a, T> CursorMut<'a, T> {
    fn check_current(&self) -> Option<()> {
        if self.index < self.vec.len() {
            Some(())
        } else {
            None
        }
    }
    pub fn index(&self) -> Option<usize> {
        let _ = self.check_current()?;
        Some(self.index)
    }
    pub fn move_next(&mut self) {
        match self.check_current() {
            None => {
                self.index = 0;
            }
            Some(_) => {
                self.index += 1;
            }
        }
    }
    pub fn move_prev(&mut self) {
        match self.check_current() {
            None if self.index == 0 => {
                self.index = 0;
            }
            _ => self.index = self.index.checked_sub(1).unwrap_or_else(|| self.vec.len()),
        }
    }
    pub fn current(&mut self) -> Option<&mut T> {
        let _ = self.check_current()?;
        unsafe { Some(self.vec.get_unchecked_mut(self.index)) }
    }
    pub fn peek_next(&mut self) -> Option<&mut T> {
        let next_index = match self.check_current() {
            None => 0,
            Some(_) => self.index + 1,
        };
        self.vec.get_mut(next_index)
    }
    pub fn peek_prev(&mut self) -> Option<&mut T> {
        let prev_index = match self.check_current() {
            None if self.index == 0 => 0,
            _ => self.index - 1,
        };
        self.vec.get_mut(prev_index)
    }
    pub fn as_cursor<'cm>(&'cm self) -> Cursor<'cm, T> {
        Cursor {
            vec: self.vec,
            index: self.index,
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn insert_after(&mut self, item: T) {
        let (next_index, need_reset) = match self.check_current() {
            None => (0, true),
            Some(_) => (self.index + 1, false),
        };
        self.vec.insert(next_index, item);
        if need_reset {
            self.index = self.vec.len()
        }
    }

    pub fn insert_before(&mut self, item: T) {
        let prev_index = match self.check_current() {
            None if self.index == 0 => 0,
            _ => self.index - 1,
        };
        self.vec.insert(prev_index, item);
        self.index += 1;
    }

    pub fn remove_current(&mut self) -> Option<T> {
        let _ = self.check_current()?;
        Some(self.vec.remove(self.index))
    }

    pub fn splice_after(&mut self, vec: Vec<T>) {
        if vec.is_empty() {
            return;
        };
        let (next_index, need_reset) = match self.check_current() {
            None => (0, true),
            Some(_) => (self.index + 1, false),
        };
        self.vec.splice(next_index..next_index, vec);
        if need_reset {
            self.index = self.vec.len()
        }
    }

    pub fn splice_before(&mut self, vec: Vec<T>) {
        if vec.is_empty() {
            return;
        }
        let splice_len = vec.len();
        let current_index = self.index;
        self.vec.splice(current_index..current_index, vec);
        self.index += splice_len;
    }

    pub fn split_after(&mut self) -> Vec<T> {
        let (next_index, need_reset) = match self.check_current() {
            None => (0, true),
            Some(_) => (self.index + 1, false),
        };
        let result = self.vec.split_off(next_index);
        if need_reset {
            self.index = self.vec.len()
        }
        result
    }

    pub fn split_before(&mut self) -> Vec<T> {
        use core::mem::swap;
        let mut result = self.vec.split_off(self.index);
        self.index = 0;
        swap(self.vec, &mut result);
        result
    }
}
