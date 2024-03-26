
pub trait Group : Sized + Eq {

    pub fn zero() -> Self;

    pub fn inverse(&self) -> Self;

    pub fn group_law(&self, other: &Self) -> Self;

}

pub trait Ring: Sized + Eq {

    pub fn zero() -> Self;

    pub fn one() -> Self;

    pub fn additive_inverse(&self) -> Self;

    pub fn add(&self, other: &Self) -> Self;

    pub fn mul(&self, other: &Self) -> Self;

}

pub trait Field: Sized + Eq {

    pub fn zero() -> Self;

    pub fn one() -> Self;

    pub fn additive_inverse(&self) -> Self;
    
    pub fn multiplicative_inverse(&self) -> Self;

    pub fn add(&self, other: &Self) -> Self;

    pub fn mul(&self, other: &Self) -> Self;

}
