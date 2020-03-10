pub trait ResultExt<T, E2, C1, C2>
where
    C1: Into<C2>,
{
    fn err_context(self, context: C1) -> Result<T, E2>;
}

impl<T, E1, E2, C1, C2> ResultExt<T, E2, C1, C2> for Result<T, E1>
where
    C1: Into<C2>,
    E2: FromSourceAndContext<E1, C2>,
{
    fn err_context(self, context: C1) -> Result<T, E2> {
        self.map_err(|error| E2::from_source_and_context(error, context.into()))
    }
}

pub trait FromSourceAndContext<E1, C2> {
    fn from_source_and_context(source: E1, context: C2) -> Self;
}
