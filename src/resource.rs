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

/// A module for http resources
use std::{error,fmt};
use std::boxed::Box;

use hyper::header::EntityTag;
use iron::{IronError, IronResult, Request, Response, status};
use iron::headers::{self, Encoding, QualityItem};
use iron::method;
use iron::modifier::Set;
use mime::{Mime, TopLevel, SubLevel};
use time::Tm;
use content_neg;
use hyper_headers;

use self::ResourceError::*;

/// Austenite's Error Type
#[derive(Debug,PartialEq,Clone)]
pub enum ResourceError{
    /// Function not implemented
    NotImplemented,
    /// Application specific error
    ApplicationError(String)
}

impl fmt::Display for ResourceError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl error::Error for ResourceError {
    fn description(&self) -> &str {
        "ResourceError"
    }
}

impl error::FromError<ResourceError> for IronError {
    fn from_error(err: ResourceError) -> IronError {
        IronError::new(err,(status::NotImplemented, "Not implemented"))
    }
}

fn internal_error(s: &str) -> (status::Status, Mime, String) {
    (status::InternalServerError,
     Mime(TopLevel::Text, SubLevel::Plain, vec![]),
     s.to_string())
}

fn not_implemented(s: &str) -> IronError {
    IronError::new(ResourceError::NotImplemented, internal_error(s))
}

/// Result type for resource trait functions.
pub type ResourceResult = Result<(),ResourceError>;

fn strong_match(_: &EntityTag, _: &EntityTag) -> bool {
    assert!(false,"not implemented");
    true
}

fn weak_match(_: &EntityTag, _: &EntityTag) -> bool {
    assert!(false,"not implemented");
    true
}

/// Cannot use this due to https://github.com/rust-lang/rust/issues/11403
macro_rules! decision_fn {
    ($decision:ident, $thenv:ident, $elsev:ident) => (
        fn concat_idents!($decision,_decision)
            (&self, req: &mut Request, mut resp: Response) -> IronResult<Response> {
                let decision=self.$decision(req);
                debug!(concat!(stringify!($decision),": {}"), decision);
                if decsion {
                    self.$thenv(req,resp)
                } else {
                    self.$elsev(req,resp)
                }
            }
        )
}

macro_rules! decision_body {
    ($me:ident, $req:ident, $resp:ident,
     $decision:ident, $thenv:ident, $elsev:ident) => (
        {
            let decision=$me.$decision($req, &mut $resp);
            debug!(concat!(stringify!($decision),": {}"), decision);
            if decision {
                $me.$thenv($req,$resp)
            } else {
                $me.$elsev($req,$resp)
            }
        }
        );
    // same, but with an expression rather than an identifier for the decision
    ($me:ident, $req:ident, $resp:ident,
     $decision:expr, $thenv:ident, $elsev:ident) => (
        {
            let decision=$decision;
            debug!(concat!(stringify!($decision),": {}"), decision);
            if decision {
                $me.$thenv($req,$resp)
            } else {
                $me.$elsev($req,$resp)
            }
        }
        )
}

/// Main trait for an HTTP resource.
///
/// Implement this trait's optional functions to control how the HTTP
/// request is handled.

pub trait Resource : Sync + Send {
    // Trait functions that can be overridden

    /// Override to control service availability.  If this returns
    /// false, then a 503 ServiceUnavailable reply will result.
    /// Defaults to true.
    fn service_available(&self, _: &mut Request, _: &mut Response) -> bool {
        return true;
    }

    /// Override to control known HTTP verbs.  If this returns false,
    /// then a 501 NotImplemented reply will result.  Defaults to
    /// true.
    fn known_method(&self, req: &mut Request, _: &mut Response) -> bool {
        match req.method {
            method::Extension(_) => false,
            _ => true
        }
    }

    /// Override to limit uri length.  If this returns true, then a
    /// 414 RequestUriTooLong reply will result.  Defaults to false.
    fn uri_too_long(&self, _: &mut Request, _: &mut Response) -> bool {
        return false;
    }

    /// Override to control valid HTTP verbs for the request.  If this
    /// returns false, then a 405 MethodNotAllowed reply will result.
    /// Defaults to alowing GET and HEAD.
    fn method_allowed(&self, req: &mut Request, _: &mut Response) -> bool {
        match req.method {
            method::Get|method::Head => true,
            _ => false
        }
    }

