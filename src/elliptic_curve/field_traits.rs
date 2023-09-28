use num_bigint::BigInt;
use crate::elliptic_curve::field::FieldElement;
pub trait FieldElementTraits{

    fn get_number(&self) -> &BigInt;
    
    fn get_prime(&self) -> &BigInt;
    
    fn mod_inv(a0: BigInt,m0:BigInt)-> FieldElement;

}


