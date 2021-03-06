extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{And, Attr, Class, Name, Predicate};
use select::node::Node;

fn main() {
    parse_bot_api("https://core.telegram.org/bots/api");
}

fn parse_bot_api(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    let document = Document::from_read(resp).unwrap();

    let content = document
        .find(Attr("id", "dev_page_content"))
        .next()
        .unwrap();
    let mut builder = TeleTypeBuilder::new();

    // TODO using next() and checking if a node is "\n", it should be easier to parse,
    // so I can use a more common builder type, without states,
    // that always builds just one type at a time.
    for node in content.children() {
        //println!("{:?}", node.name());

        if node.is(Name("h4")) {
            let is_camel = is_camel_case(node.text());
            let has_anchor = has_anchor(node);
            //println!("{} is_camel: {:?}", node.text(), is_camel);
            if is_camel {
                builder.new_type(node.text());
            }
            if let Some(mut next) = node.next() {
                // NOTE: there are newlines which are considered to be nodes!
                // we need to jump over them!
                //println!("before: {:?}", next.html());
                next = next.next().unwrap();
                //println!("after: {:?}", next.html());
                if next.is(Name("p")) {
                    builder.add_description(next.text());
                }
            }
        }
    }
    println!("{:?}", builder);
}

#[derive(Debug)]
struct TeleTypeBuilder {
    types: Vec<TeleType>,
}

impl TeleTypeBuilder {
    fn new() -> Self {
        TeleTypeBuilder { types: Vec::new() }
    }

    fn new_type<T: ToString>(&mut self, name: T) {
        let t = TeleType {
            name: name.to_string(),
            description: None,
            fields: Vec::new(),
        };
        self.types.push(t);
    }

    fn add_description<T: ToString>(&mut self, desc: T) {
        let last = self.types.last_mut();
        match last {
            Some(t) => t.description = Some(desc.to_string()),
            None => {}
        }
    }
}

#[derive(Debug)]
struct TeleType {
    name: String,
    description: Option<String>,
    fields: Vec<TeleFields>,
}

#[derive(Debug)]
struct TeleFields {
    name: String,
    is_optional: bool,
    type_str: String,
    description: String,
}

fn has_anchor(node: Node) -> bool {
    let name = node.text().to_lowercase();
    let anchor = node.find(And(Class("anchor"), Name(name.as_str()))).next();
    if anchor.is_some() {
        return true;
    } else {
        return false;
    };
}

fn is_camel_case<T: AsRef<str>>(string: T) -> bool {
    let vec: Vec<char> = string.as_ref().chars().collect();
    let first = vec[0];
    if !first.is_uppercase() || !first.is_alphabetic() {
        return false;
    }
    for c in vec {
        if !c.is_alphabetic() {
            return false;
        }
    }
    true
}

#[test]
fn test_camel_case() {
    assert!(is_camel_case("GetUpdates"));
    assert!(!is_camel_case("getUpdates"));
    assert!(!is_camel_case("getUpd4t3s"));
}
