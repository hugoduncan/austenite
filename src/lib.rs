#![deny(missing_docs)]
#![feature(collections, core)]

//! Austenite
//!
//! A library for building [Iron](https://github.com/iron/iron)
//! handlers that implements HTTP header handling and content
//! negotiation.
//!
//! ```ignore
//! #[macro_use] extern crate austenite;
//! use austenite::handle;
//! use iron::{Iron, Listening, Request, Response};
//!
//! struct GetOkContent;
//! resource_handler!(GetOkContent);
//! impl Resource for GetOkContent {
//!     fn handle_ok(&self, req: &Request, resp: &mut Response)
//!                -> IronResult<Response>
//!     {
//!       resp.set_mut((status::Ok, "hello"));
//!       Ok(Response::new())
//!     }
//! }
//!
//! fn start_iron() -> Listening {
//!   Iron::new(Resource).listen((address,0u16)).unwrap();
//! }
//! ```

#[macro_use] extern crate hyper;
extern crate iron;
extern crate mime;
extern crate time;
#[macro_use] extern crate log;

pub use hyper_headers::*;
pub use resource::Resource;

/// Content Negotiation
pub mod content_neg;
/// Headers
pub mod hyper_headers;
/// A Resource
pub mod resource;
