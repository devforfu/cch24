use actix_web::web::Bytes;
use actix_web::{post, HttpResponse};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use cargo_manifest::{Manifest, MaybeInherited};
use toml::{Table, Value};

#[derive(Clone, Debug, PartialEq)]
struct PackageParsingError(String);

impl Display for PackageParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.0)
    }
}

struct Order {
    pub item: String,
    pub quantity: u32,
}

#[post("/5/manifest")]
async fn manifest(bytes: Bytes) -> HttpResponse {
    if let Ok(data) = String::from_utf8(bytes.to_vec()) {

        if let Some(error) = validate_manifest(&data) {
            return HttpResponse::BadRequest().body(error);
        }

        return if let Ok(orders) = parse_data(&data) {
            if orders.is_empty() {
                return HttpResponse::NoContent().finish();
            }
            let mut items: HashMap<String, u32> = HashMap::new();
            let mut ordered_names = vec![];
            for order in orders.into_iter() {
                ordered_names.push(order.item.clone());
                let quantity = order.quantity;
                items.entry(order.item)
                    .and_modify(|e| *e += quantity)
                    .or_insert(quantity);
            }
            let mut body = String::new();
            for key in ordered_names.into_iter() {
                body.push_str(&format!("{}: {}\n", key, items.get(&key).unwrap()));
            }
            HttpResponse::Ok().body(String::from(body.trim()))
        } else {
            HttpResponse::NoContent().finish()
        }
    }
    HttpResponse::BadRequest().finish()
}

impl std::error::Error for PackageParsingError {}

macro_rules! err {
    ($text:expr) => {
        PackageParsingError(String::from($text))
    };
}

fn validate_manifest(data: &str) -> Option<String> {
    let mut error: Option<String> = None;
    if let Ok(parsed_manifest) = Manifest::from_str(data) {
        let keywords: Vec<String> = parsed_manifest.package
            .and_then(|p| p.keywords)
            .and_then(
                |kw| match kw {
                    MaybeInherited::Inherited { workspace: _ } => None,
                    MaybeInherited::Local(keywords) => Some(keywords)
                }
            ).unwrap_or(vec![]);
        let needle = String::from("Christmas 2024");
        if !keywords.contains(&needle) {
            error.replace(String::from("Magic keyword not provided"));
        }
    } else {
        error.replace(String::from("Invalid manifest"));
    }
    error
}

fn parse_data(data: &str) -> Result<Vec<Order>, PackageParsingError> {
    let table = data.parse::<Table>().map_err(|_| err!("cannot parse toml"))?;
    Ok(
        table.get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("orders"))
        .map(|orders| {
            let mut parsed = vec![];
            if let Value::Array(orders) = orders {
                for order in orders.iter() {
                    if let Some(order) = order.as_table() {
                        if let (Some(item), Some(quantity)) = (order.get("item"), order.get("quantity")) {
                            if let (Some(s), Some(x)) = (item.as_str(), quantity.as_integer()) {
                                parsed.push(Order {
                                    item: String::from(s),
                                    quantity: x as u32,
                                });
                            }
                        }
                    }
                }
            }
            parsed
        })
        .unwrap_or(vec![])
    )
}