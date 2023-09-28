use num_bigint::BigInt;
use num_traits::Zero;
use crate::elliptic_curve::field::FieldElement;
use crate::elliptic_curve::field_traits::FieldElementTraits;

use std::ops::{Add};
use std::fmt::{self, format};
use std::cmp::max;

#[derive(PartialEq,Debug,Clone)]
pub struct Polynomial<FieldElement>{
    coefficients:Vec<FieldElement>
}


impl Add for Polynomial<FieldElement>{
    type Output = Polynomial<FieldElement>;

    fn add(self,other:Polynomial<FieldElement>) -> Self{

        let l1 = self.coefficients.len();
        let l2 = other.coefficients.len();
        let max_length = max(l1,l2);

        let first_poly = &self.coefficients;

        let mut result = Polynomial{coefficients:vec![FieldElement::new(BigInt::zero(),first_poly[0].get_prime().clone()).unwrap();max_length]};

        for i in 0..l1{
            result.coefficients[i] = (result.coefficients[i].clone() + self.coefficients[i].clone()).unwrap(); 
        }

        for i in 0..l2{
            result.coefficients[i] = (&result.coefficients[i] + &other.coefficients[i]).unwrap(); 
        }

        result

    }
}

impl fmt::Display for Polynomial<FieldElement>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let length = self.coefficients.len();
        let mut output_string = String::new();

        for i in 0..length{
            output_string.push_str(&format!(" {}*x^{} +",self.coefficients[i].get_number(),i));
        }
        output_string.pop();

        write!(f,"P(x) = {}",output_string)


    }
}


impl Polynomial<FieldElement>{

    pub fn evaluate(&self,x: FieldElement) -> FieldElement{

        let length = self.coefficients.len();

        let mut sum = BigInt::from(0);

        for i in 0..length{
            sum += self.coefficients[i].get_number() * x.get_number();

        }
        let prim = x.get_prime().clone();
        FieldElement::new(sum % &prim, prim).unwrap()

    }


}



#[test]
fn polynomial_add(){
    let a1 = FieldElement::new(BigInt::from(3),BigInt::from(7)).unwrap();
    let a2 = FieldElement::new(BigInt::from(2),BigInt::from(7)).unwrap();
    let a3 = FieldElement::new(BigInt::from(4),BigInt::from(7)).unwrap();
    let a4 = FieldElement::new(BigInt::from(1),BigInt::from(7)).unwrap();

    let b1 = FieldElement::new(BigInt::from(5),BigInt::from(7)).unwrap();
    let b2 = FieldElement::new(BigInt::from(1),BigInt::from(7)).unwrap();
    let b3 = FieldElement::new(BigInt::from(4),BigInt::from(7)).unwrap();
    let b4 = FieldElement::new(BigInt::from(5),BigInt::from(7)).unwrap();

    let poly1_vector = vec![a1,a2,a3,a4];
    let poly2_vector = vec![b1,b2,b3,b4];

    let poly1 = Polynomial{coefficients:poly1_vector};
    let poly2 = Polynomial{coefficients:poly2_vector};

    let poly3 = poly1.clone() + poly2.clone();

    println!("{}",poly1);

    println!("{}",poly2);

    println!("{}",poly3);

    assert_eq!(poly1+poly2,poly3);



}
#[test]
fn polynomial_print(){
    let a1 = FieldElement::new(BigInt::from(3),BigInt::from(7)).unwrap();
    let a2 = FieldElement::new(BigInt::from(2),BigInt::from(7)).unwrap();
    let a3 = FieldElement::new(BigInt::from(4),BigInt::from(7)).unwrap();
    let a4 = FieldElement::new(BigInt::from(1),BigInt::from(7)).unwrap();

    let poly1_vector = vec![a1,a2,a3,a4];


    let poly1 = Polynomial{coefficients:poly1_vector};
    println!("{}",poly1);



}

#[test]
fn evaluate_polynomial(){

    let a1 = FieldElement::new(BigInt::from(3),BigInt::from(7)).unwrap();
    let a2 = FieldElement::new(BigInt::from(2),BigInt::from(7)).unwrap();
    let a3 = FieldElement::new(BigInt::from(4),BigInt::from(7)).unwrap();
    let a4 = FieldElement::new(BigInt::from(1),BigInt::from(7)).unwrap();

    let poly1_vector = vec![a1,a2,a3,a4];


    let poly1 = Polynomial{coefficients:poly1_vector};

    let result = poly1.evaluate(FieldElement::new(BigInt::from(3),BigInt::from(7)).unwrap());

    println!("{}",result);
    

}