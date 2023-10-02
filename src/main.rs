use pdf::primitive::Primitive;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let pdf_dir = "arxiv-pdfs";
    let month_years = fs::read_dir(&pdf_dir)?;
    let mut files = Vec::new();
    for month_year in month_years {
        let month_year = month_year?;
        let pdf_files = fs::read_dir(month_year.path())?;
        for pdf_file in pdf_files {
            let path = pdf_file?.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                files.push(path);
            } else {
                println!("warning: path was not a pdf file: {:?}", path);
            }
        }
    }

    files.sort();

    let mut names = BTreeMap::<String, usize>::new();

    struct Meta {
        path: String,
        title: String,
        author: String,
        keywords: String,
    }

    #[derive(Debug)]
    struct Data {
        success: usize,
        errors: usize,
        title_length: BTreeMap<usize, usize>,
        author_length: BTreeMap<usize, usize>,
        keywords_length: BTreeMap<usize, usize>,
        author_separators: BTreeMap<char, usize>,
        keywords_separators: BTreeMap<char, usize>,
    }

    let mut data = Data {
        success: 0,
        errors: 0,
        title_length: BTreeMap::default(),
        author_length: BTreeMap::default(),
        keywords_length: BTreeMap::default(),
        author_separators: BTreeMap::default(),
        keywords_separators: BTreeMap::default(),
    };

    let mut metas = Vec::new();

    for file in &files {
        let file_options = pdf::file::FileOptions::uncached();
        if let Ok(pdf_file) = file_options.open(&file) {
            data.success += 1;
            if let Some(dict) = pdf_file.trailer.info_dict {
                let title = dict
                    .get("Title")
                    .map(string_or_print_type)
                    .unwrap_or_default();
                *data.title_length.entry(title.len()).or_default() += 1;
                let author = dict
                    .get("Author")
                    .map(string_or_print_type)
                    .unwrap_or_default();
                *data.author_length.entry(author.len()).or_default() += 1;
                let keywords = dict
                    .get("Keyword")
                    .map(string_or_print_type)
                    .unwrap_or_default();
                *data.keywords_length.entry(keywords.len()).or_default() += 1;

                for char in author.chars() {
                    if char.is_ascii_punctuation() {
                        *data.author_separators.entry(char).or_default() += 1;
                    }
                }
                for char in keywords.chars() {
                    if char.is_ascii_punctuation() {
                        *data.keywords_separators.entry(char).or_default() += 1;
                    }
                }

                metas.push(Meta {
                    path: file.to_string_lossy().into_owned(),
                    title,
                    author,
                    keywords,
                });
                for (name, _primitive) in dict {
                    *names.entry(name.as_str().to_owned()).or_default() += 1;
                }
            }
        } else {
            data.errors += 1;
            println!("Failed to open {:?}", file);
        }
    }

    println!("{:#?}", names);
    println!("{:#?}", data);
    println!("files: {}", files.len(),);

    let mut results_file = File::create("results.csv")?;
    writeln!(results_file, "yearmonth,filename,Title,Author,Keywords")?;

    for meta in metas {
        let path = meta.path.strip_prefix(&format!("{}/", pdf_dir)).unwrap();
        let (yearmonth, filename) = path.split_once("/").unwrap();
        writeln!(
            results_file,
            "{},{},{:?},{:?},{:?}",
            yearmonth, filename, meta.title, meta.author, meta.keywords
        )?;
    }

    Ok(())
}

fn primitive_type(prim: &Primitive) -> String {
    match prim {
        Primitive::Null => "null",
        Primitive::Integer(_) => "integer",
        Primitive::Number(_) => "number",
        Primitive::Boolean(_) => "bool",
        Primitive::String(_) => "string",
        Primitive::Stream(_) => "stream",
        Primitive::Dictionary(_) => "dictionary",
        Primitive::Array(_) => "array",
        Primitive::Reference(_) => "reference",
        Primitive::Name(_) => "name",
    }
    .to_owned()
}

fn string_or_print_type(prim: &Primitive) -> String {
    if prim.as_string().is_err() {
        println!("found a {}", primitive_type(prim));
    }
    prim.to_string().unwrap()
}
