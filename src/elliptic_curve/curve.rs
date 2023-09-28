use crate::elliptic_curve::field::FieldElement;
use crate::elliptic_curve::field_traits::FieldElementTraits;
use num_bigint::BigInt;
use num_traits::{Zero,One};


use std::ops::{Add,Sub};



#[derive(Debug,Clone)]
pub struct CurveElement{
    element: FieldElement,
}


impl CurveElement{

    pub fn check_in_curve(&self){

    }

    pub fn double(&self) //-> Self
    {
    
    }

    
}