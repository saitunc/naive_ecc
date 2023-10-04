use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero, Signed, FromPrimitive};
use crate::elliptic_curve::field_traits::FieldElementTraits;


use std::ops::{Add,Sub,Div,Mul};
use crate::elliptic_curve::errors::FieldErrors;

use std::fmt::{self};


#[derive(Debug,Clone,PartialEq)]  
pub struct FieldElement{
    n:BigInt,
    p:BigInt,
}

impl FieldElement{
    pub fn new(number:BigInt,prime:BigInt) -> Result<FieldElement,FieldErrors>{
        if number>=prime || number.is_negative(){
            return Err(FieldErrors::InvalidParams("Bad Parameters".to_string()))
        }
        else{
            return Ok(FieldElement { n: number, p: prime })
        }
    }
    pub fn new_from_i32(number:i32,prime:BigInt) -> Result<FieldElement,FieldErrors>{
        let number = BigInt::from_i32(number).unwrap(); 
        if number>=prime || number.is_negative(){
            return Err(FieldErrors::InvalidParams("Bad Parameters".to_string()))
        }
        else{
            return Ok(FieldElement { n: number, p: prime })
        }
    }

    pub fn zero(p:BigInt)->FieldElement{
        FieldElement { n: BigInt::from(0), p: p }
    }

    pub fn one(p:BigInt)->FieldElement{
        FieldElement { n: BigInt::from(1), p: p }
    }


}

impl FieldElementTraits for FieldElement{

    fn get_number(&self) -> &BigInt{
        &self.n
    }

    fn get_prime(&self) -> &BigInt{
        &self.p
    }

    fn modinv(&self) -> FieldElement{
        let prime = self.p.clone();
        if prime == BigInt::one() {return Self{n:BigInt::one(),p:prime}}
        let (mut a, mut m, mut x0, mut inv) = (self.get_number().clone(), self.get_prime().clone(), BigInt::zero(), BigInt::one());

        while a > BigInt::one() {
            inv -= (&a / &m) * &x0;
            a = (&a % &m);
            std::mem::swap(&mut a, &mut m);
            std::mem::swap(&mut x0, &mut inv)
            }
            if inv < BigInt::zero() { inv += &prime.clone() }
            Self {n: inv,p: prime.clone()}
    }

}

impl Add<FieldElement> for FieldElement{
    type Output = FieldElement;

    fn add(self,other: FieldElement) -> Self::Output{
            let added = self.n + other.n;
            let num: BigInt = added % self.p.clone();
            Self::Output {n: num, p: self.p}
        }
}

impl Add<&FieldElement> for FieldElement{
    type Output = FieldElement;
    fn add(self,other: &FieldElement)->Self::Output{
            let added = self.n + &other.n;
            let num: BigInt = added % self.p.clone();
            Self::Output {n: num, p: self.p }

    }
}


impl Add<FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn add(self,other: FieldElement) -> Self::Output{
        let prime = self.p.clone();
        let added = &self.n + other.n;
        let num = added % prime;
        Self::Output {n: num, p: other.p}
    }
}

impl Add<&FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn add(self,other: &FieldElement) -> Self::Output{
        let added = &self.n + &other.n;
        let num: BigInt = added % &self.p;

        Self::Output {n: num, p: other.p.clone()}
    }
}


impl Sub<FieldElement> for FieldElement{
    type Output = FieldElement;

    fn sub(self,other: FieldElement) -> Self::Output{

        let mut num = (self.n - other.n) % &self.p;
        if num.is_negative(){
            num += &self.p;
            return Self::Output{n:num,p: self.p};
        }   
        Self::Output { n: num, p: other.p }
    }
    }

impl Sub<FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn sub(self,other: FieldElement) -> Self::Output{

        let mut num = (&self.n - other.n) % &self.p;
        if num.is_negative(){
            num += &self.p;

            return Self::Output { n: num, p: other.p };
        }   
        Self::Output { n: num, p: other.p }
    }
    
}

impl Sub<&FieldElement> for FieldElement{
    type Output = FieldElement;

    fn sub(self,other: &FieldElement) -> Self::Output{

        let mut num = (self.n - &other.n) % &self.p;
        if num.is_negative(){
            num += &self.p;
            return Self::Output { n: num, p: self.p };
        }   
        Self::Output { n: num, p: self.p }
    }
    }


