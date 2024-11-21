# `atprotolib-rs` [![GitHub build status](https://github.com/Smalls1652/atprotolib-rs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/Smalls1652/atprotolib-rs/actions/workflows/build.yml) [![Forgejo build status](https://git.smalls.online/smalls/atprotolib-rs/badges/workflows/build.yml/badge.svg?branch=main)](https://git.smalls.online/smalls/atprotolib-rs/actions?workflow=build.yml) [![MIT license](https://badgen.net/static/License/MIT/blue)](./LICENSE)

> [!WARNING]
> This is a work in progress and is not yet ready for use.

A Rust library for various [ATProtocol](https://atproto.com/) (Used by [BlueSky](https://bsky.social/about)) API/Lexicon types.

## 📋 API/Lexicon types implemented

> [!NOTE]
> Even if a specific namespace is marked as implemented, there might still be some missing types.

### `app.bsky`

- [x] `app.bsky.actor`
- [x] `app.bsky.embed`
- [x] `app.bsky.feed`
- [x] `app.bsky.graph`
- [x] `app.bsky.labeler`
- [x] `app.bsky.notification`
- [x] `app.bsky.richtext`
- [ ] `app.bsky.unspecced`
- [x] `app.bsky.video`

### `chat.bsky`

- [ ] `chat.bsky.actor`
- [ ] `chat.bsky.convo`
- [ ] `chat.bsky.moderation`

### `com.atproto`

- [x] `com.atproto.admin` _(Partial)_
- [x] `com.atproto.identity`
- [x] `com.atproto.label`
- [x] `com.atproto.moderation`
- [x] `com.atproto.repo`
- [x] `com.atproto.server` _(Partial)_
- [x] `com.atproto.sync`
- [ ] `com.atproto.temp`

### `tools.ozone`

- [ ] `tools.ozone.communication`
- [ ] `tools.ozone.moderation`
- [ ] `tools.ozone.server`
- [ ] `tools.ozone.set`
- [ ] `tools.ozone.setting`
- [ ] `tools.ozone.signature`
- [ ] `tools.ozone.team`

## 🗂️ Dependencies used

- [`chrono`](https://crates.io/crates/chrono)
- [`serde`](https://crates.io/crates/serde)
- [`serde_json`](https://crates.io/crates/serde_json)

## 🤝 License

The source code for this project is licensed with the [MIT License](LICENSE).
