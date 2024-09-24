use ark_ff::{FftField, Field, PrimeField};
use ark_serialize::Flags;
use num_bigint::BigUint;
use std::iter::Iterator;
use std::{str::FromStr, vec::IntoIter};

#[derive(Debug, Clone, Eq, PartialEq, Copy, Default, Ord)]
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
