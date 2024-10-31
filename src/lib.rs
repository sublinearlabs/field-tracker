use ark_ff::{AdditiveGroup, BigInt, FftField, Field, One, PrimeField, UniformRand, Zero};
use ark_serialize::{
    CanonicalDeserialize, CanonicalDeserializeWithFlags, CanonicalSerialize,
    CanonicalSerializeWithFlags, Compress, Flags, Read, SerializationError, Valid, Validate, Write,
};
use num_bigint::BigUint;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Iterator, Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;
use tracker::{update_add, update_inv, update_mul, Report, Tracker};

pub mod tracker;
pub mod util;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Default, Ord, PartialOrd)]
pub struct Ft<const N: usize, T: PrimeField> {
    inner: T,
}

impl<const N: usize, T: PrimeField> Ft<N, T> {
    pub fn start(name: &'static str) {
        Tracker::start(name);
    }

    pub fn end() {
        Tracker::end();
    }

    pub fn summary() -> Report {
        Tracker::summary()
    }
}

impl<const N: usize, T: PrimeField<BigInt = BigInt<N>>> PrimeField for Ft<N, T> {
    type BigInt = BigInt<N>;
    const MODULUS: Self::BigInt = T::MODULUS;
    const MODULUS_MINUS_ONE_DIV_TWO: Self::BigInt = T::MODULUS_MINUS_ONE_DIV_TWO;
    const MODULUS_BIT_SIZE: u32 = T::MODULUS_BIT_SIZE;
    const TRACE: Self::BigInt = T::TRACE;
    const TRACE_MINUS_ONE_DIV_TWO: Self::BigInt = T::TRACE_MINUS_ONE_DIV_TWO;

    fn from_bigint(repr: Self::BigInt) -> Option<Self> {
        T::from_bigint(repr).map(|v| from_primefield(v))
    }

    fn into_bigint(self) -> Self::BigInt {
        self.inner.into_bigint()
    }
}

impl<const N: usize, T: PrimeField<BigInt = BigInt<N>>> FftField for Ft<N, T> {
    const GENERATOR: Self = from_primefield(T::GENERATOR);
    const TWO_ADICITY: u32 = T::TWO_ADICITY;
    const TWO_ADIC_ROOT_OF_UNITY: Self = from_primefield(T::TWO_ADIC_ROOT_OF_UNITY);
}

impl<const N: usize, T: PrimeField> FromStr for Ft<N, T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(|v| from_primefield(v))
    }
}

impl<const N: usize, T: PrimeField> From<BigUint> for Ft<N, T> {
    fn from(value: BigUint) -> Self {
        from_primefield(T::from(value))
    }
}

impl<const N: usize, T: PrimeField> From<Ft<N, T>> for BigUint {
    fn from(value: Ft<N, T>) -> BigUint {
        value.inner.into()
    }
}

impl<const N: usize, T: PrimeField<BigInt = BigInt<N>>> From<BigInt<N>> for Ft<N, T> {
    fn from(value: BigInt<N>) -> Self {
        from_primefield(T::from_bigint(value).unwrap())
    }
}

impl<const N: usize, T: PrimeField<BigInt = BigInt<N>>> From<Ft<N, T>> for BigInt<N> {
    fn from(value: Ft<N, T>) -> Self {
        value.into_bigint()
    }
}

impl<const N: usize, T: PrimeField<BigInt = BigInt<N>>> Field for Ft<N, T> {
    type BasePrimeField = Ft<N, T>;

    const SQRT_PRECOMP: Option<ark_ff::SqrtPrecomputation<Self>> = None;

    const ONE: Self = from_primefield(T::ONE);

    fn extension_degree() -> u64 {
        T::extension_degree()
    }

