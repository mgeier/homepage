pub trait ResultExt<T, E2, C> {
    fn err_context(self, context: C) -> Result<T, E2>;
}

impl<T, E1, E2, C> ResultExt<T, E2, C> for Result<T, E1>
where
    E2: FromSourceAndContext<E1, C>,
{
    fn err_context(self, context: C) -> Result<T, E2> {
        self.map_err(|error| E2::from_source_and_context(error, context))
    }
}

pub trait FromSourceAndContext<E1, C> {
    fn from_source_and_context(source: E1, context: C) -> Self;
}
