use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use std::collections::HashMap;
use std::convert::TryFrom;

use std::result::Result;

use super::RestFile;
use super::RestMethod;
use super::RestRequest;

#[derive(Parser)]
#[grammar = "rest/grammar.pest"]
struct RestParser;

impl<'i> TryFrom<Pair<'i, Rule>> for RestFile {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut requests = vec![];
        for item in iterator {
            match item.as_rule() {
                Rule::EOI => {
                    break;
                }
                Rule::request => {
                    requests.push(item.try_into()?);
                }
                _ => {}
            }
        }
        Ok(Self { requests })
    }
}

impl TryFrom<&str> for RestFile {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let file = RestParser::parse(Rule::file, input.trim_start())
            .expect("unable to parse")
            .next()
            .unwrap();
        RestFile::try_from(file).map_err(|e| e.to_string())
    }
}

impl RestRequest {
    fn parse_headers(&mut self, pairs: Pairs<Rule>) {
        for item in pairs {
            let mut kv = item.into_inner();
            let key = kv.next().unwrap().as_str().to_string();
            let value = kv.next().unwrap().as_str().to_string();
            self.headers.insert(key, value);
        }
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for RestRequest {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        let mut ret = Self {
            method: iterator.next().unwrap().try_into()?,
            url: iterator.next().unwrap().as_str().to_string(),
            version: iterator.next().unwrap().as_str().to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };

        for item in iterator {
            match item.as_rule() {
                Rule::headers => {
                    ret.parse_headers(item.into_inner());
                }
                Rule::body => {
                    ret.body = item.as_str().trim().to_string();
                }
                _ => {
                    unreachable!();
                }
            }
        }

        Ok(ret)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for RestMethod {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        Ok(match pair.as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            _ => unreachable!(),
        })
    }
}
