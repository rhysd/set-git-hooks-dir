`set-git-hooks-dir` pip package
===============================
[![pypi][pypi-badge]][pypi]

## Usage

See https://github.com/rhysd/set-git-hooks-dir#readme for the usage.

## Limitation

The implementation of the package is a bit hacky to run `git` command within your repository on installing this package.

On `pip install` the source package is downloaded and built in a temporary directory outside your repository. So this
package somehow tries to find the location of your Git repository based on `site-packages` directory. This means that
the directory must exist in your repository. When it exists outside your repository because of making a Python virtual
environment outside your repository, this package can no longer know the location of your repository and cannot
configure the Git hooks.

## License

This npm package is distributed under [the MIT license](LICENSE).

[pypi-badge]: https://img.shields.io/pypi/v/set-git-hooks-dir
[pypi]: https://pypi.org/project/set-git-hooks-dir/
