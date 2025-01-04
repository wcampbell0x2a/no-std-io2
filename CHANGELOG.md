# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.9.0] - 2025-01-04
- Support io errors with arbitrary payloads in "alloc" mode [#11](https://github.com/wcampbell0x2a/no-std-io2/pull/11)
- Implement Write for `Cursor<Vec<u8>>` [#10](https://github.com/wcampbell0x2a/no-std-io2/pull/10)
- Restore `io::ErrorKind::Unsupported`, add `io::ErrorKind::OutOfMemory` [#8](https://github.com/wcampbell0x2a/no-std-io2/pull/8)
- Use `core::error`, bump MSRV to `1.81` [#9](https://github.com/wcampbell0x2a/no-std-io2/pull/9)

## [v0.8.1] - 2024-10-12
- Fix `Write` impl for `alloc::Vec`

## [v0.8.0] - 2024-09-02
- Initial Release after fork
