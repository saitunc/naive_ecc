use crate::elliptic_curve::point::point::{PointAffine,PointProjective};
use crate::elliptic_curve::field::FieldElement;
use num_bigint::BigInt;
use num_traits::{Zero,One};

pub trait PointOperations{

    fn multiply(&self, other:Self, n:BigInt) -> Self;

    fn double(&self, other:Self) -> Self;

    fn add(&self , other:Self) -> Self;

}

