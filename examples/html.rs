use zephyr::{scraping::*, Zephyr};

fn main() {
    let body = r#"<body>
    <p class="color[#e20f00] color[green]hover content['*']$before">
        this text is red, but green on hover
    </p>
    <p class="mt[10rem] content[attr(after)]$after color[red]$after" after="hi, this is an after text">
        this text has a lot of margin
    </p>
</body>"#;

    let classes = get_classes(&body);

    let z = Zephyr::new();
    let css = z.generate_classes(classes.iter().map(String::as_str));

    let html = format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
        <style>{css}</style>
    </head>
    {body}
</html>
"#
    );

    std::fs::write("./examples/index.html", html).unwrap();
}
