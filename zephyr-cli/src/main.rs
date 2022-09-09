use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use clap::Parser;
use color_eyre::eyre::Result;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use walkdir::{self, WalkDir};
use zephyr::{scraping::*, Zephyr};

/// generate css :)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// file or directory
    #[clap(value_parser)]
    path: PathBuf,
    /// output path. defaults to `zephyr.css`
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
    #[clap(short, long, value_parser)]
    watch: bool,
    /// use regex instead of an html parser to extract the classes
    #[clap(short, long, value_parser)]
    regex: bool,
    /// disables recursion into subdirectories
    #[clap(short, long, value_parser)]
    no_recurse: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let output = args.output.unwrap_or_else(|| PathBuf::from("zephyr.css"));
    // this makes it so we can call canonicalize
    std::fs::write(&output, "")?;
    let output_canonical = output.canonicalize()?;

    let z = Zephyr::new();

    run(&z, &args.path, &output, args.regex, args.no_recurse)?;
    println!("generated {}", output.as_os_str().to_string_lossy());

    if args.watch {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_secs(3), None, tx).unwrap();

        debouncer
            .watcher()
            .watch(
                &args.path,
                if args.no_recurse {
                    RecursiveMode::NonRecursive
                } else {
                    RecursiveMode::Recursive
                },
            )
            .unwrap();

        loop {
            if let Ok(Ok(e)) = rx.recv() {
                if e.into_iter().any(|e| e.path != output_canonical) {
                    run(&z, &args.path, &output, args.regex, args.no_recurse)?;
                    println!("generated {}", output.as_os_str().to_string_lossy());
                }
            }
        }
    }

    Ok(())
}

fn run(z: &Zephyr, source: &Path, output: &Path, regex: bool, no_recurse: bool) -> Result<()> {
    let mut files = vec![];
    if source.is_dir() {
        let mut w = WalkDir::new(source).follow_links(true);
        if no_recurse {
            w = w.max_depth(1);
        }
        for entry in w.into_iter().flatten() {
            if entry.path().is_file() {
                files.push(entry.into_path());
            }
        }
    } else {
        files.push(source.to_path_buf());
    }

    // TODO skip unneeded allocations
    // it currently turns stuff to strings and vecs cause lifetime stuff

    let classes = files
        .into_iter()
        .flat_map(std::fs::read_to_string)
        .flat_map(|f| {
            if regex {
                get_classes_regex(&f)
                    .into_iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            } else {
                get_classes(&f)
            }
        })
        .collect::<Vec<_>>();

    let css = z.generate_classes(classes.iter().map(String::as_str));

    std::fs::write(output, css)?;

    Ok(())
}