    fn to_base_prime_field_elements(
        &self,
    ) -> impl Iterator<Item = <Self as Field>::BasePrimeField> {
        self.inner
            .to_base_prime_field_elements()
            .map(|v| from_primefield(v))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn from_base_prime_field_elems(
        elems: impl IntoIterator<Item = Self::BasePrimeField>,
    ) -> Option<Self> {
        T::from_base_prime_field_elems(elems.into_iter().map(|v| v.inner).collect::<Vec<_>>())
            .map(|v| from_primefield(v))
    }

    fn from_base_prime_field(elem: Self::BasePrimeField) -> Self {
        from_primefield(T::from_base_prime_field(elem.inner))
    }

    fn from_random_bytes_with_flags<F: Flags>(bytes: &[u8]) -> Option<(Self, F)> {
        T::from_random_bytes_with_flags(bytes).map(|(v, f)| (from_primefield(v), f))
    }

    fn legendre(&self) -> ark_ff::LegendreSymbol {
        self.inner.legendre()
    }

    fn square(&self) -> Self {
        from_primefield(self.inner.square())
    }

    fn square_in_place(&mut self) -> &mut Self {
        self.inner.square_in_place();
        self
    }

    fn inverse(&self) -> Option<Self> {
        update_inv();
        self.inner.inverse().map(|v| from_primefield(v))
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        update_inv();
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
        self.inner.sqrt().map(|v| from_primefield(v))
    }

    fn mul_by_base_prime_field(&self, elem: &Self::BasePrimeField) -> Self {
        update_mul();
        from_primefield(self.inner.mul_by_base_prime_field(&elem.inner))
    }
}

impl<const N: usize, T: PrimeField<BigInt = BigInt<N>>> AdditiveGroup for Ft<N, T> {
    type Scalar = Ft<N, T>;
    const ZERO: Self = from_primefield(T::ZERO);
}

const fn from_primefield<const N: usize, T: PrimeField>(value: T) -> Ft<N, T> {
    Ft { inner: value }
}

impl<const N: usize, T: PrimeField> Display for Ft<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl<const N: usize, T: PrimeField> Zero for Ft<N, T> {
    fn zero() -> Self {
        from_primefield(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
}

impl<const N: usize, T: PrimeField> Add<Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn add(self, rhs: Self) -> Self::Output {
        update_add();
        from_primefield(self.inner.add(rhs.inner))
    }
}

impl<const N: usize, T: PrimeField> One for Ft<N, T> {
    fn one() -> Self {
        from_primefield(T::one())
    }
}

impl<const N: usize, T: PrimeField> Mul<Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn mul(self, rhs: Self) -> Self::Output {
        update_mul();
        from_primefield(self.inner.mul(rhs.inner))
    }
}

impl<const N: usize, T: PrimeField> Neg for Ft<N, T> {
    type Output = Ft<N, T>;

    fn neg(self) -> Self::Output {
        from_primefield(self.inner.neg())
    }
}

impl<const N: usize, T: PrimeField> UniformRand for Ft<N, T> {
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self {
        from_primefield(T::rand(rng))
    }
}

impl<const N: usize, T: PrimeField> zeroize::Zeroize for Ft<N, T> {
    fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}

impl<const N: usize, T: PrimeField> Hash for Ft<N, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<const N: usize, T: PrimeField> CanonicalSerialize for Ft<N, T> {
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.inner.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.inner.serialized_size(compress)
    }
}

impl<const N: usize, T: PrimeField> CanonicalSerializeWithFlags for Ft<N, T> {
    fn serialize_with_flags<W: Write, F: Flags>(
        &self,
        writer: W,
        flags: F,
    ) -> Result<(), SerializationError> {
        self.inner.serialize_with_flags(writer, flags)
    }

    fn serialized_size_with_flags<F: Flags>(&self) -> usize {
        self.inner.serialized_size_with_flags::<F>()
    }
}

impl<const N: usize, T: PrimeField> CanonicalDeserialize for Ft<N, T> {
    fn deserialize_with_mode<R: Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        T::deserialize_with_mode(reader, compress, validate).map(|v| from_primefield(v))
    }
}

impl<const N: usize, T: PrimeField> Valid for Ft<N, T> {
    fn check(&self) -> Result<(), SerializationError> {
        self.inner.check()
    }
}

impl<const N: usize, T: PrimeField> CanonicalDeserializeWithFlags for Ft<N, T> {
    fn deserialize_with_flags<R: Read, F: Flags>(
        reader: R,
    ) -> Result<(Self, F), SerializationError> {
        T::deserialize_with_flags(reader).map(|v| (from_primefield(v.0), v.1))
    }
}

