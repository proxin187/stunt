

///
pub trait Properties {
    ///
    type Builder;

    ///
    fn builder() -> Self::Builder;
}

///
pub trait HasProperty<P, H> {}

///
pub trait HasAllProperties<P, H> {}


