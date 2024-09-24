use ark_ff::{FftField, Field, One, PrimeField, UniformRand, Zero};
use ark_serialize::{
    CanonicalDeserialize, CanonicalDeserializeWithFlags, CanonicalSerialize,
    CanonicalSerializeWithFlags, Compress, Flags, Read, SerializationError, Valid, Validate, Write,
};
use num_bigint::BigUint;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Iterator, Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{str::FromStr, vec::IntoIter};

#[derive(Debug, Clone, Eq, PartialEq, Copy, Default, Ord, PartialOrd)]
struct Ft<T: PrimeField> {
    inner: T,
}

impl<T: PrimeField> PrimeField for Ft<T> {
    type BigInt = T::BigInt;
    const MODULUS: Self::BigInt = T::MODULUS;
    const MODULUS_MINUS_ONE_DIV_TWO: Self::BigInt = T::MODULUS_MINUS_ONE_DIV_TWO;
    const MODULUS_BIT_SIZE: u32 = T::MODULUS_BIT_SIZE;
    const TRACE: Self::BigInt = T::TRACE;
    const TRACE_MINUS_ONE_DIV_TWO: Self::BigInt = T::TRACE_MINUS_ONE_DIV_TWO;

    fn from_bigint(repr: Self::BigInt) -> Option<Self> {
        T::from_bigint(repr).map(|v| v.into())
    }

    fn into_bigint(self) -> Self::BigInt {
        self.inner.into_bigint()
    }
}

impl<T: PrimeField> FftField for Ft<T> {
    const GENERATOR: Self = from_primefield(T::GENERATOR);
    const TWO_ADICITY: u32 = T::TWO_ADICITY;
    const TWO_ADIC_ROOT_OF_UNITY: Self = from_primefield(T::TWO_ADIC_ROOT_OF_UNITY);
}

impl<T: PrimeField> FromStr for Ft<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(|v| v.into())
    }
}

impl<T: PrimeField> From<BigUint> for Ft<T> {
    fn from(value: BigUint) -> Self {
        Ft::from(value)
    }
}

impl<T: PrimeField> From<Ft<T>> for BigUint {
    fn from(value: Ft<T>) -> BigUint {
        value.inner.into()
    }
}

impl<T: PrimeField> From<T> for Ft<T> {
    fn from(value: T) -> Self {
        Ft { inner: value }
    }
}

impl<T: PrimeField> From<<Self as PrimeField>::BigInt> for Ft<T> {
    fn from(value: <Self as PrimeField>::BigInt) -> Self {
        T::from(value).into()
    }
}

impl<T: PrimeField> Field for Ft<T> {
    type BasePrimeField = Ft<T>;

    type BasePrimeFieldIter = IntoIter<Self::BasePrimeField>;

    const SQRT_PRECOMP: Option<ark_ff::SqrtPrecomputation<Self>> = None;

    const ZERO: Self = from_primefield(T::ZERO);

    const ONE: Self = from_primefield(T::ONE);

    fn extension_degree() -> u64 {
        T::extension_degree()
    }

