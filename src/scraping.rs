use regex::Regex;
use scraper::{ElementRef, Html};

/// Gets all classes from an html, using the scraper crate
///
/// If you have a templated html source and it's failing to get some classes, you might want
/// to use `get_classes_regex` instead.
///
/// ```
/// # use zephyr::{*, scraping::*};
/// # fn main() {
/// let c = get_classes("<h1 class=\"m[1rem]\">Hello world!</h1>");
/// let z = Zephyr::new();
/// let css = z.generate_classes(c.iter().map(String::as_str));
/// # }
/// ```
pub fn get_classes(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let mut classes = vec![];
    let mut queue: Vec<ElementRef> = vec![document.root_element()];

    while let Some(handle) = queue.pop() {
        let el = handle.value();
        if let Some(c) = el.attr("class") {
            classes.push(c.to_string());
        }

        for child in handle.children() {
            if let Some(child) = ElementRef::wrap(child) {
                queue.push(child);
            }
        }
    }
    classes
}

lazy_static::lazy_static! {
    static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

/// Gets all classes from an html, using a regex
///
/// It's less accurate than `get_classes`, meaning it will find more false positives.
/// Use this if you have a templated html source and don't mind generating more classes than there actually are.
pub fn get_classes_regex(html: &str) -> Vec<&str> {
    let mut classes = vec![];
    for capture in STYLE_REGEX.captures_iter(html) {
        if let Some(group) = capture.get(1) {
            classes.push(group.as_str())
        }
    }

    classes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scraper() {
        let c = get_classes(
            "<h1 class=\"hey hello\">Hello, <i class=\"hiii\">world!</i></h1>
<h1 class=\"hey hello\">Hello, <i class=\"hiii\">world!</i></h1>",
        );

        assert_eq!(c, vec!["hey hello", "hiii", "hey hello", "hiii",]);
    }

    #[test]
    fn test_parse_regex() {
        let c = get_classes_regex(
            "<h1 class=\"hey hello\">Hello, <i class=\"hiii\">world!</i></h1>
<h1 class=\"hey hello\">Hello, <i class=\"hiii\">world!</i></h1>",
        );

        assert_eq!(c, vec!["hey hello", "hiii", "hey hello", "hiii",]);
    }
}
