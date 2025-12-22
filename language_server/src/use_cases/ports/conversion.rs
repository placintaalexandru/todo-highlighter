pub trait Conversion {
    type From;
    type To;

    fn convert(from: Self::From) -> Self::To;
}
