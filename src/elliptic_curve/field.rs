use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero, Signed, FromPrimitive};
use crate::elliptic_curve::field_traits::FieldElementTraits;


use std::{ops::{Add,Sub,Div}};
use crate::elliptic_curve::errors::FieldErrors;

use std::fmt::{self,format};


#[derive(Debug,Clone,PartialEq)]  
pub struct FieldElement{
    n:BigInt,
    p:BigInt,
}

impl FieldElement{
    pub fn new(number:BigInt,prime:BigInt) -> Result<FieldElement,FieldErrors>{
        if number>=prime || number.is_negative(){
            return Err(FieldErrors::InvalidParams("uglum bu nasil parametreler".to_string()))
        }
        else{
            return Ok(FieldElement { n: number, p: prime })
        }
    }

}

impl FieldElementTraits for FieldElement{


    fn get_number(&self) -> &BigInt{
        &self.n
    }

    fn get_prime(&self) -> &BigInt{
        &self.p
    }

    fn mod_inv(a0: BigInt, m0: BigInt) -> FieldElement {
        let prime = m0.clone();
        if m0 == BigInt::one() {return Self{n:BigInt::one(),p:prime}}
        let (mut a, mut m, mut x0, mut inv) = (a0, m0.clone(), BigInt::zero(), BigInt::one());
        while a > BigInt::one() {
        inv -= (&a / &m) * &x0;
        a = (&a % &m);
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv)
        }
        if inv < BigInt::zero() { inv += m0 }
        Self {n: inv,p: prime}
        }

}

impl Add for FieldElement{
    type Output = Result<FieldElement,FieldErrors>;

    fn add(self,other: FieldElement)->Result<FieldElement,FieldErrors>{
        if(self.p != other.p){
            Err(FieldErrors::Mismatch("uglum bu primeler ne".to_string()))
        }
        else{
            let added = self.n + other.n;
            let num: BigInt = added % &self.p;
           Ok(Self {
                 n: num,
                 p: self.p.clone() })
        }

    }
}

impl<'a> Add<&'a FieldElement> for FieldElement{
    type Output = Result<FieldElement,FieldErrors>;

    fn add(self,other: &'a FieldElement)-> Self::Output{
        if(self.p != other.p){
            Err(FieldErrors::Mismatch("uglum bu primeler ne".to_string()))
        }
        else{
            let added = self.n + &other.n;
            let num: BigInt = added % &self.p;
           Ok(Self {
                 n: num,
                 p: self.p.clone() })
        }

    }
}

impl<'a> Add<&'a FieldElement> for &'a FieldElement{
    type Output = Result<FieldElement,FieldErrors>;

    fn add(self,other: &'a FieldElement)-> Self::Output{
        if(self.p != other.p){
            Err(FieldErrors::Mismatch("uglum bu primeler ne".to_string()))
        }
        else{
            let added = self.get_number() + other.get_number();
            let num: BigInt = added % &self.p;
           Ok(FieldElement {
                 n: num,
                 p: self.p.clone() })
        }

    }
}


impl Sub for FieldElement{
    type Output = Self;

    fn sub(self,other: FieldElement)->FieldElement{
        let mut num = (self.n - other.n) % &self.p;
        if num.is_negative(){
            num += &self.p;
            return FieldElement{n:num,p: self.p.clone()};
        }   
        Self { n: num, p: other.p }
    }
}


impl Div for FieldElement{
    type Output = Self;

    fn div(self,other: FieldElement)->FieldElement{
        
        let inv = FieldElement::mod_inv(other.n,other.p);

        let num = (&self.n * inv.n) % &self.p;  

        FieldElement{n:num,p:self.p.clone()}
    }
}


impl fmt::Display for FieldElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f,"Number : {} , Prime Field :{}",self.get_number(),self.get_prime())


    }
}




#[test]
fn add_test(){
    let a = FieldElement{n:BigInt::from(2_u32),p:BigInt::from(7_u32)};
    let b = FieldElement{n:BigInt::from(6_u32),p:BigInt::from(7_u32)};
    let c = FieldElement{n:BigInt::from(1_u32),p:BigInt::from(7_u32)};
    assert_eq!((a+b).unwrap(),c);

}

#[test]
fn div_test(){
    let a = FieldElement{n:BigInt::from(2_u32),p:BigInt::from(7_u32)};
    let b = FieldElement{n:BigInt::from(6_u32),p:BigInt::from(7_u32)};
    let c = FieldElement{n:BigInt::from(5_u32),p:BigInt::from(7_u32)};

    assert_eq!((a/b),c);

}

