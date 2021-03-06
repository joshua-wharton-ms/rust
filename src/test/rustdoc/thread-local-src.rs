#![crate_name = "foo"]

// @has foo/index.html '//a[@href="../src/foo/thread-local-src.rs.html#1-6"]' '[src]'

// @has foo/constant.FOO.html '//a/@href' 'https://doc.rust-lang.org/nightly/src/std/'
thread_local!(pub static FOO: bool = false);
