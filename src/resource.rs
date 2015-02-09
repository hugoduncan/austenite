/// A module for http resources
use hyper::header::EntityTag;
use hyper;
use iron::{IronError, IronResult, Request, Response, status};
use iron::headers::{self,Encoding, QualityItem};
use iron::method;
use iron::modifier::Set;
use mime::{Mime,TopLevel,SubLevel};
use std::{error,fmt};
use std::boxed::Box;
use time::Tm;
use content_neg;
use hyper_headers;

#[derive(Debug)]
pub enum ResourceError{
    RequestUriTooLong,
    ServiceUnavailable,
    UnknownMethod,
    MethodNotAllowed,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotImplemented,
    UnsupportedMediaType,
    RequestEntityTooLarge,
    NotAcceptable,
    UnprocessableEntity,
    PreconditionFailed,
    NotModified,
    PermanentRedirect,
    TemporaryRedirect,
    Gone,
    NotFound,
    NoContent,
    MultipleChoices,
    Conflict,
    Created,
    SeeOther,
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


fn strong_match(x: &EntityTag, y: &EntityTag) -> bool {
    assert!(false,"not implemented");
    true
}

fn weak_match(x: &EntityTag, y: &EntityTag) -> bool {
    assert!(false,"not implemented");
    true
}

/// Cannot use this due to https://github.com/rust-lang/rust/issues/11403
macro_rules! decision_fn {
    ($decision:ident, $thenv:ident, $elsev:ident) => (
        fn concat_idents!($decision,_decision)
            (&self, req: &Request, resp: &mut Response) -> IronResult<Response> {
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
            let decision=$me.$decision($req, $resp);
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

pub trait Resource {
    /// Trait functions that can be overridden
    fn service_available(&self, req: &Request, resp: &mut Response) -> bool {
        return true;
    }

    fn known_method(&self, req: &Request, resp: &mut Response) -> bool {
        match req.method {
            method::Extension(_) => false,
            _ => true
        }
    }

    fn uri_too_long(&self, req: &Request, resp: &mut Response) -> bool {
        return false;
    }

    fn method_allowed(&self, req: &Request, resp: &mut Response) -> bool {
        match req.method {
            method::Get|method::Head => true,
            _ => false
        }
    }

    fn malformed(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn authorized(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn allowed(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn valid_content_header(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn known_content_type(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn valid_entity_length(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn exists(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn existed(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn respond_with_entity(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn new(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn post_redirect(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn put_to_different_url(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn multiple_representations(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn conflict(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn can_post_to_missing(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn can_post_to_gone(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn can_put_to_missing(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn moved_permanently(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn moved_temporarily(&self, req: &Request, resp: &mut Response) -> bool {
        false
    }

    fn delete_enacted(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn processable(&self, req: &Request, resp: &mut Response) -> bool {
        true
    }

    fn etag(&self, req: &Request,
            resp: &mut Response) -> Option<headers::Etag> {
        None
    }

    fn last_modified(&self, req: &Request, resp: &Response) -> Option<Tm> {
        None
    }

    /// Actions
    fn delete(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        assert!(false, "not implemented, delete");
        Ok(Response::new())
    }

    fn patch(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        assert!(false, "not implemented, patch");
        Ok(Response::new())
    }

    fn post(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        assert!(false, "not implemented, post");
        Ok(Response::new())
    }

    fn put(&self, req: &Request, resp: &mut Response) -> IronResult<Response> {
        assert!(false, "not implemented, put");
        Ok(Response::new())
    }



    /// some data returning methods - not sure these are really wanted
    fn available_languages(&self, req: &Request,
                           resp: &mut Response) -> Vec<String> {
        vec!["*".to_string()]
    }

    fn available_charsets(&self, req: &Request,
                          resp: &mut Response) -> Vec<String> {
        vec!["UTF-8".to_string()]
    }

    fn available_encodings(&self, req: &Request,
                           resp: &mut Response) -> Vec<Encoding> {
        vec![Encoding::Identity]
    }

    fn available_content_types(&self, req: &Request,
                               resp: &mut Response) -> Vec<Mime> {
        vec![]
    }


    /// base logic
    fn accept_exists(&self, req: &Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::Accept>() {
            Some(_) => true,
            None => match content_neg::best_content_type(
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

    fn media_type_available(&self, req: &Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::Accept>() {
            Some(cts) => match content_neg::best_content_type(
                &cts,
                &self.available_content_types(req,resp)) {
                Some(ct) => {resp.set_mut(ct); true}
                None => false
            },
            None => true
        }
    }



    fn language_available(&self, req: &Request, resp: &mut Response) -> bool {
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

    fn charset_available(&self, req: &Request, resp: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::AcceptCharset>() {
            Some(&hyper_headers::AcceptCharset(ref x)) => match content_neg::best_charset(
                x, &self.available_charsets(req,resp)) {
                Some(l) => {resp.set_mut(l); true}
                None => false
            },
            None =>  true
        }
    }

    fn encoding_available(&self, req: &Request, resp: &mut Response) -> bool {
        match req.headers.get::<headers::AcceptEncoding>() {
            Some(&headers::AcceptEncoding(ref x)) => match content_neg::best_encoding(
                x, &self.available_encodings(req,resp)) {
                Some(l) => {  // FIXME resp.set_mut(l);
                    true}
                None => false
            },
            None =>  true
        }
    }

    fn if_match(&self, req: &Request, resp: &mut Response) -> bool {
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

    fn if_none_match(&self, req: &Request, resp: &mut Response) -> bool {
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

    fn if_match_star(&self,
                     req: &Request,
                     resp: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::IfMatch>() {
            Some(&hyper_headers::IfMatch(hyper_headers::EntityTagMatch::Star)) =>
                true,
            _ => false
        }
    }

    fn if_none_match_star(&self, req: &Request, resp: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::IfNoneMatch>() {
            Some(&hyper_headers::IfNoneMatch(hyper_headers::EntityTagMatch::Star)) =>
                true,
            _ => false
        }
    }

    fn unmodified_since(&self, req: &Request, resp: &mut Response) -> bool {
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

    fn modified_since(&self, req: &Request, resp: &mut Response) -> bool {
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

    fn if_match_star_exists_for_missing(&self, req: &Request,
                                        resp: &mut Response) -> bool {
        match req.headers.get::<hyper_headers::IfMatch>() {
            Some(&hyper_headers::IfMatch(hyper_headers::EntityTagMatch::Star)) => true,
            _ => false
        }
    }

    /// logic functions
    fn service_available_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, service_available,
                       known_method_decision, handle_service_unavailable)
    }

    fn known_method_decision(&self, req: &Request,
                             resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, known_method,
                       uri_too_long_decision, handle_unknown_method)
    }

    fn uri_too_long_decision(&self, req: &Request,
                             resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, uri_too_long,
                       handle_uri_too_long, method_allowed_decision)
    }

    fn method_allowed_decision(&self, req: &Request,
                               resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, method_allowed,
                       malformed_decision, handle_method_not_allowed)
    }

    fn malformed_decision(&self, req: &Request,
                          resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, malformed,
                       handle_malformed, authorized_decision)
    }

    fn authorized_decision(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, authorized,
                       allowed_decision, handle_unauthorized)
    }

    fn allowed_decision(&self, req: &Request,
                        resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, allowed,
                       valid_content_header_decision, handle_forbidden)
    }

    fn valid_content_header_decision(&self,
                                     req: &Request,
                                     resp: &mut Response)
                                     -> IronResult<Response> {
        decision_body!(self, req, resp, valid_content_header,
                       known_content_type_decision, handle_not_implemented)
    }

    fn known_content_type_decision(&self,
                                   req: &Request,
                                   resp: &mut Response)
                                   -> IronResult<Response> {
        decision_body!(self, req, resp, known_content_type,
                       valid_entity_length_decision,
                       handle_unsupported_media_type)
    }

    fn valid_entity_length_decision(&self,
                                    req: &Request,
                                    resp: &mut Response)
                                    -> IronResult<Response> {
        decision_body!(self, req, resp, valid_entity_length,
                       is_options_decision,
                       handle_request_entity_too_large)
    }

    fn is_options_decision(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Options,
                       handle_options, accept_exists_decision)
    }

    fn accept_exists_decision(&self, req: &Request,
                              resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, accept_exists,
                       media_type_available_decision,
                       accept_language_exists_decision)
    }

    fn media_type_available_decision(&self,
                                     req: &Request,
                                     resp: &mut Response)
                                     -> IronResult<Response> {
        decision_body!(self, req, resp, media_type_available,
                       accept_language_exists_decision,
                       handle_not_acceptable)
    }

    fn accept_language_exists_decision(&self,
                                       req: &Request,
                                       resp: &mut Response)
                                       -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::AcceptLanguage>(req),
                       language_available_decision,
                       accept_charset_exists_decision)
                                       }

    fn language_available_decision(&self,
                                   req: &Request,
                                   resp: &mut Response)
                                   -> IronResult<Response> {
        decision_body!(self, req, resp, language_available,
                       accept_charset_exists_decision,
                       handle_not_acceptable)
    }

    fn accept_charset_exists_decision(&self,
                                      req: &Request,
                                      resp: &mut Response)
                                      -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::AcceptCharset>(req),
                       charset_available_decision,
                       accept_encoding_exists_decision)
    }

    fn charset_available_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, charset_available,
                       accept_encoding_exists_decision,
                       handle_not_acceptable)
    }

    fn accept_encoding_exists_decision(&self,
                                       req: &Request,
                                       resp: &mut Response)
                                       -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::AcceptEncoding>(req),
                       encoding_available_decision,
                       processable_decision)
    }

    fn encoding_available_decision(&self,
                                   req: &Request,
                                   resp: &mut Response)
                                   -> IronResult<Response> {
        decision_body!(self, req, resp, encoding_available,
                       processable_decision,
                       handle_not_acceptable)
    }

    fn processable_decision(&self,
                            req: &Request,
                            resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, processable,
                       exists_decision,
                       handle_unprocessable_entity)
    }

    fn exists_decision(&self,
                       req: &Request,
                       resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, exists,
                       if_match_exists_decision,
                       if_match_star_exists_for_missing_decision)
    }

    fn if_match_exists_decision(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::IfMatch>(req),
                       if_match_star_decision,
                       if_unmodified_since_exists_decision)
    }

    fn if_match_star_decision(&self,
                              req: &Request,
                              resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_match_star,
                       if_unmodified_since_exists_decision,
                       if_match_decision)
    }

    fn if_match_decision(&self,
                                          req: &Request,
                                          resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_match,
                       if_unmodified_since_exists_decision,
                       handle_precondition_failed)
    }

    fn if_unmodified_since_exists_decision(&self,
                                           req: &Request,
                                           resp: &mut Response)
                                           -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::IfUnmodifiedSince>(req),
                       if_unmodified_since_decision,
                       if_none_match_exists_decision)
    }

    fn if_unmodified_since_decision(&self,
                                 req: &Request,
                                 resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, unmodified_since,
                       handle_precondition_failed,
                       if_none_match_exists_decision)
    }

    fn if_none_match_exists_decision(&self,
                                     req: &Request,
                                     resp: &mut Response)
                                     -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<hyper_headers::IfNoneMatch>(req),
                       if_none_match_star_decision,
                       if_modified_since_exists_decision)
    }

    fn if_none_match_star_decision(&self,
                                   req: &Request,
                                   resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_none_match_star,
                       if_none_match_decision,
                       none_match_status_decision)
    }

    fn if_none_match_decision(&self,
                        req: &Request,
                        resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_none_match,
                       none_match_status_decision,
                       if_modified_since_exists_decision)
    }

    fn none_match_status_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp,
                       req.method == method::Get || req.method == method::Head,
                       handle_not_modified,
                       handle_precondition_failed)
    }

    fn if_modified_since_exists_decision(&self,
                                         req: &Request,
                                         resp: &mut Response)
                                         -> IronResult<Response> {
        decision_body!(self, req, resp,
                       header_exists::<headers::IfModifiedSince>(req),
                       if_modified_since_decision,
                       method_delete_decision)
    }

    fn if_modified_since_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, modified_since,
                       method_delete_decision,
                       handle_not_modified)
    }

    fn method_delete_decision(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Delete,
                       delete,
                       method_patch_decision)
    }

    fn method_patch_decision(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Patch,
                       patch,
                       post_to_existing_decision)
    }

    fn post_to_existing_decision(&self,
                                 req: &Request,
                                 resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Post,
                       post,
                       put_to_existing_decision)
    }

    fn put_to_existing_decision(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Put,
                       conflict_decision,
                       multiple_representations_decision)
    }

    // fn if_none_match_decision(&self,
    //                           req: &Request,
    //                           resp: &mut Response) -> IronResult<Response> {
    //     decision_body!(self, req, resp, if_none_match,
    //                    handle_not_modified,
    //                    handle_precondition_failed)
    // }

    fn if_match_star_exists_for_missing_decision(&self,
                                                 req: &Request,
                                                 resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, if_match_star_exists_for_missing,
                       handle_precondition_failed,
                       method_put_decision)
    }

    fn method_put_decision(&self,
                           req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Put,
                       put_to_different_url_decision,
                       existed_decision)
    }

    fn put_to_different_url_decision(&self,
                                     req: &Request,
                                     resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, put_to_different_url,
                       handle_moved_permanently,
                       can_put_to_missing_decision)
    }

    fn can_put_to_missing_decision(&self,
                                 req: &Request,
                                 resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, can_put_to_missing,
                       conflict_decision,
                       handle_not_implemented)
    }

    fn conflict_decision(&self,
                         req: &Request,
                         resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, conflict,
                       handle_conflict,
                       put)
    }

    fn existed_decision(&self,
                                 req: &Request,
                                 resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, existed,
                       moved_permanently_decision,
                       post_to_missing_decision)
    }

    fn moved_permanently_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, moved_permanently,
                       handle_moved_permanently,
                       moved_temporarily_decision)
    }

    fn moved_temporarily_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, moved_temporarily,
                       handle_moved_temporarily,
                       post_to_gone_decision)
    }

    fn post_to_gone_decision(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Post,
                       can_post_to_gone_decision,
                       handle_gone)
    }

    fn can_post_to_gone_decision(&self,
                                 req: &Request,
                                 resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, can_post_to_gone,
                       post,
                       handle_gone)
    }

    fn post_to_missing_decision(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, req.method == method::Post,
                       can_post_to_missing_decision,
                       handle_not_found)
    }

    fn can_post_to_missing_decision(&self,
                                    req: &Request,
                                    resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, can_post_to_missing,
                       post,
                       handle_not_found)
    }

    fn post_redirect_decision(&self,
                              req: &Request,
                              resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, post_redirect,
                       handle_see_other,
                       new_decision)
    }

    fn new_decision(&self,
                    req: &Request,
                    resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, new,
                       handle_created,
                       respond_with_entity_decision)
    }

    fn respond_with_entity_decision(&self,
                                    req: &Request,
                                    resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, respond_with_entity,
                       multiple_representations_decision,
                       handle_no_content)
    }

    fn multiple_representations_decision(&self,
                                    req: &Request,
                                    resp: &mut Response) -> IronResult<Response> {
        decision_body!(self, req, resp, multiple_representations,
                       handle_multiple_representations,
                       handle_ok)
    }

    /// default handlers
    fn handle_service_unavailable(&self,
                                  req: &Request,
                                  resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::ServiceUnavailable),
            response: Response::with((status::ServiceUnavailable,
                                      "Service unavailable"))})
    }

    fn handle_unknown_method(&self, req: &Request,
                             resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::UnknownMethod),
            response: Response::with((status::NotImplemented,
                                      "Unknown method"))})
    }

    fn handle_uri_too_long(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::RequestUriTooLong),
            response: Response::with((status::NotImplemented,
                                      "Request URI too long"))})
    }

    fn handle_method_not_allowed(&self, req: &Request,
                                 resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::MethodNotAllowed),
            response: Response::with((status::MethodNotAllowed,
                                      "Method not allowed"))})
    }

    fn handle_malformed(&self, req: &Request,
                        resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::BadRequest),
            response: Response::with((status::BadRequest, "Bad request"))})
    }

    fn handle_unauthorized(&self, req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::Unauthorized),
            response: Response::with((status::Unauthorized, "Unauthorized"))})
    }

    fn handle_forbidden(&self, req: &Request,
                        resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::Forbidden),
            response: Response::with((status::Forbidden, "Forbidden"))})
    }

    fn handle_not_implemented(&self, req: &Request,
                              resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::NotImplemented),
            response: Response::with((status::NotImplemented,
                                      "Not implemented"))})
    }

    fn handle_unsupported_media_type(&self,
                                     req: &Request,
                                     resp: &mut Response)
                                     -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::UnsupportedMediaType),
            response: Response::with((status::UnsupportedMediaType,
                                      "Unsupported media type"))})
    }

    fn handle_request_entity_too_large(&self,
                                       req: &Request,
                                       resp: &mut Response)
                                       -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::RequestEntityTooLarge),
            response: Response::with((status::RequestEntityTooLarge,
                                      "Request entity too large"))})
    }

    fn handle_not_acceptable(&self,
                             req: &Request,
                             resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::NotAcceptable),
            response: Response::with((status::NotAcceptable,
                                      "Not Acceptable"))})
    }

    fn handle_unprocessable_entity(&self,
                                   req: &Request,
                                   resp: &mut Response)
                                   -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::UnprocessableEntity),
            response: Response::with((status::UnprocessableEntity,
                                      "Unprocessable entity"))})
    }

    fn handle_conflict(&self,
                       req: &Request,
                       resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::Conflict),
            response: Response::with((status::Conflict,
                                      "Conflict"))})
    }

    fn handle_see_other(&self,
                        req: &Request,
                        resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::SeeOther),
            response: Response::with((status::SeeOther,
                                      "Conflict"))})
    }

    fn handle_created(&self,
                       req: &Request,
                       resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::Created),
            response: Response::with((status::Created,
                                      "Created"))})
    }

    fn handle_precondition_failed(&self,
                                  req: &Request,
                                  resp: &mut Response)
                                  -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::PreconditionFailed),
            response: Response::with((status::PreconditionFailed,
                                      "Precondition failed"))})
    }

    fn handle_not_modified(&self,
                           req: &Request,
                           resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::NotModified),
            response: Response::with((status::NotModified,
                                      "Not modified"))})
    }

    fn handle_moved_permanently(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::PermanentRedirect),
            response: Response::with((status::PermanentRedirect,
                                      "Permanent redirect"))})
    }

    fn handle_moved_temporarily(&self,
                                req: &Request,
                                resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::TemporaryRedirect),
            response: Response::with((status::TemporaryRedirect,
                                      "Temporary redirect"))})
    }

    fn handle_gone(&self,
                   req: &Request,
                   resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::Gone),
            response: Response::with((status::Gone,
                                      "Gone"))})
    }

    fn handle_not_found(&self,
                   req: &Request,
                   resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::NotFound),
            response: Response::with((status::NotFound,
                                      "Not found"))})
    }

    fn handle_no_content(&self,
                         req: &Request,
                         resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::NoContent),
            response: Response::with((status::NoContent,
                                      "No content"))})
    }

    fn handle_multiple_representations(&self,
                                       req: &Request,
                                       resp: &mut Response) -> IronResult<Response> {
        Err(IronError{
            error: Box::new(ResourceError::MultipleChoices),
            response: Response::with((status::MultipleChoices,
                                      "Multiple Choices"))})
    }

    fn handle_ok(&self,
                 req: &Request,
                 resp: &mut Response) -> IronResult<Response> {
        resp.set_mut(status::Ok);
        // Ok(*resp);;
        // Fixme
        Ok(Response::new())

    }

    fn handle_options(&self, req: &Request,
                      resp: &mut Response) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "")))
    }


}