    fn to_base_prime_field_elements(&self) -> Self::BasePrimeFieldIter {
        self.inner
            .to_base_prime_field_elements()
            .map(|v| v.into())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn from_base_prime_field_elems(elems: &[Self::BasePrimeField]) -> Option<Self> {
        T::from_base_prime_field_elems(elems.iter().map(|v| v.inner).collect::<Vec<_>>().as_slice())
            .map(|v| v.into())
    }

    fn from_base_prime_field(elem: Self::BasePrimeField) -> Self {
        T::from_base_prime_field(elem.inner).into()
    }

    fn double(&self) -> Self {
        Ft::from(self.inner.double())
    }

    fn double_in_place(&mut self) -> &mut Self {
        self.inner.double_in_place();
        self
    }

    fn neg_in_place(&mut self) -> &mut Self {
        self.inner.neg_in_place();
        self
    }

    fn from_random_bytes_with_flags<F: Flags>(bytes: &[u8]) -> Option<(Self, F)> {
        T::from_random_bytes_with_flags(bytes).map(|(v, f)| (v.into(), f))
    }

    fn legendre(&self) -> ark_ff::LegendreSymbol {
        self.inner.legendre()
    }

    fn square(&self) -> Self {
        self.inner.square().into()
    }

    fn square_in_place(&mut self) -> &mut Self {
        self.inner.square_in_place();
        self
    }

    fn inverse(&self) -> Option<Self> {
        self.inner.inverse().map(|v| v.into())
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        let inner = self.inner.inverse_in_place();
        if inner.is_none() {
            None
        } else {
            Some(self)
        }
    }

    fn frobenius_map_in_place(&mut self, power: usize) {
        self.inner.frobenius_map_in_place(power)
    }

    fn sqrt(&self) -> Option<Self> {
        self.inner.sqrt().map(|v| v.into())
    }
}

const fn from_primefield<T: PrimeField>(value: T) -> Ft<T> {
    Ft { inner: value }
}

// Wisdom

impl<T: PrimeField> Display for Ft<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: PrimeField> PartialEq<Self> for Ft<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T: PrimeField> Zero for Ft<T> {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<T: PrimeField> Add<Self, Output = Self> for Ft<T> {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: PrimeField> One for Ft<T> {
    fn one() -> Self {
        todo!()
    }
}

impl<T: PrimeField> Mul<Self, Output = Self> for Ft<T> {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: PrimeField> Neg<Output = Self> for Ft<T> {
    type Output = ();

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl<T: PrimeField> UniformRand for Ft<T> {
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self {
        todo!()
    }
}

impl<T: PrimeField> zeroize::Zeroize for Ft<T> {
    fn zeroize(&mut self) {
        todo!()
    }
}

impl<T: PrimeField> Hash for Ft<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<T: PrimeField> CanonicalSerialize for Ft<T> {
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        todo!()
    }
}

impl<T: PrimeField> CanonicalSerializeWithFlags for Ft<T> {
    fn serialize_with_flags<W: Write, F: Flags>(
        &self,
        writer: W,
        flags: F,
    ) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size_with_flags<F: Flags>(&self) -> usize {
        todo!()
    }
}

impl<T: PrimeField> CanonicalDeserialize for Ft<T> {
    fn deserialize_with_mode<R: Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        todo!()
    }
}

impl<T: PrimeField> Valid for Ft<T> {
    fn check(&self) -> Result<(), SerializationError> {
        todo!()
    }
}

impl<T: PrimeField> CanonicalDeserializeWithFlags for Ft<T> {
    fn deserialize_with_flags<R: Read, F: Flags>(
        reader: R,
    ) -> Result<(Self, F), SerializationError> {
        todo!()
    }
}

impl<T: PrimeField> Sub<Self, Output = Self> for Ft<T> {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: PrimeField> Div<Self, Output = Self> for Ft<T> {
    type Output = ();

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: PrimeField> AddAssign<Self> for Ft<T> {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl<T: PrimeField> SubAssign<Self> for Ft<T> {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl<T: PrimeField> MulAssign<Self> for Ft<T> {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl<T: PrimeField> DivAssign<Self> for Ft<T> {
    fn div_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl<T: PrimeField> Add<&'a Self, Output = Self> for Ft<T> {
    type Output = ();

    fn add(self, rhs: &'a Self) -> Self::Output {
        todo!()
    }
}

impl<T: PrimeField> Sub<&'a Self, Output = Self> for Ft<T> {
    type Output = ();

    fn sub(self, rhs: &'a Self) -> Self::Output {
        todo!()
    }
}

// Francis

impl<'a, T: PrimeField> Mul<&'a Self> for Ft<T> {
    type Output = Ft<T>;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        from_primefield(self.inner.mul(rhs.inner))
    }
}

impl<'a, T: PrimeField> Div<&'a Self> for Ft<T> {
    type Output = Ft<T>;

    fn div(self, rhs: &'a Self) -> Self::Output {
        from_primefield(self.inner.div(rhs.inner))
    }
}

impl<'a, T: PrimeField> AddAssign<&'a Self> for Ft<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.inner.add_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> SubAssign<&'a Self> for Ft<T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.inner.sub_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> MulAssign<&'a Self> for Ft<T> {
    fn mul_assign(&mut self, rhs: &'a Self) {
        self.inner.mul_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> DivAssign<&'a Self> for Ft<T> {
    fn div_assign(&mut self, rhs: &'a Self) {
        self.inner.div_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> Add<&'a mut Self> for Ft<T> {
    type Output = Ft<T>;

    fn add(self, rhs: &'a mut Self) -> Self::Output {
        from_primefield(self.inner.add(rhs.inner))
    }
}

impl<'a, T: PrimeField> Sub<&'a mut Self> for Ft<T> {
    type Output = Ft<T>;

    fn sub(self, rhs: &'a mut Self) -> Self::Output {
        from_primefield(self.inner.sub(rhs.inner))
    }
}

impl<'a, T: PrimeField> Mul<&'a mut Self> for Ft<T> {
    type Output = Ft<T>;

    fn mul(self, rhs: &'a mut Self) -> Self::Output {
        from_primefield(self.inner.mul(rhs.inner))
    }
}

impl<'a, T: PrimeField> Div<&'a mut Self> for Ft<T> {
    type Output = Ft<T>;

    fn div(self, rhs: &'a mut Self) -> Self::Output {
        from_primefield(self.inner.div(rhs.inner))
    }
}

impl<'a, T: PrimeField> AddAssign<&'a mut Self> for Ft<T> {
    fn add_assign(&mut self, rhs: &'a mut Self) {
        self.inner.add_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> SubAssign<&'a mut Self> for Ft<T> {
    fn sub_assign(&mut self, rhs: &'a mut Self) {
        self.inner.sub_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> MulAssign<&'a mut Self> for Ft<T> {
    fn mul_assign(&mut self, rhs: &'a mut Self) {
        self.inner.mul_assign(rhs.inner);
    }
}

impl<'a, T: PrimeField> DivAssign<&'a mut Self> for Ft<T> {
    fn div_assign(&mut self, rhs: &'a mut Self) {
        self.inner.div_assign(rhs.inner);
    }
}

impl<T: PrimeField> Sum<Self> for Ft<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.sum()
    }
}

impl<'a, T: PrimeField> Sum<&'a Self> for Ft<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.sum()
    }
}

impl<T: PrimeField> Product<Self> for Ft<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.product()
    }
}

impl<'a, T: PrimeField> Product<&'a Self> for Ft<T> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.product()
    }
}

impl<T: PrimeField> From<u128> for Ft<T> {
    fn from(value: u128) -> Self {
        from_primefield(value.into())
    }
}

impl<T: PrimeField> From<u64> for Ft<T> {
    fn from(value: u64) -> Self {
        from_primefield(value.into())
    }
}

impl<T: PrimeField> From<u32> for Ft<T> {
    fn from(value: u32) -> Self {
        from_primefield(value.into())
    }
}

impl<T: PrimeField> From<u16> for Ft<T> {
    fn from(value: u16) -> Self {
        from_primefield(value.into())
    }
}

impl<T: PrimeField> From<u8> for Ft<T> {
    fn from(value: u8) -> Self {
        from_primefield(value.into())
    }
}

impl<T: PrimeField> From<bool> for Ft<T> {
    fn from(value: bool) -> Self {
        from_primefield(value.into())
    }
}
