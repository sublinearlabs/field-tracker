#[macro_export]
macro_rules! Ft {
    ($field_type:ty) => {
        $crate::Ft<{({(<$field_type as ark_ff::PrimeField>::MODULUS_BIT_SIZE + 64 - 1) / 64}) as usize}, $field_type>
    };
}

#[macro_export]
macro_rules! start_tscope {
    ($scope_name:expr) => {
        if std::any::type_name_of_val($scope_name).contains("str") {
            $crate::Tracker::start($scope_name)
        } else {
            panic!("start_scope requires scope name (string)")
        }
    };
}

#[macro_export]
macro_rules! end_tscope {
    () => {
        $crate::Tracker::end()
    };
}

#[macro_export]
macro_rules! print_summary {
    () => {
        println!("{}", $crate::Tracker::summary())
    };
}

#[macro_export]
macro_rules! summary {
    () => {
        $crate::Tracker::summary()
    };
}
