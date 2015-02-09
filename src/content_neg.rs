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

/// Content Negotiation
use iron::headers::{Encoding,QualityItem};
use mime::{Mime,TopLevel,SubLevel};
use std::cmp::Ordering::Equal;
use log;

/// Compare a requested mime type x (with wild cards), to an available
/// mime type y, to see if they match
fn mime_match(x: &Mime, y: &Mime) -> bool {
    match x {
        &Mime(TopLevel::Star,_,_) => true,
        &Mime(ref tl,SubLevel::Star,_) => tl==&y.0,
        _ => x==y
    }
}

/// Return the best allowed content type for the request.  The best
/// type is the first type in the Accept header that is available.
pub fn best_content_type(accept: &Vec<QualityItem<Mime>>,
                         avail: &Vec<Mime>) -> Option<Mime> {
    debug!("best_content_type {:?} in {:?}",accept, avail);
    let mut accept = accept.clone();
    accept.sort_by(
        |x,y|
        x.quality.partial_cmp(&y.quality)
            .unwrap_or(Equal)
            .reverse());
    match accept.iter()
        .find(|m|
              avail.iter().find(|a| mime_match(&m.item, &a)).is_some()) {
            Some(qi) => Some(qi.item.clone()),
            None => None
        }
}

/// Compare a requested language type x (with wild cards), to an available
/// language type y, to see if they match
fn language_match(x: &String, y: &String) -> bool {
    if x==&"*".to_string() { return true; }
    x==y
}

/// Return the best allowed language.
pub fn best_language(mut accept: Vec<QualityItem<String>>,
                     avail: &Vec<String>) -> Option<String> {
    accept.sort_by(
        |x,y|
        x.quality.partial_cmp(&y.quality)
            .unwrap_or(Equal)
            .reverse());
    match accept.iter()
        .find(|m|
              avail.iter().find(|a| language_match(&m.item, &a)).is_some()) {
            Some(qi) => Some(qi.item.clone()),
            None => None
        }
}

/// Compare a requested charset type x (with wild cards), to an available
/// charset type y, to see if they match
fn charset_match(x: &String, y: &String) -> bool {
    if x==&"*".to_string() { return true; }
    x==y
}

/// Return the best allowed charset.
pub fn best_charset(acceptv: &Vec<QualityItem<String>>,
                    avail: &Vec<String>) -> Option<String> {
    let mut accept = acceptv.clone();
    accept.sort_by(
        |x,y|
        x.quality.partial_cmp(&y.quality)
            .unwrap_or(Equal)
            .reverse());
    match accept.iter()
        .find(|m|
              avail.iter().find(|a| charset_match(&m.item, &a)).is_some()) {
            Some(qi) => Some(qi.item.clone()),
            None => None
        }
}

/// Compare a requested encoding type x (with wild cards), to an available
/// encoding type y, to see if they match
fn encoding_match(x: &Encoding, y: &Encoding) -> bool {
    x==y
}

/// Return the best allowed encoding.
pub fn best_encoding(accept: &Vec<QualityItem<Encoding>>,
                     avail: &Vec<Encoding>) -> Option<Encoding> {
    let mut accept = accept.clone();
    accept.sort_by(
        |x,y|
        x.quality.partial_cmp(&y.quality)
            .unwrap_or(Equal)
            .reverse());
    match accept.iter()
        .find(|m|
              avail.iter().find(|a| encoding_match(&m.item, &a)).is_some()) {
            Some(qi) => Some(qi.item.clone()),
            None => None
        }
}


#[cfg(test)]
mod tests {
    use super::mime_match;
    use mime::{Mime,TopLevel,SubLevel};

    #[test]
    fn test_mime_match() {
        assert!(
            mime_match(&Mime(TopLevel::Star, SubLevel::Star, vec![]),
                       &Mime(TopLevel::Text, SubLevel::Plain, vec![])));
        assert!(
            !mime_match(&Mime(TopLevel::Application, SubLevel::Star, vec![]),
                        &Mime(TopLevel::Text, SubLevel::Plain, vec![])));
        assert!(
            mime_match(&Mime(TopLevel::Application, SubLevel::Star, vec![]),
                       &Mime(TopLevel::Application, SubLevel::Json, vec![])));
        assert!(
            mime_match(&Mime(TopLevel::Application, SubLevel::Json, vec![]),
                       &Mime(TopLevel::Application, SubLevel::Json, vec![])));
        assert!(
            !mime_match(&Mime(TopLevel::Application, SubLevel::Json, vec![]),
                        &Mime(TopLevel::Application,
                              SubLevel::Ext("yaml".to_string()),
                              vec![])));
    }
}
