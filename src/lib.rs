// Copyright 2015 Hugo Duncan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
