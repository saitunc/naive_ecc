use crate::elliptic_curve::field_traits::FieldElementTraits;

#[derive(Debug,Clone,PartialEq)]
pub struct PointAffine<T:FieldElementTraits>{
    x:T,
    y:T,
    a:T, // a & b are the components of y^2 = x^3 + ax + b. 
    b:T, 
}


#[derive(Debug,Clone,PartialEq)]
pub struct PointProjective<T:FieldElementTraits>{
    x:T,
    y:T,
    z:T,
    a:T, // a & b are the components of y^2 = x^3 + ax + b. 
    b:T, 
    
}
