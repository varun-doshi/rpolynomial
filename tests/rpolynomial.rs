#[cfg(test)]
extern crate rpolynomial;
use crate::rpolynomial::utils::error::PolynomialError;
use crate::rpolynomial::Polynomial;
use primitive_types::U256;

#[test]
fn polynomial_initialization_from_u256() {
    let p = Polynomial::new(
        vec![U256::from(2), U256::from(5), U256::from(7), U256::from(9)],
        7,
    );

    println!(
        "Initialized Polynomial= [{},{},{},{}]",
        p.coeffs[0], p.coeffs[1], p.coeffs[2], p.coeffs[3]
    );

    assert_eq!(p.deg(), 3);

    assert_eq!(p.coeffs[0], U256::from(2));
    assert_eq!(p.coeffs[1], U256::from(5));
    assert_eq!(p.coeffs[2], U256::from(0));
    assert_eq!(p.coeffs[3], U256::from(2));
}

#[test]
fn polynomial_initialization_from_u64() {
    let p = Polynomial::new(vec![2u64, 5u64, 7u64, 9u64], 7);

    println!(
        "Initialized Polynomial= [{},{},{},{}]",
        p.coeffs[0], p.coeffs[1], p.coeffs[2], p.coeffs[3]
    );

    assert_eq!(p.deg(), 3);

    assert_eq!(p.coeffs[0], U256::from(2));
    assert_eq!(p.coeffs[1], U256::from(5));
    assert_eq!(p.coeffs[2], U256::from(0));
    assert_eq!(p.coeffs[3], U256::from(2));
}

#[test]
fn polynomial_addition() {
    let p1 = Polynomial::new(vec![2, 5, 7, 9], 7);
    let p2 = Polynomial::new(vec![1, 2, 3, 4, 5], 7);

    let sum = p1.add(p2).unwrap();

    println!(
        "Added Polynomial= [{},{},{},{},{}]",
        sum.coeffs[0], sum.coeffs[1], sum.coeffs[2], sum.coeffs[3], sum.coeffs[4]
    );

    assert_eq!(sum.deg(), 4);

    assert_eq!(sum.coeffs[0], U256::from(3));
    assert_eq!(sum.coeffs[1], U256::from(0));
    assert_eq!(sum.coeffs[2], U256::from(3));
    assert_eq!(sum.coeffs[3], U256::from(6));
    assert_eq!(sum.coeffs[4], U256::from(5));
}

#[test]
fn fail_polynomial_addition() {
    let p1 = Polynomial::new(
        vec![U256::from(2), U256::from(5), U256::from(7), U256::from(9)],
        9,
    );
    let p2 = Polynomial::new(
        vec![
            U256::from(1),
            U256::from(2),
            U256::from(3),
            U256::from(4),
            U256::from(5),
        ],
        7,
    );

    let sum = p1.add(p2);

    assert!(sum.is_err());
    assert_eq!(sum.unwrap_err(), PolynomialError::FieldNotEqual);
}


#[test]
fn polynomial_subtration() {
    let p1 = Polynomial::new(vec![2, 5, 7, 9], 7);
    let p2 = Polynomial::new(vec![1, 2, 3, 4, 5], 7);

    let diff = p1.sub(p2).unwrap();

    println!(
        "Subtracted Polynomial= [{},{},{},{},{}]",
        diff.coeffs[0], diff.coeffs[1], diff.coeffs[2], diff.coeffs[3], diff.coeffs[4]
    );

    assert_eq!(diff.deg(), 4);

    assert_eq!(diff.coeffs[0], U256::from(1));
    assert_eq!(diff.coeffs[1], U256::from(3));
    assert_eq!(diff.coeffs[2], U256::from(4));
    assert_eq!(diff.coeffs[3], U256::from(5));
    assert_eq!(diff.coeffs[4], U256::from(2));
}


#[test]
fn polynomial_multiplication() {
    let p1 = Polynomial::new(vec![2, 5, 7, 9], 7);
    let p2 = Polynomial::new(vec![1, 2, 3, 4, 5], 7);

    let product = p1.mult(p2).unwrap();

    println!(
        "Multiplicated Polynomial= [{},{},{},{},{}]",
        product.coeffs[0], product.coeffs[1], product.coeffs[2], product.coeffs[3], product.coeffs[4]
    );

    assert_eq!(product.deg(), 3);

    assert_eq!(product.coeffs[0], U256::from(2));
    assert_eq!(product.coeffs[1], U256::from(3));
    assert_eq!(product.coeffs[2], U256::from(0));
    assert_eq!(product.coeffs[3], U256::from(1));
    assert_eq!(product.coeffs[4], U256::from(0));
}


#[test]
fn polynomial_roots() {
    let p1 = Polynomial::new(vec![2, 5, 7, 9], 7);

    let roots = p1.find_roots(7.into());

    println!("Roots: {:?}",roots);

    assert_eq!(roots.len(), 1);

    assert_eq!(roots, vec![U256::from(2)]);
   
}