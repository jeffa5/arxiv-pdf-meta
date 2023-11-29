# arXiv PDF Metadata analysis

A small effort to quantify how many papers on arXiv just don't bother with metadata.

## Running

This project is just a small rust binary that will print out some info and save more comprehensive data to a `results.csv`:

```sh
cargo run --release
```

You can then generate some plots with Julia, running:
```sh
julia --project=.
> include("analysis.jl")
> main()
```

## Datasets

The notebook does not currently download the datasets itself, instead it expects them to be in a certain location.

```sh
# list available pdf groups
gsutil ls gs://arxiv-dataset/arxiv/arxiv/pdf

# make the data dir
mkdir -p arxiv-pdfs

# then, download one set
gsutil -m cp -r gs://arxiv-dataset/arxiv/arxiv/pdf/<yearmonth> ./arxiv-pdfs
```
