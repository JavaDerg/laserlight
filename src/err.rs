use std::fmt::Display;
use wasm_bindgen::__rt::core::fmt::Formatter;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum InnerError {
    None,
    OsError(winit::error::OsError),
    JsValue(wasm_bindgen::JsValue),
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EngineError {
    inner: InnerError,
    cause: String,
}

impl EngineError {
    #[inline(always)]
    pub fn new(err: InnerError, cause: String) -> Self {
        let inner = err;
        Self { inner, cause }
    }

    pub fn describe(mut self, cause: String) -> Self {
        self.cause = cause;
        self
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The Engine ran into a Issue: {}\n{:#?}",
            self.cause, self.inner
        )
    }
}

pub(crate) trait ErrorConverter<T, E>
where
    E: Into<EngineError>,
{
    fn convert(self) -> Result<T, EngineError>;

    fn describe<F>(self, cause: F) -> Result<T, EngineError>
    where
        F: ToString;
}

impl<T, E> ErrorConverter<T, E> for Result<T, E>
where
    E: Into<EngineError>,
{
    fn convert(self) -> Result<T, EngineError> {
        self.map_err(|err| err.into())
    }

    fn describe<F>(self, cause: F) -> Result<T, EngineError>
    where
        F: ToString,
    {
        self.map_err(|err| err.into().describe(cause.to_string()))
    }
}

impl<T> ErrorConverter<T, ()> for Option<T> {
    fn convert(self) -> Result<T, EngineError> {
        self.ok_or_else(|| EngineError::new(InnerError::None, String::from("No Cause")))
    }

    fn describe<F>(self, cause: F) -> Result<T, EngineError>
    where
        F: ToString,
    {
        self.ok_or_else(|| EngineError::new(InnerError::None, cause.to_string()))
    }
}

impl From<()> for EngineError {
    fn from(_: ()) -> Self {
        EngineError::new(InnerError::None, String::from("No Cause"))
    }
}

impl From<winit::error::OsError> for EngineError {
    fn from(err: winit::error::OsError) -> Self {
        EngineError::new(InnerError::OsError(err), String::from("No Cause"))
    }
}

impl From<wasm_bindgen::JsValue> for EngineError {
    fn from(err: wasm_bindgen::JsValue) -> Self {
        EngineError::new(InnerError::JsValue(err), String::from("No Cause"))
    }
}

impl From<js_sys::Object> for EngineError {
    fn from(err: js_sys::Object) -> Self {
        EngineError::new(InnerError::JsValue(err.into()), String::from("No Cause"))
    }
}
