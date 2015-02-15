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

use hyper::header::{self, EntityTag, QualityItem};
use hyper::header::parsing::{self};
use std::fmt;
use std::str::FromStr;


/// A charset code
pub type Charset = String;

/// The `Accept-Charset` header
///
/// The `Accept-Charset` header can be used by clients to indicate what
/// response charsets they accept.
#[derive(Clone, PartialEq, Debug)]
pub struct AcceptCharset(pub Vec<QualityItem<Charset>>);

// Can't be used, as the macros specify crate relative paths.
// impl_list_header!(AcceptCharset,
//                   "Accept-Charset",
//                   Vec<QualityItem<Charset>>);

impl ::std::ops::Deref for AcceptCharset {
    type Target = Vec<QualityItem<Charset>>;

    fn deref<'a>(&'a self) -> &'a Vec<QualityItem<Charset>> {
        &self.0
    }
}

impl ::std::ops::DerefMut for AcceptCharset {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec<QualityItem<Charset>> {
        &mut self.0
    }
}

impl header::Header for AcceptCharset {
    fn header_name() -> &'static str {
        "Accept-Charset"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<AcceptCharset> {
        header::parsing::from_comma_delimited(raw).map(AcceptCharset)
    }
}

impl header::HeaderFormat for AcceptCharset {
    fn fmt_header(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        header::parsing::fmt_comma_delimited(fmt, &self[])
    }
}

impl ::std::fmt::Display for AcceptCharset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use hyper::header::HeaderFormat;
        self.fmt_header(f)
    }
}