    /// Override to control request validity.  If this returns false,
    /// then a 405 MethodNotAllowed reply will result.  Defaults to
    /// alowing GET and HEAD.
    fn malformed(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Override to control request authentication.  If this returns
    /// false, then a 401 Unauthorized reply will result.  Defaults to
    /// true.
    fn authorized(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Override to control request authorisation.  If this returns
    /// false, then a 403 Forbidden reply will result.  Defaults to
    /// true.
    fn allowed(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Override to control content header validity.  If this returns
    /// false, then a 501 NotImplemented reply will result.  Defaults
    /// to true.
    fn valid_content_header(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Override to control content type validity.  If this returns
    /// false, then a 415 UnsupportedMediaType reply will result.
    /// Defaults to true.
    fn known_content_type(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Override to control content length validity.  If this returns
    /// false, then a 413 PayloadTooLarge reply will result.
    /// Defaults to true.
    fn valid_entity_length(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Override to control whether an entity exists.  If this returns
    /// false, then the entity is deemed not too exist.  This effects
    /// the handling of POST, PUT, and DELETE verbs.  Defaults to
    /// true.
    fn exists(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Override to control whether an entity once existed.  If this
    /// returns false, then the entity is deemed not to have existed.
    /// This effects the handling of GET, POST, and PUT verbs.
    /// Defaults to true.
    fn existed(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Control whether an entity is returned.  If false, returns 205 NoContent.
    /// Defaults to false.
    fn respond_with_entity(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Indicates whether an entity was successfully created for the request.
    /// Defaults to true.
    fn new(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Controls whether to redirect after a POST request.  Defaults
    /// to false.
    fn post_redirect(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Indicates the PUT request should be made to a different URL.
    /// Defaults to false.
    fn put_to_different_url(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Controls whether to return a 300 MultipleChoices response.
    /// Defaults to false.
    fn multiple_representations(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Indicates the PUT request has resulted in a conflict.  If this
    /// returns true, it will cause a 409 Conflict response.  Defaults
    /// to false.
    fn conflict(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Controls whether a POST to a non existing entity is handled.
    /// When false, causes a 410 Gone response.  Defaults to true.
    fn can_post_to_missing(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Controls whether a POST to a deleted entity is handled.
    /// When false, causes a 410 Gone response.  Defaults to false.
    fn can_post_to_gone(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Controls whether a PUT to a non-existing entity is handled.
    /// When false, causes a 501 NotImplemented response.  Defaults to true.
    fn can_put_to_missing(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Indicates whether a resource was moved permanently.  Defaults
    /// to false.
    fn moved_permanently(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Indicates whether a resource was moved temporarily.  Defaults
    /// to false.
    fn moved_temporarily(&self, _: &mut Request, _: &mut Response) -> bool {
        false
    }

    /// Indicates whether a delete request was processed.  Defaults to
    /// true.
    fn delete_enacted(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Indicates whether an entity is processable.  Returns 422
    /// UnprocessableEntity when false.  Defaults to true.
    fn processable(&self, _: &mut Request, _: &mut Response) -> bool {
        true
    }

    /// Return an optional ETag for the entity
    fn etag(&self, _: &Request, _: &mut Response) -> Option<headers::Etag> {
        None
    }

    /// Return an optional last modified time for the entity
    fn last_modified(&self, _: &Request, _: &Response) -> Option<Tm> {
        None
    }

    // Actions

    #[allow(missing_docs)]
    fn get(&self, _: &mut Request, _: Response) -> IronResult<Response> {
        Err(not_implemented("GET not implemented"))
    }

    /// Execute a DELETE request.  Will assert! by default.
    fn delete(&self, _: &mut Request, _: Response) -> IronResult<Response> {
        Err(not_implemented("DELETE not implemented"))
    }

    /// Execute a PATCH request.  Will assert! by default.
    fn patch(&self, _: &mut Request, _: Response) -> IronResult<Response> {
        Err(not_implemented("PATCH not implemented"))
    }

    /// Execute a POST request.  Will assert! by default.
    fn post(&self, _: &mut Request, _: Response) -> IronResult<Response> {
        Err(not_implemented("POST not implemented"))
    }

    /// Execute a PUT request.  Will assert! by default.
    fn put(&self, _: &mut Request, _: Response) -> IronResult<Response> {
        Err(not_implemented("PUT not implemented"))
    }



    // some data returning methods - not sure these are really wanted

    /// Return a vector of available languages.
    fn available_languages(&self, _: &Request,
                           _: &mut Response) -> Vec<String> {
        vec!["*".to_string()]
    }

    /// Return a vector of available charsets.
    fn available_charsets(&self, _: &Request,
                          _: &mut Response) -> Vec<String> {
        vec!["UTF-8".to_string()]
    }

    /// Return a vector of available encodings.
    fn available_encodings(&self, _: &Request,
                           _: &mut Response) -> Vec<Encoding> {
        vec![Encoding::Identity]
    }

    /// Return a vector of available content types.
    fn available_content_types(&self, _: &Request,
                               _: &mut Response) -> Vec<Mime> {
        vec![]
    }


    /// base logic

    #[allow(missing_docs)]
    fn accept_exists(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::Accept>() {
            Some(_) => true,
            None =>
                match content_neg::best_content_type(
                    &vec![QualityItem::<Mime>{
                        item: Mime(TopLevel::Star, SubLevel::Star, vec![]),
                        quality: 1.0f32
                    }],
                    &self.available_content_types(req,resp)) {
                    Some(ct) => {resp.set_mut(ct); true}
                    None => false
                }
        }
    }

    #[allow(missing_docs)]
    fn media_type_available(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::Accept>() {
            Some(cts) if !cts.is_empty() => {
                let available = &self.available_content_types(req,resp);
                match content_neg::best_content_type(&cts, available) {
                    Some(ct) => {resp.set_mut(ct); true}
                    None => false
                }},
            _ => {
                let available = self.available_content_types(req,resp);
                if available.len()>0 {
                    resp.set_mut(available[0].clone());
                }
                true
            }
        }
    }



    #[allow(missing_docs)]
    fn language_available(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::AcceptLanguage>() {
            Some(_) => match content_neg::best_language(
                vec![QualityItem::<String>{
                    item: "*".to_string(),
                    quality: 1.0f32}],
                &self.available_languages(req,resp)) {
                Some(l) => {resp.set_mut(l); true}
                None => false
            },
            None =>  true
        }
    }

    #[allow(missing_docs)]
    fn charset_available(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::AcceptCharset>() {
            Some(&hyper_headers::AcceptCharset(ref x)) => match content_neg::best_charset(
                x, &self.available_charsets(req,resp)) {
                Some(l) => {resp.set_mut(l); true}
                None => false
            },
            None =>  true
        }
    }

    #[allow(missing_docs)]
    fn encoding_available(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::AcceptEncoding>() {
            Some(&headers::AcceptEncoding(ref x)) => match content_neg::best_encoding(
                x, &self.available_encodings(req,resp)) {
                Some(_) => {  // FIXME resp.set_mut(l);
                    true}
                None => false
            },
            None =>  true
        }
    }

    #[allow(missing_docs)]
    fn if_match(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::Etag>() {
            Some(&headers::Etag(ref x)) => {
                match self.etag(req, resp) {
                    Some(ref tag) => strong_match(x,tag),
                    None => false
                }
            },
            _ => false
        }
    }

    #[allow(missing_docs)]
    fn if_none_match(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::Etag>() {
            Some(&headers::Etag(ref x)) => {
                match self.etag(req, resp) {
                    Some(ref tag) => weak_match(x,tag),
                    None => false
                }
            },
            _ => false
        }
    }

    #[allow(missing_docs)]
    fn if_match_star(&self, req: &mut Request, _: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::IfMatch>() {
            Some(&hyper_headers::IfMatch(hyper_headers::EntityTagMatch::Star)) =>
                true,
            _ => false
        }
    }

    #[allow(missing_docs)]
    fn if_none_match_star(&self, req: &mut Request, _: &mut Response) -> bool {
        match req.headers.get::<headers::IfNoneMatch>() {
            Some(&headers::IfNoneMatch::Any) => true,
            _ => false
        }
    }

    #[allow(missing_docs)]
    fn unmodified_since(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::IfUnmodifiedSince>() {
            Some(ref x) => {
                match self.last_modified(req, resp) {
                    Some(ref y) => y<=x,
                    None => false
                }
            }
            None => false
        }
    }

    #[allow(missing_docs)]
    fn modified_since(&self, req: &mut Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::IfUnmodifiedSince>() {
            Some(ref x) => {
                match self.last_modified(req, resp) {
                    Some(ref y) => y>x,
                    None => false
                }
            }
            None => false
        }
    }

    #[allow(missing_docs)]
    fn if_match_star_exists_for_missing(&self, req: &mut Request,
                                        _: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::IfMatch>() {
            Some(&hyper_headers::IfMatch(hyper_headers::EntityTagMatch::Star)) => true,
            _ => false
        }
    }

    /// logic functions
    #[allow(missing_docs)]
    fn service_available_decision(&self,
                                  req: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, service_available,
                       known_method_decision, handle_service_unavailable)
    }

    #[allow(missing_docs)]
    fn known_method_decision(&self, req: &mut Request,
                             mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, known_method,
                       uri_too_long_decision, handle_unknown_method)
    }

    #[allow(missing_docs)]
    fn uri_too_long_decision(&self, req: &mut Request,
                             mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, uri_too_long,
                       handle_uri_too_long, method_allowed_decision)
    }

    #[allow(missing_docs)]
    fn method_allowed_decision(&self, req: &mut Request,
                               mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, method_allowed,
                       malformed_decision, handle_method_not_allowed)
    }

    #[allow(missing_docs)]
    fn malformed_decision(&self, req: &mut Request,
                          mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, malformed,
                       handle_malformed, authorized_decision)
    }

    #[allow(missing_docs)]
    fn authorized_decision(&self, req: &mut Request,
                           mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, authorized,
                       allowed_decision, handle_unauthorized)
    }

    #[allow(missing_docs)]
    fn allowed_decision(&self, req: &mut Request,
                        mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, allowed,
                       valid_content_header_decision, handle_forbidden)
    }

    #[allow(missing_docs)]
    fn valid_content_header_decision(&self,
                                     req: &mut Request,
                                     mut resp: Response)
                                     -> IronResult<Response> {
        decision_body!(self, req, resp, valid_content_header,
                       known_content_type_decision, handle_not_implemented)
    }

    #[allow(missing_docs)]
    fn known_content_type_decision(&self,
                                   req: &mut Request,
                                   mut resp: Response)
                                   -> IronResult<Response> {
        decision_body!(self, req, resp, known_content_type,
                       valid_entity_length_decision,
                       handle_unsupported_media_type)
    }

    #[allow(missing_docs)]
    fn valid_entity_length_decision(&self,
                                    req: &mut Request,
                                    mut resp: Response)
                                    -> IronResult<Response> {
        decision_body!(self, req, resp, valid_entity_length,
                       is_options_decision,
                       handle_payload_too_large)
    }

    #[allow(missing_docs)]
    fn is_options_decision(&self, req: &mut Request,
                           resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Options,
                       handle_options, accept_exists_decision)
    }

    #[allow(missing_docs)]
    fn accept_exists_decision(&self, req: &mut Request,
                              mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, accept_exists,
                       media_type_available_decision,
                       accept_language_exists_decision)
    }

    #[allow(missing_docs)]
    fn media_type_available_decision(&self,
                                     req: &mut Request,
                                     mut resp: Response)
                                     -> IronResult<Response> {
        decision_body!(self, req, resp, media_type_available,
                       accept_language_exists_decision,
                       handle_not_acceptable)
    }

    #[allow(missing_docs)]
    fn accept_language_exists_decision(&self,
                                       req: &mut Request,
                                       resp: Response)
                                       -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::AcceptLanguage>(req),
                       language_available_decision,
                       accept_charset_exists_decision)
                                       }

    #[allow(missing_docs)]
    fn language_available_decision(&self,
                                   req: &mut Request,
                                   mut resp: Response)
                                   -> IronResult<Response> {
        decision_body!(self, req, resp, language_available,
                       accept_charset_exists_decision,
                       handle_not_acceptable)
    }

    #[allow(missing_docs)]
    fn accept_charset_exists_decision(&self,
                                      req: &mut Request,
                                      resp: Response)
                                      -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::AcceptCharset>(req),
                       charset_available_decision,
                       accept_encoding_exists_decision)
    }

    #[allow(missing_docs)]
    fn charset_available_decision(&self,
                                  req: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, charset_available,
                       accept_encoding_exists_decision,
                       handle_not_acceptable)
    }

    #[allow(missing_docs)]
    fn accept_encoding_exists_decision(&self,
                                       req: &mut Request,
                                       resp: Response)
                                       -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::AcceptEncoding>(req),
                       encoding_available_decision,
                       processable_decision)
    }

    #[allow(missing_docs)]
    fn encoding_available_decision(&self,
                                   req: &mut Request,
                                   mut resp: Response)
                                   -> IronResult<Response> {
        decision_body!(self, req, resp, encoding_available,
                       processable_decision,
                       handle_not_acceptable)
    }

    #[allow(missing_docs)]
    fn processable_decision(&self,
                            req: &mut Request,
                            mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, processable,
                       exists_decision,
                       handle_unprocessable_entity)
    }

    #[allow(missing_docs)]
    fn exists_decision(&self,
                       req: &mut Request,
                       mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, exists,
                       if_match_exists_decision,
                       if_match_star_exists_for_missing_decision)
    }

    #[allow(missing_docs)]
    fn if_match_exists_decision(&self,
                                req: &mut Request,
                                resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::IfMatch>(req),
                       if_match_star_decision,
                       if_unmodified_since_exists_decision)
    }

