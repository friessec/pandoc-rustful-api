# pandoc-rustful-api
[![Rust](https://github.com/friessec/pandoc-rustful-api/actions/workflows/rust.yml/badge.svg)](https://github.com/friessec/pandoc-rustful-api/actions/workflows/rust.yml)

Pandoc RESTful API written in Rust


## Build and Run

Rust API is work in progress, but container can be used locally

```
docker build -t pandoc-rustful-api .     
```

Start with
```
docker run --rm -u $(id -u):$(id -g) -v $(pwd):/home/webapp/pandoc pandoc-rustful-api doc.md -o doc.pdf \                                                                                                                                                                [130]
    --from markdown+yaml_metadata_block+raw_html \
    --template eisvogel \
    --table-of-contents \
    --highlight-style breezedark \
    --toc-depth 5 \
    --number-sections \
    --top-level-division=chapter
```
