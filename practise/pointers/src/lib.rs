use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: UnsafeCell::new(val),
        }
    }
    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    pub fn set(&self, val: T) {
        unsafe {
            *self.value.get() = val;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bad_code() {
        let x = Cell::new(vec![43]);
        let first = &x.get()[0];
        for y in 0..100000 {
            x.set(vec![y]);
        }
        eprintln!("{first}");
        eprintln!("{}", &x.get()[0]);
    }
    #[test]
    fn code() {
        let mut value = 43i32;
        let x = &raw mut value;
        unsafe {
            *x = 3;
            eprintln!("{}", *x);
        }
    }

    #[test]
    fn test_new_and_get() {
        let cell = Cell::new(10);
        assert_eq!(*cell.get(), 10);
    }

    #[test]
    fn test_set() {
        let cell = Cell::new(5);
        assert_eq!(*cell.get(), 5);
        cell.set(15);
        assert_eq!(*cell.get(), 15);
    }
    //hl
    #[test]
    fn test_with_struct() {
        #[derive(Debug, PartialEq, Clone, Copy)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 1, y: 2 };
        let cell = Cell::new(p);
        assert_eq!(*cell.get(), p);
        let p2 = Point { x: 10, y: 20 };
        cell.set(p2);
        assert_eq!(*cell.get(), p2);
    }
}
