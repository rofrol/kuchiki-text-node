extern crate kuchiki;

use kuchiki::traits::*;

fn main() {
    let html = r"<a href='https://example.com'><em>@</em>Bananowy</a>";

    let document = kuchiki::parse_html().one(html);

    let selector = "a";
    let anchor = document.select_first(selector).unwrap();
    // Quick and dirty hack
    let last_child = anchor.as_node().last_child().unwrap();
    println!("{:?}", last_child.into_text_ref().unwrap());

    // Iterating solution
    for children in anchor.as_node().children() {
        if let Some(a) = children.as_text() {
            println!("{:?}", a);
        }
    }

    // Iterating solution - Using `text_nodes()` iterators
    anchor.as_node().children().text_nodes().for_each(|e| {
        println!("{:?}", e);
    });

    let last = match anchor.as_node().children().text_nodes().last() {
        Some(x) => x.as_node().text_contents(),
        None => String::from(""),
    };
    println!("{:?}", last);
}
