#!/bin/bash

set -e -o pipefail

if [ ! -f pyproject.toml ]; then
    echo 'Run this script from python/ subdirectory' 1>&2
    exit 1
fi

source ./venv/bin/activate

set -x

rm -rf ./dist

python -m build --sdist
python -m twine upload dist/*
