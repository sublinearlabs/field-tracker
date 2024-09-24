use ark_ff::PrimeField;
struct Ft<T: PrimeField> {
    inner: T,
}

impl<T: PrimeField> From<T> for Ft<T> {
    fn from(value: T) -> Self {
        Ft { inner: value }
    }
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
