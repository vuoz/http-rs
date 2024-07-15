#[derive(Clone, Debug, Default, PartialEq)]
pub enum Method {
    #[default]
    GET,
    PUT,
    POST,
    HEAD,
    PATCH,
    TRACE,
    DELETE,
    OPTIONS,
    CONNECT,
}
pub enum MethodError {
    Err(&'static str),
}
impl Method {
    pub fn from_bytes(inpt: &[u8]) -> Result<Method, MethodError> {
        match inpt.len() {
            3 => match inpt {
                b"GET" => Ok(Method::GET),
                b"PUT" => Ok(Method::PUT),
                _ => Err(MethodError::Err("invalid Method")),
            },
            4 => match inpt {
                b"POST" => Ok(Method::POST),
                b"HEAD" => Ok(Method::HEAD),
                _ => Err(MethodError::Err("invalid Method")),
            },
            5 => match inpt {
                b"TRACE" => Ok(Method::TRACE),
                b"PATCH" => Ok(Method::PATCH),
                _ => Err(MethodError::Err("invalid Method")),
            },
            6 => match inpt {
                b"DELETE" => Ok(Method::DELETE),
                _ => Err(MethodError::Err("invalid Method")),
            },
            7 => match inpt {
                b"OPTIONS" => Ok(Method::OPTIONS),
                b"CONNECT" => Ok(Method::CONNECT),
                _ => Err(MethodError::Err("invalid Method")),
            },
            _ => Err(MethodError::Err("invalid length")),
        }
    }
}
