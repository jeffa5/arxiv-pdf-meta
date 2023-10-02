# arXiv PDF Metadata analysis

A small effort to quantify how many papers on arXiv just don't bother with metadata.

## Running

This is a project using Julia and Pluto notebooks.
To run the notebook you can launch the nix `devShell` and do the following:

```sh
# launch julia, install Pluto and run it locally
julia --project=. -e 'import Pkg; Pkg.add("Pluto"); import Pluto; Pluto.run()'
```

If you need to run the notebook and access it remotely then change the `run` command to be of the form:

```julia
# ip address, port
Pluto.run("0.0.0.0", 1234)
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
