use hyper::header::{self, EntityTag, QualityItem};
use hyper::header::parsing::{self};
use std::fmt;
use std::str::FromStr;

/// A languge code
pub type Language = String;

/// The `Accept-Language` header
///
/// The `Accept-Language` header can be used by clients to indicate what
/// response languages they accept.
#[derive(Clone, PartialEq, Debug)]
pub struct AcceptLanguage(pub Vec<QualityItem<Language>>);

// Can't be used, as the macros specify crate relative paths.
// impl_list_header!(AcceptLanguage,
//                   "Accept-Language",
//                   Vec<QualityItem<Language>>);

impl ::std::ops::Deref for AcceptLanguage {
    type Target = Vec<QualityItem<Language>>;

    fn deref<'a>(&'a self) -> &'a Vec<QualityItem<Language>> {
        &self.0
    }
}

impl ::std::ops::DerefMut for AcceptLanguage {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec<QualityItem<Language>> {
        &mut self.0
    }
}

impl header::Header for AcceptLanguage {
    fn header_name() -> &'static str {
        "Accept-Language"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<AcceptLanguage> {
        header::parsing::from_comma_delimited(raw).map(AcceptLanguage)
    }
}

impl header::HeaderFormat for AcceptLanguage {
    fn fmt_header(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        header::parsing::fmt_comma_delimited(fmt, &self[])
    }
}

impl ::std::fmt::Display for AcceptLanguage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use hyper::header::HeaderFormat;
        self.fmt_header(f)
    }
}



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




// /// The `If-Unmodified-Since` header field.
// #[derive(Copy, PartialEq, Clone, Debug)]
// pub struct IfUnmodifiedSince(pub Tm);

// deref!(IfUnmodifiedSince => Tm);

// impl header::Header for IfUnmodifiedSince {
//     fn header_name() -> &'static str {
//         "If-Unmodified-Since"
//     }

//     fn parse_header(raw: &[Vec<u8>]) -> Option<IfUnmodifiedSince> {
//         from_one_raw_str(raw)
//     }
// }


// impl header::HeaderFormat for IfUnmodifiedSince {
//     fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         let tm = self.0;
//         let tm = match tm.tm_utcoff {
//             0 => tm,
//             _ => tm.to_utc(),
//         };
//         fmt::Display::fmt(&tm.rfc822(), fmt)
//     }
// }

// impl FromStr for IfUnmodifiedSince {
//     type Err = ();
//     fn from_str(s: &str) -> Result<IfUnmodifiedSince, ()> {
//         tm_from_str(s).map(IfUnmodifiedSince).ok_or(())
//     }
// }

/// A match for an entity tag
#[derive(PartialEq, Clone, Debug)]
pub enum EntityTagMatch{
    /// A vec of entity tags to match
    EntityTags(Vec<EntityTag>),
    /// Match any entity
    Star
}


/// The `If-Match` header
///
/// The `If-Match` header can be used by clients to indicate what
/// response matchs they if.
#[derive(Clone, PartialEq, Debug)]
pub struct IfMatch(pub EntityTagMatch);

// Can't be used, as the macros specify crate relative paths.
// impl_list_header!(IfMatch,
//                   "If-Match",
//                   Vec<QualityItem<Match>>);

impl ::std::ops::Deref for IfMatch {
    type Target = EntityTagMatch;

    fn deref<'a>(&'a self) -> &'a EntityTagMatch {
        &self.0
    }
}

impl ::std::ops::DerefMut for IfMatch {
    fn deref_mut<'a>(&'a mut self) -> &'a mut EntityTagMatch {
        &mut self.0
    }
}

impl header::Header for IfMatch {
    fn header_name() -> &'static str {
        "If-Match"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<IfMatch> {
        if raw[0][0]==b'*' {
            Some(IfMatch(EntityTagMatch::Star))
        } else {
            match parsing::from_comma_delimited(raw).map(EntityTagMatch::EntityTags) {
                Some(tag) => Some(IfMatch(tag)),
                None => None
            }
        }
    }
}

impl header::HeaderFormat for IfMatch {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            EntityTagMatch::Star => write!(fmt, "{}", "*"),
            EntityTagMatch::EntityTags(ref x) =>
                parsing::fmt_comma_delimited(fmt, &x[])
        }
    }
}

impl FromStr for IfMatch {
    type Err = ();
    fn from_str(s: &str) -> Result<IfMatch, ()> {
        if s.trim()=="*" {
            Ok(IfMatch(EntityTagMatch::Star))
        } else {
            let mut v : Vec<u8> = Vec::new();
            v.push_all(s.as_bytes());
            match parsing::from_comma_delimited(&[v]).map(EntityTagMatch::EntityTags) {
                Some(tag) => Ok(IfMatch(tag)),
                None => Err(())
            }
        }
    }
}


/// The `If-None-Match` header field.
#[derive(PartialEq, Clone, Debug)]
pub struct IfNoneMatch(pub EntityTagMatch);

deref!(IfNoneMatch => EntityTagMatch);

impl header::Header for IfNoneMatch {
    fn header_name() -> &'static str {
        "If-None-Match"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<IfNoneMatch> {
        if raw[0][0]==b'*' {
            Some(IfNoneMatch(EntityTagMatch::Star))
        } else {
            match parsing::from_comma_delimited(raw).map(EntityTagMatch::EntityTags) {
                Some(tag) => Some(IfNoneMatch(tag)),
                None => None
            }
        }
    }
}


impl header::HeaderFormat for IfNoneMatch {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            EntityTagMatch::Star => write!(fmt, "{}", "*"),
            EntityTagMatch::EntityTags(ref x) =>
                parsing::fmt_comma_delimited(fmt, &x[])
        }
    }
}

impl FromStr for IfNoneMatch {
    type Err = ();
    fn from_str(s: &str) -> Result<IfNoneMatch, ()> {
        if s.trim()=="*" {
            Ok(IfNoneMatch(EntityTagMatch::Star))
        } else {
            let mut v : Vec<u8> = Vec::new();
            v.push_all(s.as_bytes());
            match parsing::from_comma_delimited(&[v]).map(EntityTagMatch::EntityTags) {
                Some(tag) => Ok(IfNoneMatch(tag)),
                None => Err(())
            }
        }
    }
}



// #[test]
// fn test_parse_header() {
//     let a: IfMatch = header::Header::parse_header([b"de;q=1.0, en; q=0.5".to_vec()].as_slice()).unwrap();
//     let b = IfMatch(vec![
//         QualityItem{item: "de".to_string(), quality: 1f32},
//         QualityItem{item: "en".to_string(), quality: 0.5f32},
//     ]);
//     assert_eq!(a, b);
// }
