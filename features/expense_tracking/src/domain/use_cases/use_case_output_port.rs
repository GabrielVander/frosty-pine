pub trait UseCaseOutputPort: Send + Sync {
    type Input;
    type Output;

    fn apply(&self, _: Self::Input) -> Self::Output;
}
