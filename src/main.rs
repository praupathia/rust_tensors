
#[macro_use] extern crate frunk_core; // allows us to use the handy hlist! macro

use std::fmt;
use std::fmt::Debug;
//use std::ops::Index;

//----------------------------------------------------------------------


#[derive (PartialOrd, Clone, Copy)]
struct Dim<F> {
    size:     usize,
    cap:      usize,
    value:    F        
}

// Is there some annotation to Dim that will permit us to skip
// implementing these?
impl <F> Debug for Dim<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dim {{ {}, {} }}", self.size, self.cap)
    }
}

impl <F> PartialEq for Dim<F> {
    fn eq(&self, other: &Dim<F>) -> bool {
        (self.size == other.size)
            &&
            (self.cap == other.cap)
    }
}

//----------------------------------------------------------------------
//----------------------------------------------------------------------

// DimensionList

fn main() {

    let f = |x: usize| x;
    
    let dim3 = Dim {size:3, cap:3, value:f};
    
//    let h: Hlist!(&Dim,&Dim) = hlist![&dim3, &dim3];
    let h = hlist![&dim3, &dim3];
    
    // We use the Hlist! type macro to make it easier to write
    // a type signature for HLists, which is a series of nested HCons
    // h has an expanded static type of: HCons<&str, HCons<&str, HCons<i32, HCons<bool, HNil>>>>

//    let nrow = dim3[0];
    
    
    let hlist_pat!(dim1, dim2) = h;
    assert_eq!(dim1, &dim3);
    assert_eq!(dim2, &dim3);

    println!("The HList is : {:#?}", h);
    
    println!("Hello, world!!");
}
