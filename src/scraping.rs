use scraper::{ElementRef, Html};

/// Gets all classes from an html
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let c = get_classes(
            "<h1 class=\"hey hello\">Hello, <i class=\"hiii\">world!</i></h1>
<h1 class=\"hey hello\">Hello, <i class=\"hiii\">world!</i></h1>",
        );

        assert_eq!(c, vec!["hey hello", "hiii", "hey hello", "hiii",]);
    }
}
