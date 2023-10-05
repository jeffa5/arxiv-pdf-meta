use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Serialize)]
struct Meta {
    yearmonth: String,
    filename: String,
    title: String,
    author: String,
    keywords: String,
}

fn main() -> anyhow::Result<()> {
    let pdf_dir = "arxiv-pdfs";
    let month_years = fs::read_dir(&pdf_dir)?;

    // bad paths
    let ignored_paths = [
        PathBuf::from("arxiv-pdfs/2001/2001.07824v1.pdf"),
        PathBuf::from("arxiv-pdfs/2001/2001.07824v2.pdf"),
        PathBuf::from("arxiv-pdfs/2001/2001.07824v3.pdf"),
        PathBuf::from("arxiv-pdfs/2001/2001.07824v4.pdf"),
    ];

    for month_year in month_years {
        let month_year = month_year?;
        process_month_year(&month_year.path(), &ignored_paths)?;
    }

    Ok(())
}

fn process_month_year(path: &Path, ignored_paths: &[PathBuf]) -> anyhow::Result<()> {
    println!("Processing month year {:?}", path);

    let results_path = path.join("data.csv");
    if results_path.exists() {
        println!("Skipping {:?} as data file exists", path);
        return Ok(());
    }

    let mut data_writer = csv::Writer::from_path(&results_path)?;

    let mut i = 0;
    let interval = 100;

    let pdf_files = fs::read_dir(path)?;
    for pdf_file in pdf_files {
        let path = pdf_file?.path();
        i += 1;
        if i % interval == 0 {
            println!("{:?}", path);
        }
        if path.is_file()
            && path.extension().and_then(|s| s.to_str()) == Some("pdf")
            && !ignored_paths.contains(&path)
        {
            let file_options = pdf::file::FileOptions::uncached();
            match file_options.open(&path) {
                Ok(pdf_file) => {
                    if let Some(dict) = pdf_file.trailer.info_dict {
                        let title = dict
                            .title
                            .and_then(|s| s.to_string().ok())
                            .unwrap_or_default();
                        let author = dict
                            .author
                            .and_then(|s| s.to_string().ok())
                            .unwrap_or_default();
                        let keywords = dict
                            .keywords
                            .and_then(|s| s.to_string().ok())
                            .unwrap_or_default();

                        let filename = path.file_name().unwrap().to_string_lossy().into_owned();
                        let yearmonth = path
                            .parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned();

                        data_writer.serialize(Meta {
                            yearmonth,
                            filename,
                            title,
                            author,
                            keywords,
                        })?;
                    }
                }
                Err(error) => {
                    println!("Failed to open file {:?}: {:?}", path, error);
                }
            }
        } else {
            println!("warning: path was not a pdf file: {:?}", path);
        }
    }
    Ok(())
}
