use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Rem, Sub};

use modc::modc_math::math::Field;
use primitive_types::U256;
pub mod utils;
use utils::error::PolynomialError;

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    pub coeffs: Vec<U256>,
    pub field: Field,
}

impl Polynomial {
    pub fn new<T: Copy + Into<U256>>(coeffs: Vec<T>, f: u64) -> Self {
        let field = Field::new(U256::from(f)).unwrap();
        let mut field_coeffs: Vec<U256> = vec![];
        for i in 0..(coeffs.len()) {
            let coeff = coeffs.get(i).unwrap().to_owned();
            field_coeffs.push(field.self_mod(coeff));
        }
        Self {
            coeffs: field_coeffs,
            field,
        }
    }

    pub fn deg(&self) -> usize {
        self.coeffs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, a)| **a != U256::from(0))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    pub fn coeffs(&self) -> Vec<U256> {
        return self.coeffs.clone();
    }

    pub fn evaluate(&self, value: u64) -> U256 {
        let mut result = U256::from(0);
        for i in 0..self.coeffs.len() {
            result = result
                + (self.field.mult(
                    self.coeffs.get(i).unwrap(),
                    &self.field.pow(value, i as u64).unwrap(),
                ))
                .unwrap();
        }
        result
    }

    pub fn add(self, rhs: Polynomial) -> Result<Polynomial, PolynomialError> {
        if self.field != rhs.field {
            eprintln!("Fields are different. Operation not possible");
            return Err(PolynomialError::FieldNotEqual);
        }
        let mut sum: Vec<U256> = vec![];
        for i in 0..usize::max(self.deg(), rhs.deg()) + 1 {
            sum.push(
                self.field
                    .add(
                        self.coeffs.get(i).unwrap_or(&U256::from(0)).as_u64(),
                        rhs.coeffs.get(i).unwrap_or(&U256::from(0)).as_u64(),
                    )
                    .unwrap(),
            );
        }
        Ok(Polynomial {
            coeffs: sum,
            field: self.field,
        })
    }

    pub fn sub(self, rhs: Polynomial) -> Result<Polynomial, PolynomialError> {
        if self.field != rhs.field {
            eprintln!("Fields are different. Operation not possible");
            return Err(PolynomialError::FieldNotEqual);
        }
        let mut sum: Vec<U256> = vec![];
        for i in 0..usize::max(self.deg(), rhs.deg()) + 1 {
            sum.push(
                self.field
                    .sub(
                        self.coeffs.get(i).unwrap_or(&U256::from(0)).as_u64(),
                        rhs.coeffs.get(i).unwrap_or(&U256::from(0)).as_u64(),
                    )
                    .unwrap(),
            );
        }
        Ok(Polynomial {
            coeffs: sum,
            field: self.field,
        })
    }
    
    /// Find multiplication of 2 polynomials in a finite field
    pub fn mult(self, rhs: Polynomial) -> Result<Polynomial, PolynomialError> {
        if self.field != rhs.field {
            eprintln!("Fields are different. Operation not possible");
            return Err(PolynomialError::FieldNotEqual);
        }
        let mut sum: Vec<U256> = vec![];
        for i in 0..usize::max(self.deg(), rhs.deg()) + 1 {
            sum.push(
                self.field
                    .mult(
                        self.coeffs.get(i).unwrap_or(&U256::from(0)).as_u64(),
                        rhs.coeffs.get(i).unwrap_or(&U256::from(0)).as_u64(),
                    )
                    .unwrap(),
            );
        }
        Ok(Polynomial {
            coeffs: sum,
            field: self.field,
        })
    }

    /// Finds roots of a polynomial mod `p` using brute force approach
    pub fn find_roots(&self,modulus:U256) -> Vec<U256> {
        let mut roots = Vec::new();

        for x in 0..modulus.as_usize() {
            let mut value = U256::zero();
            let mut power = U256::from(1); 
            for coeff in self.coeffs() {
                value =  self.field.add(value, self.field.mult(coeff, power).unwrap_or(U256::zero())).unwrap();
                power =  self.field.mult(power, x.into()).unwrap();
            }
            if value == U256::zero() {
                roots.push(self.field.self_mod(x));
            }
        }

        roots
    }


}
