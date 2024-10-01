#[macro_export]
macro_rules! Ft {
    ($field_type:ty) => {
        Ft<{({(<$field_type as ark_ff::PrimeField>::MODULUS_BIT_SIZE + 64 - 1) / 64}) as usize}, $field_type>
    };
}