    #[allow(missing_docs)]
    fn if_match_star_decision(&self,
                              req: &mut Request,
                              mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_match_star,
                       if_unmodified_since_exists_decision,
                       if_match_decision)
    }

    #[allow(missing_docs)]
    fn if_match_decision(&self,
                                          req: &mut Request,
                                          mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_match,
                       if_unmodified_since_exists_decision,
                       handle_precondition_failed)
    }

    #[allow(missing_docs)]
    fn if_unmodified_since_exists_decision(&self,
                                           req: &mut Request,
                                           resp: Response)
                                           -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::IfUnmodifiedSince>(req),
                       if_unmodified_since_decision,
                       if_none_match_exists_decision)
    }

    #[allow(missing_docs)]
    fn if_unmodified_since_decision(&self,
                                 req: &mut Request,
                                 mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, unmodified_since,
                       handle_precondition_failed,
                       if_none_match_exists_decision)
    }

    #[allow(missing_docs)]
    fn if_none_match_exists_decision(&self,
                                     req: &mut Request,
                                     resp: Response)
                                     -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::IfNoneMatch>(req),
                       if_none_match_star_decision,
                       if_modified_since_exists_decision)
    }

    #[allow(missing_docs)]
    fn if_none_match_star_decision(&self, req: &mut Request,
                                   mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_none_match_star,
                       if_none_match_decision,
                       none_match_status_decision)
    }

    #[allow(missing_docs)]
    fn if_none_match_decision(&self, req: &mut Request,
                              mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_none_match,
                       none_match_status_decision,
                       if_modified_since_exists_decision)
    }

    #[allow(missing_docs)]
    fn none_match_status_decision(&self, req: &mut Request,
                                  resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp,
                       req.method == method::Get || req.method == method::Head,
                       handle_not_modified,
                       handle_precondition_failed)
    }

    #[allow(missing_docs)]
    fn if_modified_since_exists_decision(&self, req: &mut Request,
                                         resp: Response)
                                         -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::IfModifiedSince>(req),
                       if_modified_since_decision,
                       method_delete_decision)
    }

    #[allow(missing_docs)]
    fn if_modified_since_decision(&self, req: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, modified_since,
                       method_delete_decision,
                       handle_not_modified)
    }

    #[allow(missing_docs)]
    fn method_delete_decision(&self, req: &mut Request,
                              resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Delete,
                       delete,
                       method_patch_decision)
    }

    #[allow(missing_docs)]
    fn method_patch_decision(&self, req: &mut Request,
                             resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Patch,
                       patch,
                       post_to_existing_decision)
    }

    #[allow(missing_docs)]
    fn post_to_existing_decision(&self, req: &mut Request,
                                 resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Post,
                       post,
                       put_to_existing_decision)
    }

    #[allow(missing_docs)]
    fn put_to_existing_decision(&self, req: &mut Request,
                                resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Put,
                       conflict_decision,
                       multiple_representations_decision)
    }

    #[allow(missing_docs)]
    fn if_match_star_exists_for_missing_decision(&self,
                                                 req: &mut Request,
                                                 mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_match_star_exists_for_missing,
                       handle_precondition_failed,
                       method_put_decision)
    }

    #[allow(missing_docs)]
    fn method_put_decision(&self, req: &mut Request,
                           resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Put,
                       put_to_different_url_decision,
                       existed_decision)
    }

    #[allow(missing_docs)]
    fn put_to_different_url_decision(&self,
                                     req: &mut Request,
                                     mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, put_to_different_url,
                       handle_moved_permanently,
                       can_put_to_missing_decision)
    }

    #[allow(missing_docs)]
    fn can_put_to_missing_decision(&self,
                                 req: &mut Request,
                                 mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, can_put_to_missing,
                       conflict_decision,
                       handle_not_implemented)
    }

    #[allow(missing_docs)]
    fn conflict_decision(&self,
                         req: &mut Request,
                         mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, conflict,
                       handle_conflict,
                       put)
    }

    #[allow(missing_docs)]
    fn existed_decision(&self,
                                 req: &mut Request,
                                 mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, existed,
                       moved_permanently_decision,
                       post_to_missing_decision)
    }

    #[allow(missing_docs)]
    fn moved_permanently_decision(&self,
                                  req: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, moved_permanently,
                       handle_moved_permanently,
                       moved_temporarily_decision)
    }

    #[allow(missing_docs)]
    fn moved_temporarily_decision(&self,
                                  req: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, moved_temporarily,
                       handle_moved_temporarily,
                       post_to_gone_decision)
    }

    #[allow(missing_docs)]
    fn post_to_gone_decision(&self, req: &mut Request,
                             resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Post,
                       can_post_to_gone_decision,
                       handle_gone)
    }

    #[allow(missing_docs)]
    fn can_post_to_gone_decision(&self, req: &mut Request,
                                 mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, can_post_to_gone,
                       post,
                       handle_gone)
    }

    #[allow(missing_docs)]
    fn post_to_missing_decision(&self, req: &mut Request,
                                resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Post,
                       can_post_to_missing_decision,
                       handle_not_found)
    }

    #[allow(missing_docs)]
    fn can_post_to_missing_decision(&self,
                                    req: &mut Request,
                                    mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, can_post_to_missing,
                       post,
                       handle_not_found)
    }

    #[allow(missing_docs)]
    fn post_redirect_decision(&self,
                              req: &mut Request,
                              mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, post_redirect,
                       handle_see_other,
                       new_decision)
    }

    #[allow(missing_docs)]
    fn new_decision(&self,
                    req: &mut Request,
                    mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, new,
                       handle_created,
                       respond_with_entity_decision)
    }

    #[allow(missing_docs)]
    fn respond_with_entity_decision(&self,
                                    req: &mut Request,
                                    mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, respond_with_entity,
                       multiple_representations_decision,
                       handle_no_content)
    }

    #[allow(missing_docs)]
    fn multiple_representations_decision(&self,
                                    req: &mut Request,
                                    mut resp: Response) -> IronResult<Response> {
        decision_body!(self, req, resp, multiple_representations,
                       handle_multiple_representations,
                       get)
    }

    // default handlers

    #[allow(missing_docs)]
    fn handle_service_unavailable(&self, _: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::ServiceUnavailable, "Service unavailable"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_unknown_method(&self, _: &mut Request,
                             mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NotImplemented, "Unknown method"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_uri_too_long(&self, _: &mut Request,
                           mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NotImplemented, "Request URI too long"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_method_not_allowed(&self, _: &mut Request,
                                 mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::MethodNotAllowed, "Method not allowed"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_malformed(&self, _: &mut Request,
                        mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::BadRequest, "Bad request"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_unauthorized(&self, _: &mut Request,
                           mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::Unauthorized, "Unauthorized"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_forbidden(&self, _: &mut Request,
                        mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::Forbidden, "Forbidden"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_not_implemented(&self, _: &mut Request,
                              mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NotImplemented, "Not implemented"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_unsupported_media_type(&self, _: &mut Request,
                                     mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::UnsupportedMediaType, "Unsupported media type"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_payload_too_large(&self, _: &mut Request,
                                mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::PayloadTooLarge,
                      "Payload too large"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_not_acceptable(&self, _: &mut Request,
                             mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NotAcceptable, "Not Acceptable"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_unprocessable_entity(&self, _: &mut Request,
                                   mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::UnprocessableEntity, "Unprocessable entity"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_conflict(&self, _: &mut Request,
                       mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::Conflict, "Conflict"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_see_other(&self, _: &mut Request,
                        mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::SeeOther, "Conflict"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_created(&self, _: &mut Request,
                      mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::Created, "Created"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_precondition_failed(&self, _: &mut Request,
                                  mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::PreconditionFailed, "Precondition failed"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_not_modified(&self, _: &mut Request,
                           mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NotModified, "Not modified"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_moved_permanently(&self, _: &mut Request,
                                mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::PermanentRedirect, "Permanent redirect"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_moved_temporarily(&self, _: &mut Request,
                                mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::TemporaryRedirect, "Temporary redirect"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_gone(&self, _: &mut Request, mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::Gone, "Gone"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_not_found(&self, _: &mut Request,
                        mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NotFound, "Not found"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_no_content(&self, _: &mut Request,
                         mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::NoContent, "No content"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_multiple_representations(&self, _: &mut Request,
                                       mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::MultipleChoices, "Multiple Choices"));
        Ok(resp)
    }

    #[allow(missing_docs)]
    fn handle_options(&self, _: &mut Request,
                      mut resp: Response) -> IronResult<Response> {
        resp.set_mut((status::Ok, ""));
        Ok(resp)
    }

    /// Iron handler function
    fn resource_handle(&self, req: &mut Request) -> IronResult<Response> {
        self.service_available_decision(req, Response::new())
    }
}

fn header_exists<T: headers::Header+headers::HeaderFormat>(req: &mut Request) -> bool {
    req.headers.get::<T>().is_some()
}

/// Implement an Iron Handler on a resource
#[macro_export]
pub macro_rules! resource_handler {
    ($s:ident) => {
        impl ::iron::Handler for $s {
            fn handle(&self, req: &mut Request) -> IronResult<Response> {
                self.resource_handle(req)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{self, IpAddr};
    use hyper::server::Listening;
    use iron::{Handler, Iron, IronResult, Request, Response, status};
    use iron::error::HttpResult;
    use iron::modifier::Set;


    fn http_server<T>(resource: T) -> HttpResult<Listening> where T: Resource+Handler+Sync+Send {
        let address = IpAddr::Ipv4Addr(127,0,0,1);
        Iron::new(resource).listen((address,0u16))
    }

    // struct GetOk;
    // impl Resource for GetOk {}
    // resource_handler!(GetOk);
    // // resource!(GetOk);

    // #[test]
    // fn test_get_ok() {
    //   let mut listen = http_server(GetOk).unwrap();
    //   let mut client = hyper::Client::new();
    //    match client.get(&format!("http://127.0.0.1:{}", listen.socket.port)[])
    //       .send() {
    //           Ok(ref mut r) => {
    //               assert_eq!("", r.read_to_string().unwrap());
    //               assert_eq!(status::Ok, r.status);
    //           },
    //           Err(x) => assert!(false, "get failed")
    //       };
    //    listen.close().unwrap();
    // }


    struct GetOkContent;
    resource_handler!(GetOkContent);

    impl Resource for GetOkContent {
        fn get(&self, _: &mut Request, mut resp: Response) -> IronResult<Response> {
            resp.set_mut((status::Ok, "hello"));
            Ok(resp)
        }
    }

    #[test]
    fn test_get_ok_content() {
      let mut listen = http_server(GetOkContent).unwrap();
      let mut client = hyper::Client::new();
       match client.get(&format!("http://127.0.0.1:{}", listen.socket.port)[])
          .send() {
              Ok(ref mut r) => {
                  assert_eq!("hello", r.read_to_string().unwrap());
                  assert_eq!(status::Ok, r.status);
              },
              Err(_) => assert!(false, "get failed")
          };
       listen.close().unwrap();
    }
}
