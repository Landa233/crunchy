use npyz::{AutoSerialize, DType};

pub trait DTypeable {
    fn generate_dtype(&self) -> DType;
}

impl<T: AutoSerialize> DTypeable for T {
    fn generate_dtype(&self) -> DType {
        <T as AutoSerialize>::default_dtype()
    }
}
