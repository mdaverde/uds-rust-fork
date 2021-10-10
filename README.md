# uds-rust-fork

Working on a [custom Rust toolchain](https://github.com/mdaverde/rust/tree/uds-abstract) to test out UDS abstract namespaces on linux

As a test client to use against examples you can use [socat](https://www.redhat.com/sysadmin/getting-started-socat):

```sh
$ echo "hello world" | socat ABSTRACT-CONNECT:namespace_goes_here -
```
