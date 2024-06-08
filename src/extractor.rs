use html5ever::tendril::stream::TendrilSink;
use html5ever::{parse_document, serialize};
use markup5ever_rcdom::{RcDom, SerializableHandle};
use scorer::Candidate;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::default::Default;
use std::path::Path;
use url::Url;

use crate::dom;
use crate::error::Error;
use crate::scorer;

#[derive(Debug, Clone)]
pub struct Product {
    pub title: String,
    /// Normalised HTML content
    pub content: String,
    /// Human readable text
    pub text: String,
}

pub(crate) fn parse(input: &str) -> Result<RcDom, Error> {
    let tendril = html5ever::tendril::Tendril::try_from_byte_slice(input.as_bytes())
        .map_err(|_| Error::InvalidTendrilBuffer)?;

    Ok(parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one(tendril))
}

pub fn extract(input: &str, url: &Url) -> Result<Product, Error> {
    let mut dom = parse(input)?;

    let mut title = String::new();
    let mut candidates = BTreeMap::new();
    let mut nodes = BTreeMap::new();
    let handle = dom.document.clone();
    scorer::preprocess(&mut dom, handle.clone(), &mut title);
    scorer::find_candidates(Path::new("/"), handle.clone(), &mut candidates, &mut nodes);
    let mut id: &str = "/";
    let mut top_candidate: &Candidate = &Candidate {
        node: handle.clone(),
        score: Cell::new(0.0),
    };

    for (i, candidate) in candidates.iter() {
        let score =
            candidate.score.get() * (1.0 - scorer::get_link_density(candidate.node.clone()));
        candidate.score.set(score);
        if score <= top_candidate.score.get() {
            continue;
        }
        id = i;
        top_candidate = candidate;
    }
    let mut bytes = vec![];

    let node = top_candidate.node.clone();
    scorer::clean(&mut dom, Path::new(id), node.clone(), url, &candidates);

    serialize(
        &mut bytes,
        &SerializableHandle::from(node.clone()),
        Default::default(),
    )
    .ok();

    let content = String::from_utf8_lossy(&bytes).to_string();

    let mut text: String = String::new();
    dom::extract_text(node.clone(), &mut text, true);
    Ok(Product {
        title,
        content,
        text,
    })
}
