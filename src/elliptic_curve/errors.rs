use std::fmt;

use std::fmt::Formatter;


#[derive(Debug,Clone,PartialEq)]
pub enum FieldErrors{
    InvalidParams(String),
    PointNotOnCurve(String),
    Mismatch(String),
}


impl fmt::Display for FieldErrors{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        match self{
            FieldErrors::InvalidParams(errmsg) => {write!(f, "Invalid Parameter Error: {}",errmsg)}
            FieldErrors::PointNotOnCurve(errmsg) => {write!(f, "Point & Curve Error: {}",errmsg)}
            FieldErrors::Mismatch(errmsg) => {write!(f,"Mismatch Error: {}",errmsg)}

        }

    }
}