extern crate either;
#[macro_use]
extern crate log;
#[macro_use]
extern crate monto;
extern crate pretty_logger;
extern crate serde_json;
extern crate tokio_core;
extern crate unicode_segmentation;
extern crate void;

use std::fmt::Display;

use either::{Left, Right};
use monto::common::messages::{Language, Product, ProductIdentifier, ProductName};
use monto::service::Service;
use monto::service::config::Config;
use monto::service::messages::{ServiceError, ServiceNotice};
use serde_json::Value;
use tokio_core::reactor::Core;
use unicode_segmentation::UnicodeSegmentation;
use void::unreachable;

fn main() {
    pretty_logger::init_to_defaults().unwrap();
    let mut c = Core::new().unwrap();
    let config = Config::load("example_services");
    let mut s = Service::new(config, c.handle());

    s.add_provider(CharCount);
    s.add_provider(LineCount);
    s.add_provider(Reverse);

    let err = match c.run(s.serve_forever()) {
        Ok(void) => unreachable(void),
        Err(Right(err)) => err,
        Err(Left(void)) => unreachable(void),
    };
    error!("{}", err);
}

simple_service_provider! {
    name = CharCount;
    product = "edu.umn.cs.melt.monto_example_services.char_count";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| -> Result<_, &str> {
            Ok(src.len().into())
        })
    }
}

simple_service_provider! {
    name = LineCount;
    product = "edu.umn.cs.melt.monto_example_services.line_count";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| -> Result<_, &str> {
            let line_count = src.chars()
                .filter(|&c| c == '\n')
                .count();
            Ok(line_count.into())
        })
    }
}

simple_service_provider! {
    name = Reverse;
    product = "edu.umn.cs.melt.monto_example_services.reverse";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| -> Result<_, &str> {
            let mut graphemes = src.graphemes(true)
                .collect::<Vec<_>>();
            graphemes.reverse();
            let reversed = graphemes.into_iter()
                .collect::<String>();
            Ok(Value::String(reversed))
        })
    }
}

fn simple_fn<E: Display, F: FnOnce(String) -> Result<Value, E>>(path: &str, mut products: Vec<Product>, lang: Language, f: F) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    let idx = products.iter().position(|p| {
        p.name == ProductName::Source && p.language == lang && p.path == path
    });

    let r = if let Some(idx) = idx {
        match products.swap_remove(idx).value {
            Value::String(src) => {
                f(src).map_err(|e| ServiceError::Other(e.to_string()))
            }
            _ => Err(ServiceError::Other("bad source product".to_string())),
        }
    } else {
        Err(ServiceError::UnmetDependency(ProductIdentifier {
            name: ProductName::Source,
            language: lang,
            path: path.to_string(),
        }))
    };
    let notices = products.into_iter()
        .map(|p| p.into())
        .map(ServiceNotice::UnusedDependency)
        .collect();
    (match r {
        Ok(product) => Ok(product),
        Err(err) => Err(vec![err]),
    }, notices)
}
