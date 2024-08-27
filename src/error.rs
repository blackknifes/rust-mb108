#[derive(Debug, Clone)]
pub struct StdError(String);
impl std::fmt::Display for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for StdError {}

impl StdError {
    pub fn new(msg: String) -> Self {
        Self(msg)
    }
}

#[derive(Debug)]
pub enum Error {
    StdError(String),
    TypeMismatch(String),
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TypeMismatch(str) | Error::StdError(str) => str.fmt(f),
            Error::Other(err) => std::fmt::Display::fmt(err, f),
        }
    }
}
pub type Result<T> = core::result::Result<T, Error>;

impl<ERR> From<ERR> for Error
where
    ERR: std::error::Error + 'static,
{
    fn from(value: ERR) -> Self {
        Error::other(value)
    }
}

impl From<Error> for Box<dyn std::error::Error> {
    fn from(value: Error) -> Self {
        match value {
            Error::TypeMismatch(msg) | Error::StdError(msg) => Box::new(StdError::new(msg)),
            Error::Other(err) => err,
        }
    }
}

impl Error {
    pub fn msg(str: impl Into<String>) -> Self {
        return Error::StdError(str.into());
    }

    pub fn other<ERR>(err: ERR) -> Self
    where
        ERR: std::error::Error + 'static,
    {
        return Self::Other(Box::new(err));
    }

    pub(crate) fn type_mismatch(msg: impl Into<String>) -> Self {
        Error::TypeMismatch(msg.into())
    }

    pub fn downcast<ERR>(self) -> Result<ERR>
    where
        ERR: std::error::Error + 'static,
    {
        match self {
            Error::Other(err) => err
                .downcast::<ERR>()
                .map(|err| *err)
                .map_err(|err| Error::Other(err)),
            _ => Err(Self::type_mismatch(format!("error type is not other"))),
        }
    }

    pub fn downcast_ref<ERR>(&self) -> Result<&ERR>
    where
        ERR: std::error::Error + 'static,
    {
        match self {
            Error::Other(err) => err
                .downcast_ref::<ERR>()
                .ok_or_else(|| Self::type_mismatch("error type mismatch".to_owned())),
            _ => Err(Self::type_mismatch(format!("error type is not other"))),
        }
    }

    pub fn downcast_mut<ERR>(&mut self) -> Result<&mut ERR>
    where
        ERR: std::error::Error + 'static,
    {
        match self {
            Error::Other(err) => err
                .downcast_mut::<ERR>()
                .ok_or_else(|| Self::type_mismatch("error type mismatch".to_owned())),
            _ => Err(Self::type_mismatch(format!("error type is not other"))),
        }
    }
}
