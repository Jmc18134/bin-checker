use std::env;
use std::error::Error;

use reqwest::Url;
use select::document::Document;
use select::predicate::{Attr, Class};

fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("BIN_URL")?;
    let url = Url::parse(url.as_str())?;
    let body = reqwest::blocking::get(url)?;

    Document::from_read(body)?
        .find(Attr("id", "yourNextCollection"))
        .into_selection()
        .children()
        .filter(Class("binColour"))
        .iter()
        .for_each(|colour| println!("{}", colour.as_text().unwrap()));

    Ok(())
}