pub fn handle<T>(resource: &T,
                 req: &mut Request) -> IronResult<Response> where T: Resource {
    let mut resp = Response::new();
    try!(resource.service_available_decision(req, &mut resp));
    Ok(resp)
}

// pub fn hyper_handle<T: Resource>(resource: &T,
//                                  req: &hyper::server::Request,
//                                  resp: &mut hyper::server::Response
//                                  ) -> Result<(),()> where T: Resource {
//     match resource.service_available_decision(req, &mut resp) {
//         Ok(_) => (),
//         Err(_) => ()
//     };
// }

fn header_exists<T: headers::Header+headers::HeaderFormat>(req: &Request) -> bool {
    req.headers.get::<T>().is_some()
}

fn method_eq(req: &Request, method: method::Method) -> bool {
    req.method == method
}

// pub fn service_available<T>(resource: &T,
//                             req: &Request,
//                             resp: &mut Response) -> IronResult<Response>
//        where T: Resource {
//     service_available(resource, req, resp)
// }

macro_rules! resource {
    ($s:ident) => {
        impl Resource for $s {}
        resource_handler!($s);
    }
}

macro_rules! resource_handler {
    ($s:ident) => {
        impl Handler for $s {
            fn handle(&self, req: &mut Request) -> IronResult<Response> {
            handle(self, req)
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::old_io;
    use std::old_io::fs;
    use std::ops;
    use std::sync::{Arc,Mutex};
    use hyper::{self, IpAddr};
    use hyper::server::Listening;
    use iron::{Handler, Iron, IronError, IronResult, Request, Response, status};
    use iron::error::HttpResult;
    use iron::modifier::Set;


    fn http_server<T>(resource: T) -> HttpResult<Listening> where T: Resource+Handler+Sync+Send {
        let address = IpAddr::Ipv4Addr(127,0,0,1);
        let count = Arc::new(Mutex::new(0u8));
        let server_count = count.clone();

        // let f = |req: &mut Request| {
        //     handle(resource, req).unwrap();
        // };

        // let g : &::std::ops::Fn((&mut Request)) = &f;

        // let i = Iron::new(resource);

        Iron::new(resource).listen((address,0u16))
    }

    struct GetOk;
    resource!(GetOk);
    // impl Resource for GetOk {}

    // impl Handler for GetOk {
    //     fn handle(&self, req: &mut Request) -> IronResult<Response> {
    //         super::handle(self, req)
    //     }
    // }

    #[test]
    fn test_get_ok() {
      let mut listen = http_server(GetOk).unwrap();
      let mut client = hyper::Client::new();
       match client.get(&format!("http://127.0.0.1:{}", listen.socket.port)[])
          .send() {
              Ok(ref mut r) => {
                  assert_eq!("", r.read_to_string().unwrap());
                  assert_eq!(status::Ok, r.status);
              },
              Err(x) => assert!(false, "get failed")
          };
       listen.close().unwrap();
    }


    struct GetOkContent;
    resource_handler!(GetOkContent);

    impl Resource for GetOkContent {
       fn handle_ok(&self, req: &Request, resp: &mut Response) -> IronResult<Response> {
          resp.set_mut((status::Ok, "hello"));
          Ok(Response::new())
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
              Err(x) => assert!(false, "get failed")
          };
       listen.close().unwrap();
    }
}
