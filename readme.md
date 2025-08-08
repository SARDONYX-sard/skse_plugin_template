# Rust SKSE Plugin template

<div align="center">
  <!-- Release & Build Badges -->
  <p>
    <a href="https://github.com/SARDONYX-sard/skse_plugin_template/releases">
      <img src="https://img.shields.io/github/downloads/SARDONYX-sard/skse_plugin_template/total?style=flat-square" alt="Total Downloads">
    </a>
    <a href="https://github.com/SARDONYX-sard/skse_plugin_template/actions/workflows/release-cli.yaml">
      <img src="https://github.com/SARDONYX-sard/skse_plugin_template/actions/workflows/release.yaml/badge.svg?style=flat-square" alt="Release">
    </a>
    <a href="https://github.com/SARDONYX-sard/skse_plugin_template/actions/workflows/build-cli.yaml">
      <img src="https://github.com/SARDONYX-sard/skse_plugin_template/actions/workflows/build.yaml/badge.svg?style=flat-square" alt="Build">
    </a>
    <a href="https://github.com/SARDONYX-sard/skse_plugin_template/actions/workflows/test.yaml">
      <img src="https://github.com/SARDONYX-sard/skse_plugin_template/actions/workflows/test.yaml/badge.svg?style=flat-square" alt="Test (Cargo)">
    </a>
  </p>

  <!-- License & Meta Badges -->
  <p>
    <a href="https://github.com/SARDONYX-sard/skse_plugin_template/issues">
      <img src="https://img.shields.io/github/issues/SARDONYX-sard/skse_plugin_template?style=flat-square" alt="Open Issues">
    </a>
    <a href="https://github.com/SARDONYX-sard/skse_plugin_template/pulls">
      <img src="https://img.shields.io/github/issues-pr/SARDONYX-sard/skse_plugin_template?style=flat-square" alt="Open PRs">
    </a>
    <!-- <a href="https://opensource.org/licenses/MIT">
      <img src="https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square" alt="License: MIT">
    </a>
    <a href="https://opensource.org/licenses/Apache-2.0">
      <img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=flat-square" alt="License: Apache 2.0">
    </a> -->
  </p>
</div>

Search for `FIXME:` and change the necessary items.

# How to build?

Execute the following command in PowerShell to create a dll in `./build/mods/`.

```shell
cargo xtask build --dest_mode root
```

## License

There is no license specified for this template.
It is okay to delete this item, but we recommend MIT OR APACHE2.0. This is because the license terms are clear when there is a documented license.

If you really want to include the original license for this template, please paste the following in an easy-to-find place.(This is not necessary in most cases.)

```md
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT
```
