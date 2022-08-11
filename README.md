rust-mkl container
===================

Container for building Rust crate with Intel MKL,
and helping to develop [intel-mkl-src](https://github.com/rust-math/intel-mkl-src) crate.

Usage
-----

- With docker:

  ```
  docker pull ghcr.io/rust-math/rust-mkl:1.62.1-2020.1
  ```

- In GitHub Actions:
  ```
  runs-on: ubuntu-latest
  container:
    image: ghcr.io/rust-math/rust-mkl:1.62.1-2020.1
  ```
  See [`jobs.<job_id>.container`](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#jobsjob_idcontainer) section of the workflow syntax.

⚠️ Notes
--------

- You should not use `latest` tag
  - I strongly encourage you not to use `latest` tag for any image and in any situation.
- This is designed to use for development, i.e. build and test of crates.
  Do not use as a base of production container.

License
--------
This container consists of many binaries.
Most of them are from [rust official image](https://hub.docker.com/_/rust/),
which is based on [Debian](https://www.debian.org/).
Please see the license of each application by searching on [debian index](https://packages.debian.org/index).

The Intel MKL library is re-distributed under the [Intel Simplified Software License for Intel(R) Math Kernel Library](https://www.intel.com/content/www/us/en/developer/articles/license/end-user-license-agreement.html).
