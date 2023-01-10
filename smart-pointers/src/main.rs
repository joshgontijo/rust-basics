fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::borrow::Cow;


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
        let _b = ref_cell.try_borrow_mut();
        assert!(!ref_cell.try_borrow_mut().is_ok()); //NOK - there is already a mut borrow (b)
    }

    #[test]
    fn test_rc() {
        struct Owner {
            name: String,
            // ...other fields
        }

        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
            // ...other fields
        }

        fn main() {
            // Create a reference-counted `Owner`.
            let gadget_owner: Rc<Owner> = Rc::new(
                Owner {
                    name: "Gadget Man".to_string(),
                }
            );

            // Create `Gadget`s belonging to `gadget_owner`. Cloning the `Rc<Owner>`
            // gives us a new pointer to the same `Owner` allocation, incrementing
            // the reference count in the process.
            let gadget1 = Gadget {
                id: 1,
                owner: Rc::clone(&gadget_owner),
            };
            let gadget2 = Gadget {
                id: 2,
                owner: Rc::clone(&gadget_owner),
            };

            // Dispose of our local variable `gadget_owner`.
            drop(gadget_owner);

            // Despite dropping `gadget_owner`, we're still able to print out the name
            // of the `Owner` of the `Gadget`s. This is because we've only dropped a
            // single `Rc<Owner>`, not the `Owner` it points to. As long as there are
            // other `Rc<Owner>` pointing at the same `Owner` allocation, it will remain
            // live. The field projection `gadget1.owner.name` works because
            // `Rc<Owner>` automatically dereferences to `Owner`.
            println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
            println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

            // At the end of the function, `gadget1` and `gadget2` are destroyed, and
            // with them the last counted references to our `Owner`. Gadget Man now
            // gets destroyed as well.
        }
    }

    //Cow not added:
    //Cow can have either a reference or a owned value
    //if a mutation is required on a reference then it will copy the value into a owned and return the reference

}
