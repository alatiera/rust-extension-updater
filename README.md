# Rustc releases sha

Given a version of rustc, print the sha of the source tarball and the precompiled
binaries for the following architectures

# How to use it

```sh
$ cargo run -- 1.29.1
source tarball: f1b0728b66ce6bce6d72bbe5ea9e3a24ea22a045665da2ed8fcdfad14f61a349
x86_64: b36998aea6d58525f25d89f1813b6bfd4cad6ff467e27bd11e761a20dde43745
i686: 05e2880beca45e7319074d2268fd79a70c7aade2fb14dbcbf39585b5560f2048
armv7hf: 2685224f67b2ef951e0e8b48829f786cbfed95e19448ba292ac33af719843dbe
aarch64: 2cae2ecc366914707d6b753a96505c727df69df8bcbc1f8d14fbd66fca005239
```
