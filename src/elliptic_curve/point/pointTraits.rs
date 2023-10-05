use crate::elliptic_curve::point::point::{PointAffine,PointProjective};
use crate::elliptic_curve::field::FieldElement;
use num_bigint::BigInt;
use num_traits::{Zero,One};

pub trait PointOperations{

    fn multiply(&self, n:isize) ->Self;

    fn double(&self) -> Self;

    fn add(&self , other: Self) -> Self;

}

