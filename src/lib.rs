use ark_ff::{Field, PrimeField};
use ark_serialize::Flags;

struct Ft<T: PrimeField> {
    inner: T,
}

impl<T: PrimeField> Field for Ft<T> {
    type BasePrimeField = T::BasePrimeField;

    type BasePrimeFieldIter = T::BasePrimeFieldIter;

    const SQRT_PRECOMP: Option<ark_ff::SqrtPrecomputation<Self>> =
        T::SQRT_PRECOMP.map(|v| v.into());

    const ZERO: Self = from_primefield(T::ZERO);

    const ONE: Self = from_primefield(T::ONE);

    fn extension_degree() -> u64 {
        T::extension_degree()
    }

    fn to_base_prime_field_elements(&self) -> Self::BasePrimeFieldIter {
        self.inner.to_base_prime_field_elements()
    }

    fn from_base_prime_field_elems(elems: &[Self::BasePrimeField]) -> Option<Self> {
        T::from_base_prime_field_elems(elems).map(|v| v.into())
    }

    fn from_base_prime_field(elem: Self::BasePrimeField) -> Self {
        T::from_base_prime_field(elem).into()
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
}

impl<T: PrimeField> From<T> for Ft<T> {
    fn from(value: T) -> Self {
        Ft { inner: value }
    }
}

const fn from_primefield<T: PrimeField>(value: T) -> Ft<T> {
    Ft { inner: value }
}
