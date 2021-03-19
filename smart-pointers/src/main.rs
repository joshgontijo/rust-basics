fn main() {}

///RefCell -> Rwlock
///Rc -> Arc
///
///
#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_cell() {
        struct SomeStruct {
            regular_field: u8,
            special_field: Cell<u8>,
        }

        let my_struct = SomeStruct {
            regular_field: 0,
            special_field: Cell::new(1),
        };

        //Cell is a mutable memory location.

        let new_value = 100;
        // ERROR: `my_struct` is immutable
        // my_struct.regular_field = new_value;

        // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
        // which can always be mutated
        my_struct.special_field.set(new_value);
        assert_eq!(my_struct.special_field.get(), new_value);
    }

    #[test]
    fn test_refcell() {
        struct SomeStruct {
            v: usize
        }

        let my_struct = SomeStruct { v: 0 };

        let ref_cell = std::cell::RefCell::new(my_struct);

        //RefCell basically 'count' the number of given immutable and mutable references in order to
        //determine whether the value can be borrowed mutably or immutably at runtime.
        //It's a thread unsafe version of RwLock, that instead blocking until you can have the access you want (write/read)

        assert!(ref_cell.try_borrow().is_ok()); //OK - immut ref borrow
        let a = ref_cell.borrow(); //borrow immut
        assert!(ref_cell.try_borrow().is_ok()); //OK - can have multiple immut brrows
        assert!(!ref_cell.try_borrow_mut().is_ok()); //NOK - there are immut borrow (a)
        drop(a);

        assert!(ref_cell.try_borrow_mut().is_ok()); //OK - no immut or mut borrow are given at this point
        let b = ref_cell.try_borrow_mut();
        assert!(!ref_cell.try_borrow_mut().is_ok()); //NOK - there is already a mut borrow (b)
    }

    #[test]
    fn test_rc() {
        struct SomeStruct {
            v: usize
        }

        let my_struct = SomeStruct { v: 0 };

        let rc = Rc::new(my_struct);


    }
}
