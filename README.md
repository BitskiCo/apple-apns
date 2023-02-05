# Rust client for Apple Push Notification service

![License](https://img.shields.io/crates/l/apple-apns)
[![Crates.io](https://img.shields.io/crates/v/apple-apns)][crates-io]

See [Setting Up a Remote Notification Server][setting_up_a_remote_notification_server]
for the official Apple developer documentation.

[setting_up_a_remote_notification_server]: https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server

## Use with HTTP mocks

By default, `apple-apns` only makes HTTP2 connections. To allow HTTP1
connections for compatibility with HTTP mock libraries that don't support HTTP2,
enable the `http1` feature.

[crates-io]: https://crates.io/crates/apple-apns
