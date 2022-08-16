use super::*;
use class::Class;

#[test]
fn generate_margin_works() {
    let z = Zephyr::new();

    let class = Class {
        property: "m",
        value: Some("1rem"),
        modifiers: vec![].into(),
        pseudo: None,
        original: "m[1rem]",
        value_type: class::ValueType::Normal,
    };
    let css = class.generate(&z, 0).unwrap();
    assert_eq!(css, r#".m\[1rem\]{margin:1rem}"#);

    let class = Class {
        property: "m",
        value: Some("1rem"),
        modifiers: vec!["focus"].into(),
        pseudo: None,
        original: "m[1rem]focus",
        value_type: class::ValueType::Normal,
    };
    let css = class.generate(&z, 0).unwrap();
    assert_eq!(css, r#".m\[1rem\]focus:focus{margin:1rem}"#);

    let class = Class {
        property: "m",
        value: Some("1rem"),
        modifiers: vec!["focus", "hover", "odd"].into(),
        pseudo: None,
        original: "m[1rem]focus,hover,odd",
        value_type: class::ValueType::Normal,
    };
    let css = class.generate(&z, 0).unwrap();
    assert_eq!(
        css,
        r#".m\[1rem\]focus,hover,odd:focus:hover:nth-child\(odd\){margin:1rem}"#
    );
}

#[test]
fn generate_classes_works() {
    let z = Zephyr::new();

    let classes = z.generate_classes(["flex-row"]);
    assert_eq!(classes, r#".flex-row{display:flex;flex-direction:row}"#);

    let classes = z.generate_classes(["m[3rem]hover,focus$placeholder"]);
    assert_eq!(
        classes,
        r#".m\[3rem\]hover,focus\$placeholder:hover:focus::placeholder{margin:3rem}"#
    );

    let classes = z.generate_classes(["flex|hover,focus$placeholder"]);
    assert_eq!(
        classes,
        r#".flex\|hover,focus\$placeholder:hover:focus::placeholder{display:flex}"#
    );

    let classes = z.generate_classes(["mr[0.5rem]"]);
    assert_eq!(classes, r#".mr\[0\.5rem\]{margin-right:0.5rem}"#);
}

#[test]
fn generate_multiple_works() {
    let z = Zephyr::new();

    let classes_joined = z.generate_classes(["flex-row mt[1rem]"]);
    let classes_separate = z.generate_classes(["flex-row", "mt[1rem]"]);
    assert_eq!(
        classes_joined,
        r#".flex-row{display:flex;flex-direction:row}.mt\[1rem\]{margin-top:1rem}"#
    );
    assert_eq!(classes_separate, classes_joined);
}

#[test]
fn generate_specials_works() {
    let z = Zephyr::new();

    let classes = z.generate_classes(["mx[1rem]"]);
    assert_eq!(
        classes,
        r#".mx\[1rem\]{margin-left:1rem;margin-right:1rem}"#
    );
}

#[test]
fn generate_with_spaces_works() {
    let z = Zephyr::new();

    let classes = z.generate_classes(["border[1px_solid_black]"]);
    assert_eq!(
        classes,
        r#".border\[1px_solid_black\]{border:1px solid black}"#
    );
}

#[test]
fn generate_literals_works() {
    let z = Zephyr::new();

    // the curly brackets indicate that the value should not go through replacements
    let classes = z.generate_classes(["border{1px_solid_black}", "w{full}"]);
    assert_eq!(
        classes,
        r#".border\{1px_solid_black\}{border:1px_solid_black}.w\{full\}{width:full}"#
    );
}

#[test]
fn generate_with_media_query() {
    let z = Zephyr::new();

    let classes = z.generate_classes(["m[1rem]sm"]);
    assert_eq!(
        classes,
        r#"@media(min-width:640px){.m\[1rem\]sm{margin:1rem}}"#
    );

    let classes = z.generate_classes(["m[1rem]<md"]);
    assert_eq!(
        classes,
        r#"@media(max-width:767.9px){.m\[1rem\]\<md{margin:1rem}}"#
    );

    let classes = z.generate_classes(["m[1rem]motion-reduce"]);
    assert_eq!(
        classes,
        r#"@media(prefers-reduced-motion:reduce){.m\[1rem\]motion-reduce{margin:1rem}}"#
    );

    let classes = z.generate_classes(["m[1rem]@xl,motion-reduce"]);
    assert_eq!(
        classes,
        r#"@media(min-width:1280px)and(max-width:1535.9px)and(prefers-reduced-motion:reduce){.m\[1rem\]\@xl,motion-reduce{margin:1rem}}"#
    );
}

#[test]
fn generate_variable() {
    let z = Zephyr::new();

    // the parens indicate that it should be replaced by `var(--...)`
    let classes = z.generate_classes(["m(my-margin)"]);
    assert_eq!(classes, r#".m\(my-margin\){margin:var(--my-margin)}"#);
}

#[test]
fn generate_css_colors() {
    let z = Zephyr::new().with_css_colors();

    let classes = z.generate_classes(["white blanchedalmond"]);
    assert_eq!(
        classes,
        r#".white{color:white}.blanchedalmond{color:blanchedalmond}"#
    );
}

#[test]
fn generate_context_aware_value() {
    let z = Zephyr::new();

    let classes = z.generate_classes(["tt[u]"]);
    assert_eq!(classes, r#".tt\[u\]{text-transform:uppercase}"#);
}
