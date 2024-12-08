use std::fmt::{Debug, Display, Formatter};
use actix_web::{get, web, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct Dest {
    from: String,
    key: String,
}

#[derive(Deserialize)]
struct Check {
    from: String,
    to: String,
}

#[derive(Clone, Debug, PartialEq)]
struct RequestError(String);

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.0)
    }
}

impl std::error::Error for RequestError {}

#[get("/2/dest")]
async fn egregious_encryption(query: web::Query<Dest>) -> String {
    let dest = query.0;
    if let Ok(value) = compute_octets_with_overflow(&dest.from, &dest.key, Op::Add) {
        value
    } else {
        String::from("invalid octets")
    }
}

#[get("/2/key")]
async fn egregious_encryption_check(query: web::Query<Check>) -> String {
    let check = query.0;
    if let Ok(value) = compute_octets_with_overflow(&check.from, &check.to, Op::Sub) {
        value
    } else {
        String::from("invalid octets")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    Add,
    Sub,
}

fn compute_octets_with_overflow(first: &str, second: &str, op: Op) -> Result<String, RequestError> {
    let oct_fst = get_octets(first);
    let oct_snd = get_octets(second);
    if let (Some(from), Some(key)) = (oct_fst, oct_snd) {
        let mut added = vec![];
        for (o1, o2) in from.into_iter().zip(key.into_iter()) {
            let (value, _) = match op {
                Op::Add => o1.overflowing_add(o2),
                Op::Sub => o2.overflowing_sub(o1),
            };
            added.push(value);
        }
        Ok(added.into_iter().map(|o| format!("{}", o)).collect::<Vec<_>>().join("."))
    } else {
        Err(RequestError(format!(
            "cannot decode request: octets are {} and {}", first, second
        )))
    }
}

fn get_octets(s: &str) -> Option<Vec<u8>> {
    let octets: Vec<_> = s.split('.').flat_map(|oct| oct.parse::<u8>()).collect();
    if octets.len() != 4 { None } else { Some(octets) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_octets() {
        assert_eq!(get_octets("10.128.52.14"), Some(vec![10, 128, 52, 14]));
        assert_eq!(get_octets("10.128.52"), None);
    }

    #[test]
    fn test_op_octets_with_overflow() {
        let fst = "1.2.3.4".to_string();
        let snd = "255.128.252.101".to_string();
        assert_eq!(
            compute_octets_with_overflow(&fst, &snd, Op::Add),
            Ok("0.130.255.105".to_string())
        );
        assert_eq!(
            compute_octets_with_overflow(&snd, &fst, Op::Sub),
            Ok("2.130.7.159".to_string())
        );
    }
}