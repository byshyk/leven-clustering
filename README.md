# Levenshtein + kodama + pyo3

## Building and Testing

To build this package, first install `maturin`:

```shell
pip install maturin
```

To build and test use `maturin develop`:

```shell
pip install -r requirements-dev.txt
maturin develop && pytest
```

Alternatively, install tox and run the tests inside an isolated environment:

```shell
tox -e py
```

## Build optimized wheels (?)

The following command build multiple wheels (cp36-cp39), should be run from project root:

```shell
docker run --rm -v $(pwd):/io konstin2/maturin build --release --no-sdist --manylinux=2014
```

## TODO

- [ ] fix all code TODOs
- [x] write .gitignore
- [x] write/update build scripts
- [ ] rewrite readme (especially `Motivation` block)
- [ ] add more tests
- [ ] export base levenshtein algorithm and optimize it
- [ ] fix actions for manylinux
- [ ] add description to pypi

## Motivation

Python version of this code was too slow (Python-levenshtein + scikit-learn), so I just implemented it using rust lang.
