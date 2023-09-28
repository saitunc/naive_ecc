use crate::elliptic_curve::point::point::{PointAffine,PointProjective};
use crate::elliptic_curve::field::FieldElement;
use num_bigint::BigInt;
use num_traits::{Zero,One};

pub trait PointOperations{

    fn new(&self,x:FieldElement,y:FieldElement)->Self;
    
    fn multiply(&self,other:Self,n:BigInt)->Self;


    

}
