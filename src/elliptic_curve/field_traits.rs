use num_bigint::BigInt;

pub trait FieldElementTraits{

    fn get_number(&self) -> &BigInt;
    
    fn get_prime(&self) -> &BigInt;
    

    fn mod_inv(a0: BigInt,m0:BigInt)-> BigInt;

}


