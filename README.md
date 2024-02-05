<p align="center">
<a href="https://Giton-App.github.io"><img src="https://raw.githubusercontent.com/Giton-App/giton-cli/main/docs/logo.svg" height="100px"/></a>
 </p>
<h1 align="center">giton</h1>
<div align="center">
 <strong>
    AI augmentation for the GIT commandline
 </strong>
</div>
<br/>

[![Build](https://github.com/Giton-App/giton-cli/actions/workflows/build.yml/badge.svg)](https://github.com/Giton-App/giton-cli/actions/workflows/build.yml)
[![Tests](https://github.com/Giton-App/giton-cli/actions/workflows/test.yml/badge.svg)](https://github.com/Giton-App/giton-cli/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/giton.svg)](https://crates.io/crates/giton)
[![MIT License](https://img.shields.io/github/license/Giton-App/giton-cli)](https://github.com/Giton-App/giton-cli/LICENSE)

`giton` is an AI augmentation program for `git`. It uses [OpenAI GPT-4](https://openai.com/gpt-4), and your local git context to make command suggestions. `giton` is a passthrough for `git` commands, so you can already use it as a replacement for git.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [Installation](#installation)
  - [Pre-Built Binaries](#pre-built-binaries)
  - [Compilation with Cargo](#compilation-with-cargo)
- [Configuration](#configuration)
- [Commands](#commands)
  - [history](#history)
  - [helpme](#helpme)
  - [undo](#undo)
- [GPT-4 Costs](#gpt-4-costs)
- [Vulnerability reporting](#vulnerability-reporting)
- [License](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

### Installation

> You need [git](https://git-scm.com) installed and accessible from your commandline.

#### Pre-Built Binaries

You can download the relevant binary from the [latest release](https://github.com/Giton-App/giton-cli/releases). Linux, macOS and Windows binaries are available for x86_64.

#### Compilation with Cargo

If you have the rust toolchain installed, you can install `giton` with `cargo`

```sh
cargo install giton
```

### Configuration

You need an [OpenAI API Key](https://help.openai.com/en/articles/4936850-where-do-i-find-my-api-key) to be able to use `giton`. After creating an account with OpenAI, you can generate an API key. Then, you need to set this API key in your shell environment.

For Linux/macOS

```sh
export GITON_OPENAI_KEY=your_api_key
```

For Windows/Powershell

```powershell
$env:GITON_OPENAI_KEY = "your_api_key"
```

### Commands

#### history

`giton` stores your commands history in the file `.giton` in the root of your directory. It is recommended to track this file as it improves the accuracy of the `undo` and `helpme` commands. 

Usage

```sh
giton history
```

Output

```
2023-12-13 20:01:45: status
2023-12-20 00:56:03: add .
2023-12-20 00:56:20: commit -m imp(display): add spinners
```

#### helpme

`helpme` uses your current repository context to suggest possible commands.

Usage

```
giton helpme
```

Output

```
⢿ Communicating with Open AI
+---+----------------------------------------------+
| # | Command                                      |
+==================================================+
| 0 | git add README.md                            |
|---+----------------------------------------------|
| 1 | git commit -m "chore(readme): update README" |
+---+----------------------------------------------+

:: Prooced with Command(s)?: [Y/n]
```

#### undo

`undo` uses `giton` stored history to suggest a command (or set of commands) that cancels out your previous command.

Usage

```
giton undo
```

Output

```
⣷ Communicating with Open AI
+---+------------------------------------------+
| # | Command                                  |
+==============================================+
| 0 | git restore --staged core/src/command.rs |
+---+------------------------------------------+

:: Prooced with Command(s)?: [Y/n]
```

### GPT-4 Costs

GPT-4 is not free and can actually be pricey. Only the `helpme` and `undo` commands make calls to GPT-4; all other commands (including `git` passthrough commands) do not make calls to GPT-4.

It is hard to estimate the cost of GPT-4 calls. I recommend you frequently check the usage tab in the Open AI dashboard to get an idea of the cost; and adjust your consumption accordingly.

### Vulnerability reporting

For reporting a security vulnerability, you can directly [send me an email](mailto:giton@omarabid.com).

### License

giton is distributed under the MIT License.

See [LICENSE](LICENSE) for details.
