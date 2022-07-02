fn main() {
    // this list of used classes would ideally be parsed out of the written html,
    // but i don't want to over complicate this example
    let classes = [
        "mt[10rem]",
        "color[#e20f00]",
        "color[green]hover",
        "content[attr(after)]$after",
        "content['*']$before",
        "color[red]$after",
    ];

    let z = zephyr::Zephyr::new();
    let css = z.generate_classes(classes);

    let html = format!(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
    <style>{css}</style>
  </head>
  <body>
    <p class="color[#e20f00] color[green]hover content['*']$before">
      this text is red, but green on hover
    </p>
    <p class="mt[10rem] content[attr(after)]$after color[red]$after" after="hi, this is an after text">
      this text has a lot of margin
    </p>
  </body>
</html>
"#
    );

    std::fs::write("./examples/index.html", html).unwrap();
}
