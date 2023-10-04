use num_bigint::{ToBigInt, BigInt};
use num_traits::{One, Zero};
use core::num;
use crate::elliptic_curve::field_traits::FieldElementTraits;
use crate::elliptic_curve::field::FieldElement;

use crate::elliptic_curve::point::pointTraits::PointOperations;

use std::ops::Add;


#[derive(Debug,Clone,PartialEq,Copy)]
pub struct PointAffine<FieldElement>{
    values: [FieldElement;2],
    curve: [FieldElement;2],
     
}


#[derive(Debug,Clone,PartialEq,Copy)]
pub struct PointProjective<FieldElement>{
    values: [FieldElement;3],
    curve: [FieldElement;2],

}


impl PointAffine<FieldElement>{

    pub fn new(x:FieldElement,y:FieldElement,a:FieldElement,b:FieldElement) -> Self{
        PointAffine{values: [x,y],curve: [a,b]}
    }

    pub fn get_x(&self) -> &FieldElement{
        &self.values[0]
        
    }
    pub fn get_y(&self) -> &FieldElement{
        &self.values[1]
        
    }
    pub fn get_values(&self) -> &[FieldElement;2]{
        &self.values
    }

    pub fn get_a(&self) -> &FieldElement{
        &self.curve[0]
        
    }
    pub fn get_b(&self) -> &FieldElement{
        &self.curve[1]
        
    }
    
    pub fn to_projectivez<>(&self, z: FieldElement, a: FieldElement, b:FieldElement) -> PointProjective<FieldElement>{

        let mut x = self.get_x().clone();
        let mut y = self.get_y().clone();
        
        x = (z.modinv() * x);
        y = (z.modinv() * y);

        PointProjective {values: [x,y,z], curve: [a,b]}

    }

    pub fn element_prime(&self)->BigInt{
        self.values[0].get_prime().clone()
    }

}



impl PointProjective<FieldElement>{

    pub fn new(&self,x:FieldElement,y:FieldElement,z:FieldElement,a:FieldElement,b:FieldElement) -> Self{
        PointProjective{values: [x,y,z] , curve: [a,b]}
    }
    pub fn get_x(&self) -> &FieldElement{
        &self.values[0]
        
    }
    pub fn get_y(&self) -> &FieldElement{
        &self.values[1]
        
    }
    pub fn get_z(&self) -> &FieldElement{
        &self.values[2]
        
    }
    pub fn get_a(&self) -> &FieldElement{
        &self.curve[0]
        
    }
    pub fn get_b(&self) -> &FieldElement{
        &self.curve[1]
        
    }

    pub fn normalize_z(&self) -> Self{
        let z_inv =self.get_z().modinv();

        let x = (self.get_x().clone() * z_inv.clone());

        let y = (self.get_y().clone() * z_inv);
        

        let z = FieldElement::one(x.get_prime().clone());
        PointProjective{values: [x, y, z],curve:[self.get_a().clone(),self.get_b().clone()]}
    }

}



impl PointOperations for PointAffine<FieldElement>{
    
    fn add(&self , other: Self) -> Self {
        let slope = ((self.get_y() - other.get_y()) / (self.get_x() - other.get_x())).clone();
        let x_r = (&slope * &slope) - (self.get_x() - other.get_x()); 

        let y_r = slope * (self.get_x() - &x_r) - self.get_y();

        Self{values: [x_r,y_r], curve: [self.get_a().clone(),self.get_b().clone()]}

    }



    // fn multiply(&self, other:Self, n:num_bigint::BigInt) -> Self {}

    fn double(&self) -> Self {
        let three = FieldElement::new_from_i32(3, self.get_x().get_prime().clone()).unwrap();
        let two = FieldElement::new_from_i32(2, self.get_x().get_prime().clone()).unwrap();
        let x = self.get_x().clone();
        let y = self.get_x().clone();

        let slope = (three * self.get_x() * &x + self.get_a()) / (&two * &y);
        
        let x_r = &slope * &slope - &two* &x;
        
        let y_r = &slope*(&x - &x_r) - &y;
        Self{values: [x_r,y_r],curve: [self.curve[0].clone(),self.curve[1].clone()]}

    }

    fn multiply(&self, n:BigInt) -> Self {
        let mut Q = self.clone();

        let prime = self.element_prime().clone();

        let mut R = PointAffine{
            values:[FieldElement::zero(prime.clone()),FieldElement::zero(prime)],
            curve:[self.curve[0].clone(),self.curve[1].clone()]
        };

        let mut n = n;

        while !n.is_zero() {
            let coeff: BigInt = &n % 2;

            if(coeff.is_one()){
                R = R + &Q;
                Q = Q.double();
                
            }
            
            n = n/2;


        }       
        R


    }

}


impl Add<PointAffine<FieldElement>> for PointAffine<FieldElement>{
    type Output = Self;


    fn add(self,other:PointAffine<FieldElement>) -> Self{
        if(self.get_x().get_number() == &BigInt::from(0) && self.get_y().get_number() == &BigInt::from(0) ){
            other
        }
        else if other.get_x() == &FieldElement::zero(self.element_prime()) && other.get_y() == &FieldElement::zero(self.element_prime()) {
            self
        }
        else if (self.get_x() == other.get_x() && self.get_y() == other.get_y()){
            self.double()
        }
        else {
            let slope = (self.get_y() - other.get_y()) / (self.get_x() - other.get_x());
        
            let x_r = &slope * &slope - self.get_x() - other.get_x(); 
    
            let y_r = slope * (self.get_x() - &x_r) - self.get_y();
    
            Self{values: [x_r,y_r],curve: [self.curve[0].clone(),self.curve[1].clone()]}

        }



    }

}


impl Add<&PointAffine<FieldElement>> for PointAffine<FieldElement>{
    type Output = Self;


    fn add(self, other:&PointAffine<FieldElement>) -> Self{
        if(self.get_x().get_number() == &BigInt::from(0) && self.get_y().get_number() == &BigInt::from(0) ){
            other.clone()
        }
        else if other.get_x() == &FieldElement::zero(self.element_prime()) && other.get_y() == &FieldElement::zero(self.element_prime()) {
            self
        }
        else if (self.get_x() == other.get_x() && self.get_y() == other.get_y()){
            self.double()
        }
        else {
            let slope = (self.get_y() - other.get_y()) / (self.get_x() - other.get_x());
        
            let x_r = &slope * &slope - self.get_x() - other.get_x(); 
    
            let y_r = slope * (self.get_x() - &x_r) - self.get_y();
    
            Self{values: [x_r,y_r],curve: [self.curve[0].clone(),self.curve[1].clone()]}

        }



    }

}




#[test]
fn multiply_point(){



}


#[test]
fn double_point(){

    let x = FieldElement::new(BigInt::from(3_u32),BigInt::from(7_u32)).unwrap();
    let y = FieldElement::new(BigInt::from(3_u32),BigInt::from(7_u32)).unwrap();


    let a = FieldElement::new(BigInt::from(0_u32),BigInt::from(7_u32)).unwrap();
    let b = FieldElement::new(BigInt::from(3_u32),BigInt::from(7_u32)).unwrap();


    let x2 = FieldElement::new(BigInt::from(2_u32),BigInt::from(7_u32)).unwrap();
    let y2 = FieldElement::new(BigInt::from(5_u32),BigInt::from(7_u32)).unwrap();


    let p1 = PointAffine::new(x,y,a.clone(),b.clone());
    let p2 = PointAffine::new(x2,y2,a,b);

    let doubled_p1 = p1.double();

    assert_eq!(doubled_p1,p2);
    
}