impl<const N: usize, T: PrimeField> Sub<Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        from_primefield(self.inner.sub(rhs.inner))
    }
}

impl<const N: usize, T: PrimeField> Div<Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn div(self, rhs: Self) -> Self::Output {
        from_primefield(self.inner.div(rhs.inner))
    }
}

impl<const N: usize, T: PrimeField> AddAssign<Self> for Ft<N, T> {
    fn add_assign(&mut self, rhs: Self) {
        update_add();
        self.inner.add_assign(rhs.inner)
    }
}

impl<const N: usize, T: PrimeField> SubAssign<Self> for Ft<N, T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner.sub_assign(rhs.inner)
    }
}

impl<const N: usize, T: PrimeField> MulAssign<Self> for Ft<N, T> {
    fn mul_assign(&mut self, rhs: Self) {
        update_mul();
        self.inner.mul_assign(rhs.inner)
    }
}

impl<const N: usize, T: PrimeField> DivAssign<Self> for Ft<N, T> {
    fn div_assign(&mut self, rhs: Self) {
        self.inner.div_assign(rhs.inner)
    }
}

impl<'a, const N: usize, T: PrimeField> Add<&'a Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn add(self, rhs: &'a Self) -> Self::Output {
        update_add();
        from_primefield(self.inner.add(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> Sub<&'a Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn sub(self, rhs: &'a Self) -> Self::Output {
        from_primefield(self.inner.sub(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> Mul<&'a Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        update_mul();
        from_primefield(self.inner.mul(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> Div<&'a Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn div(self, rhs: &'a Self) -> Self::Output {
        from_primefield(self.inner.div(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> AddAssign<&'a Self> for Ft<N, T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        update_add();
        self.inner.add_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> SubAssign<&'a Self> for Ft<N, T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.inner.sub_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> MulAssign<&'a Self> for Ft<N, T> {
    fn mul_assign(&mut self, rhs: &'a Self) {
        update_mul();
        self.inner.mul_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> DivAssign<&'a Self> for Ft<N, T> {
    fn div_assign(&mut self, rhs: &'a Self) {
        self.inner.div_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> Add<&'a mut Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn add(self, rhs: &'a mut Self) -> Self::Output {
        update_add();
        from_primefield(self.inner.add(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> Sub<&'a mut Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn sub(self, rhs: &'a mut Self) -> Self::Output {
        from_primefield(self.inner.sub(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> Mul<&'a mut Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn mul(self, rhs: &'a mut Self) -> Self::Output {
        update_mul();
        from_primefield(self.inner.mul(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> Div<&'a mut Self> for Ft<N, T> {
    type Output = Ft<N, T>;

    fn div(self, rhs: &'a mut Self) -> Self::Output {
        from_primefield(self.inner.div(rhs.inner))
    }
}

impl<'a, const N: usize, T: PrimeField> AddAssign<&'a mut Self> for Ft<N, T> {
    fn add_assign(&mut self, rhs: &'a mut Self) {
        update_add();
        self.inner.add_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> SubAssign<&'a mut Self> for Ft<N, T> {
    fn sub_assign(&mut self, rhs: &'a mut Self) {
        self.inner.sub_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> MulAssign<&'a mut Self> for Ft<N, T> {
    fn mul_assign(&mut self, rhs: &'a mut Self) {
        update_mul();
        self.inner.mul_assign(rhs.inner);
    }
}

impl<'a, const N: usize, T: PrimeField> DivAssign<&'a mut Self> for Ft<N, T> {
    fn div_assign(&mut self, rhs: &'a mut Self) {
        self.inner.div_assign(rhs.inner);
    }
}

impl<const N: usize, T: PrimeField> Sum<Self> for Ft<N, T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut iterator = iter;
        let sum = iterator.next();
        iterator
            .fold(sum, |acc, val| acc.map(|v| v + val))
            .unwrap_or(Self::zero())
    }
}

impl<'a, const N: usize, T: PrimeField> Sum<&'a Self> for Ft<N, T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut iterator = iter;
        let sum = iterator.next().cloned();
        iterator
            .fold(sum, |acc, val| acc.map(|v| v + val))
            .unwrap_or(Self::zero())
    }
}

impl<const N: usize, T: PrimeField> Product<Self> for Ft<N, T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut iterator = iter;
        let prod = iterator.next();
        iterator
            .fold(prod, |acc, val| acc.map(|v| v * val))
            .unwrap_or(Self::one())
    }
}

impl<'a, const N: usize, T: PrimeField> Product<&'a Self> for Ft<N, T> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut iterator = iter;
        let prod = iterator.next().cloned();
        iterator
            .fold(prod, |acc, val| acc.map(|v| v * val))
            .unwrap_or(Self::one())
    }
}

impl<const N: usize, T: PrimeField> From<u128> for Ft<N, T> {
    fn from(value: u128) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<u64> for Ft<N, T> {
    fn from(value: u64) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<u32> for Ft<N, T> {
    fn from(value: u32) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<u16> for Ft<N, T> {
    fn from(value: u16) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<u8> for Ft<N, T> {
    fn from(value: u8) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<bool> for Ft<N, T> {
    fn from(value: bool) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField + std::convert::From<i32>> From<i32> for Ft<N, T> {
    fn from(value: i32) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<i8> for Ft<N, T> {
    fn from(value: i8) -> Self {
        from_primefield(value.into())
    }
}
impl<const N: usize, T: PrimeField> From<i16> for Ft<N, T> {
    fn from(value: i16) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<i64> for Ft<N, T> {
    fn from(value: i64) -> Self {
        from_primefield(value.into())
    }
}

impl<const N: usize, T: PrimeField> From<i128> for Ft<N, T> {
    fn from(value: i128) -> Self {
        from_primefield(value.into())
    }
}

#[cfg(test)]
mod test {
    use ark_bn254::Fr;
    use ark_ff::Field;

    use crate::{end_tscope, start_tscope, summary, Ft};

    type F = Ft!(Fr);

    #[test]
    fn test_integration() {
        let num1 = F::from(3);
        let num2 = F::from(7);

        let mut num3 = num1 + num2;

        let mut num4 = num1 * num2;

        num3 += num2;

        let _ = num4.inverse();

        num4.inverse_in_place();

        assert_eq!(F::summary().values.add, 2);
        assert_eq!(F::summary().values.mul, 1);
        assert_eq!(F::summary().values.inv, 2);
    }

    #[test]
    fn test_sum_and_product_iterators() {
        let values = vec![F::from(3), F::from(7), F::from(9)];
        let sum = values.iter().sum::<F>();
        let prod = values.iter().product::<F>();

        assert_eq!(sum, F::from(19));
        assert_eq!(prod, F::from(189));

        assert_eq!(F::summary().values.add, 2);
        assert_eq!(F::summary().values.mul, 2);
    }

    #[test]
    fn test_scope() {
        let num1 = F::from(3);
        let num2 = F::from(7);

        start_tscope!("main");
        let mut num3 = num1 + num2;

        start_tscope!("inner");
        let mut num4 = num1 * num2;
        num3 += num2;
        end_tscope!();

        let _ = num4.inverse();
        end_tscope!();

        num4.inverse_in_place();

        let summary = summary!();

        // global assertions
        assert_eq!(summary.values.add, 2);
        assert_eq!(summary.values.mul, 1);
        assert_eq!(summary.values.inv, 2);

        // main assertions
        assert_eq!(summary.children.as_ref().unwrap()[0].values.add, 2);
        assert_eq!(summary.children.as_ref().unwrap()[0].values.mul, 1);
        assert_eq!(summary.children.as_ref().unwrap()[0].values.inv, 1);

        // inner assertions
        assert_eq!(
            summary.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[0]
                .values
                .add,
            1
        );
        assert_eq!(
            summary.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[0]
                .values
                .mul,
            1
        );
        assert_eq!(
            summary.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[0]
                .values
                .inv,
            0
        );
    }
}
