
#[macro_use] extern crate frunk_core; // allows us to use the handy hlist! macro

use std::ops::Index;

//----------------------------------------------------------------------

pub trait IsDimension:
Index<usize>
{
    fn len     (&self) -> usize;
    fn capacity(&self) -> usize;
}

struct Dim {
    size_:     usize,
    capacity_ : usize,
}

impl Index<usize> for Dim {
    type Output = usize;
    fn index (&self, i: usize) -> &usize {
        i
    }
}

impl IsDimension for Dim {

    fn capacity(&self) -> usize {
        self.size_
    }
    fn len     (&self) -> usize {
        self.capacity_
    }
}

//----------------------------------------------------------------------

// DimensionList

fn main() {
    
    let h: Hlist!(&str, &str, i32, bool) = hlist!["Joe", "Blow", 30, true];
    
    // We use the Hlist! type macro to make it easier to write
    // a type signature for HLists, which is a series of nested HCons
    // h has an expanded static type of: HCons<&str, HCons<&str, HCons<i32, HCons<bool, HNil>>>>
    
    
    let hlist_pat!(f_name, l_name, age, is_admin) = h;
    assert_eq!(f_name, "Joe");
    assert_eq!(l_name, "Blow");
    assert_eq!(age, 30);
    assert_eq!(is_admin, true);

    println!("The HList is : {:#?}", h);
    
    println!("Hello, world!!");
}