impl Sub<&FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn sub(self,other: &FieldElement)->Self::Output{

        let mut num = (&self.n - &other.n) % &self.p;
        if num.is_negative(){
            num += &self.p;
            return Self::Output { n: num, p: other.p.clone() }
        }   
        Self::Output { n: num, p: other.p.clone() }
    }
}


impl Mul<FieldElement> for FieldElement{
    type Output = FieldElement;

    fn mul(self,other:FieldElement )-> Self::Output{

        let num = (self.n * other.n) % &self.p;
        Self::Output { n: num, p: self.p }

    }

}


impl Mul<&FieldElement> for FieldElement{
    type Output = FieldElement;

    fn mul(self,other: &FieldElement )-> Self::Output{
            let num = (self.n * &other.n) % &self.p;

            Self::Output { n: num, p: self.p }
    }

}


impl Mul<FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn mul(self,other: FieldElement )-> Self::Output{
        let num = (&self.n * other.n) % &self.p;
        Self::Output { n: num, p: self.p.clone() }

    }

}



impl Mul<&FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn mul(self,other: &FieldElement )-> Self::Output{

        let num = (&self.n * &other.n) % &self.p;

        Self::Output { n: num, p: self.p.clone() }
        }
    }




impl Div<FieldElement> for FieldElement{
    type Output = FieldElement;

    fn div(self,other: FieldElement)->Self::Output{

        let inv = other.modinv();

        let num = (self.n * inv.n) % self.p;  

        Self::Output { n: num, p: other.p }
    }
}


impl Div<FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn div(self,other: FieldElement) -> Self::Output{
        
        let inv = other.modinv();

        let num = (&self.n * &inv.n) % &self.p;  

        Self::Output { n: num, p: other.p }   
    }
}


impl Div<&FieldElement> for FieldElement{
    type Output = FieldElement;

    fn div(self,other: &FieldElement) -> Self::Output{
        
        let inv = &other.modinv();

        let num = (self.n * &inv.n) % &self.p;  

        Self::Output { n: num, p: self.p }    
    }
}



impl Div<&FieldElement> for &FieldElement{
    type Output = FieldElement;

    fn div(self,other: &FieldElement) -> Self::Output{
        
        let inv = &other.modinv();

        let num = (&self.n * &inv.n) % &self.p;  

        Self::Output { n: num, p: other.p.clone() }
    }
}




impl fmt::Display for FieldElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f,"Number : {} , Prime Field :{}",self.get_number(),self.get_prime())

    }
}




#[test]
fn add_tests(){
    let a = FieldElement{n:BigInt::from(2_u32),p:BigInt::from(7_u32)};
    let b = FieldElement{n:BigInt::from(6_u32),p:BigInt::from(7_u32)};
    let c = FieldElement{n:BigInt::from(1_u32),p:BigInt::from(7_u32)};
    assert_eq!((&a+&b),c);

}


#[test]
fn mul_tests(){
    let a = FieldElement{n:BigInt::from(2_u32),p:BigInt::from(7_u32)};
    let b = FieldElement{n:BigInt::from(6_u32),p:BigInt::from(7_u32)};
    let c = FieldElement{n:BigInt::from(5_u32),p:BigInt::from(7_u32)};
    assert_eq!((&a*&b),c);

    let a = &a;
    
    assert_eq!((a*&b),c);

    let b = &b;
    
    assert_eq!((a*b),c);
    

    assert_eq!((a*b),c);

}

#[test]
fn div_test(){
    let a = FieldElement{n:BigInt::from(2_u32),p:BigInt::from(7_u32)};
    let b = FieldElement{n:BigInt::from(6_u32),p:BigInt::from(7_u32)};
    let c = FieldElement{n:BigInt::from(5_u32),p:BigInt::from(7_u32)};

    assert_eq!((&a/&b),c);

    let a = &a;
    
    assert_eq!((a/&b),c);

    let b = &b;
    
    assert_eq!((a/b),c);
    

}

#[test]

fn inv_test(){
    let a = FieldElement{n:BigInt::from(6_u32),p:BigInt::from(7_u32)};

    let a_inv = a.modinv();

    assert_eq!(a_inv,a);

}