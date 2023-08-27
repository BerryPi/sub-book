use std::{fmt::Write, path::{PathBuf}, fs::{self, File}};

use clap::Parser;
use epub_builder::{EpubBuilder, ZipLibrary, EpubContent, ReferenceType};
use srtlib::Subtitles;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    in_dir: String,

    #[arg(short, long, default_value_t = String::from("SubBook.epub"))]
    out_file: String,

    #[arg(short, long)]
    title: String
}

fn get_srts_from_source_dir(dir: &String) -> Vec<PathBuf> {
    let mut file_vec: Vec<PathBuf> = Vec::new();

    let sub_dir = fs::read_dir(dir)
        .expect("Failed to open source directory.")
        .flatten();

    for entry in sub_dir {
        if let Some(ext) = entry.path().extension(){
            if ext == "srt" {
                file_vec.push(entry.path());
            }
        }
    }

    return file_vec;
}

fn main() {
    let args = Args::parse();

    let sub_files = get_srts_from_source_dir(&args.in_dir);

    let mut out_file = File::create(args.out_file).expect("Failed to create output file.");

    let mut builder = EpubBuilder::new(ZipLibrary::new().expect("Failed to init zip library")).expect("Failed to init builder.");
    builder.metadata("title", args.title)
    .expect("Failed to set title")
    .stylesheet(FORMATTING.as_bytes())
    .expect("Failed to set formatting");

    for file in sub_files {
        let name = file.file_name().expect("Failed to extract filename.")
            .to_str().expect("Failed to extract filename.")
            .to_owned();

        let subs = Subtitles::parse_from_file(file, None).expect("Failed to parse subtitle file.");

        let chapter_filename = name.clone() + ".xhtml";
        let mut chapter_contents = CSS_REF.to_string();
        for s in subs {
            write!(&mut chapter_contents, "<p>{}</p>", s.text).expect("Failed to extract subtitles.");
        }

        builder.add_content(EpubContent::new(chapter_filename, chapter_contents.as_bytes())
            .title(name)
            .reftype(ReferenceType::Text))
        .expect("Failed to form EPUB content.");
    }

    builder.inline_toc()
    .generate(&mut out_file)
    .expect("Failed to write output file.");
}

const FORMATTING: &str =
"p {
    margin: 0.5cm 1cm;
}";

const CSS_REF: &str =
r#"<head>
  <link rel="stylesheet" type="text/css" href="stylesheet.css" />
</head>"#;
