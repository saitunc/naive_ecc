use num_bigint::BigInt;
use crate::elliptic_curve::field::FieldElement;
pub trait FieldElementTraits{

    fn get_number(&self) -> &BigInt;
    
    fn get_prime(&self) -> &BigInt;

    fn modinv(&self) -> FieldElement;
}


