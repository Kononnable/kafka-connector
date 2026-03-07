mod broker;
pub mod clients;
pub mod cluster;
pub mod protocol_consts;
#[cfg(test)]
pub(crate) mod test_utils;

// TODO: before official release
// define proper visibility on modules (pub)
// don't allow protocol structs to be part of the api
// go through code and do proper doc comments (https://doc.rust-lang.org/rustdoc/)
// require doc comments for public api (#![warn(missing_docs)])
// go through public api and check if it's in form ready for semver maintenance(#[non_exhaustive] enums, https://rust-lang.github.io/api-guidelines/)
// automated tests for matrix of kafka versions(+redpanda? - what about feature mismatch)
// go through cluster/broker/client options and check if there are no missing major features for 1.0
// go through protocol todos - check if there are no critical ones
// cleanup protocol code (not required for release)
// reserve and change crate names
// readme
// examples
// check tracing - if it is where needed, if it doesn't log that should not be logged (e.g. channels, mutexes are ok if underneath data is valuable)
