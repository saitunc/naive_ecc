use crate::elliptic_curve::field_traits::FieldElementTraits;
use crate::elliptic_curve::field::FieldElement;

#[derive(Debug,Clone,PartialEq)]
pub struct PointAffine<FieldElement>{
    x:FieldElement,
    y:FieldElement,
    a:FieldElement, // a & b are the components of y^2 = x^3 + ax + b. 
    b:FieldElement, 
}


#[derive(Debug,Clone,PartialEq)]
pub struct PointProjective<FieldElement>{
    x:FieldElement,
    y:FieldElement,
    z:FieldElement,
    a:FieldElement, // a & b are the components of y^2 = x^3 + ax + b. 
    b:FieldElement, 
}



impl PointAffine<FieldElement>{

    pub fn new(&self,x:FieldElement,y:FieldElement,a:FieldElement,b:FieldElement) -> Self{
        PointAffine{x,y,a,b}
    }

}





impl PointProjective<FieldElement>{

    pub fn new(&self,x:FieldElement,y:FieldElement,z:FieldElement,a:FieldElement,b:FieldElement) -> Self{
        PointProjective{x,y,z,a,b}
    }

}
