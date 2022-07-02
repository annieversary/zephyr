//! for situations where you have a set of components,
//! and you want to register the classes you use from different files without too much complexity

use zephyr::{register_class, Zephyr};

fn main() {
    let z = Zephyr::new();
    let generated_css = z.generate_from_inventory();

    let head = head(&generated_css);

    let header = header();
    let body = body();

    let html = format!(
        r#"
<!DOCTYPE html>
<html>
  {head}
  <body>
    {header}
    {body}
  </body>
</html>
"#
    );

    std::fs::write("./examples/index.html", html).unwrap();
}

fn head(generated_css: &str) -> String {
    format!(
        r#"
        <head>
          <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
          <style>{generated_css}</style>
        </head>
        "#
    )
}

fn header() -> String {
    let class = register_class!("color[#e20f00] color[green]hover content['*']$before");
    format!(
        r#"
    <p class="{class}">
      this text is red, but green on hover
    </p>
    "#
    )
}

fn body() -> String {
    let class = register_class!("mt[10rem] content[attr(after)]$after color[red]$after");
    format!(
        r#"
    <p class="{class}" after="hi, this is an after text">
      this text has a lot of margin
    </p>
    "#
    )
}
