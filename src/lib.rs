use ark_ff::PrimeField;

struct Ft<T: PrimeField> {
    inner: T
}