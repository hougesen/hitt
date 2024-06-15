# hitt

<a href="https://crates.io/crates/hitt"><img src="https://img.shields.io/crates/v/hitt.svg"></a>
<a href="https://github.com/hougesen/hitt/actions/workflows/validate.yml"><img src="https://github.com/hougesen/hitt/actions/workflows/validate.yml/badge.svg"></a>
<a href="https://codecov.io/gh/hougesen/hitt"><img src="https://codecov.io/gh/hougesen/hitt/branch/main/graph/badge.svg"/></a>

hitt is a command line HTTP testing tool focused on speed and simplicity.

## Install

hitt can be installed using Cargo.

```shell
cargo install hitt --locked
```

If you do not have Cargo installed, you need to [install it first](https://www.rust-lang.org/learn/get-started).

### Dependencies

`hitt` does not require any external dependencies for Windows and macOS user.

Linux users must install `openssl`.

#### Debian and Ubuntu

```shell
sudo apt-get install pkg-config libssl-dev
```

#### Arch Linux

```shell
sudo pacman -S pkg-config openssl
```

#### Fedora

```shell
sudo dnf install pkg-config perl-FindBin openssl-devel
```

#### Alpine Linux

```shell
apk add pkgconfig openssl-dev
```

#### openSUSE

```shell
sudo zypper in libopenssl-devel
```

## Usage

To send a request create a file ending in `.http`.

The syntax of `.http` files is pretty straightforward:

```http
GET https://mhouge.dk/
```

The file can then be run using the following command:

```shell
hitt run PATH_TO_FILE
```

That is all that is need to send a request.

### Arguments

| Argument                 | Description                    |
| ------------------------ | ------------------------------ |
| `--var <KEY>=<VALUE>`    | Variables to pass to request   |
| `--recursive`            | Run all files in directory     |
| `--fail-fast`            | Exit on status code 4XX or 5xx |
| `--hide-headers`         | Hide response headers          |
| `--hide-body`            | Hide response body             |
| `--timeout <TIMEOUT_MS>` | Request timeout in ms          |

### Request headers

Request headers can be added by writing key value pairs (`KEY:VALUE`) on a new line after the method and URL:

```http
GET https://mhouge.dk/
key:value
```

Leading spaces in the header value is ignored, so `KEY: VALUE` and `KEY:VALUE` will both have the value `VALUE`.

### Request body

A body can be sent with the request by creating a blank line, followed by the desired body input.

Please note, hitt **does not** infer content type. That has to be written as a header.

```http
POST https://mhouge.dk/
content-type:application/json

{
    "key": "value"
}
```

### Multiple request in single file

Multiple requests can be written in a single file by adding a line with `###` as a separator:

```http
GET https://mhouge.dk/

###

GET https://mhouge.dk/
```

### Variables

hitt has support for request variables.

A variable can be set in a file using the following syntax `@name = VALUE`. Whitespace is ignored.

Variables are used by wrapping the name in curly brackets (`{{ name }}`).

```http
@variable_name = localhost

GET {{ variable_name }}/api
```

In-file variables are not shared between other files.

#### Variable arguments

Variables can be passed to all requests using the `--var <KEY>=<VALUE>` argument:

```http
# file.http

GET {{ host }}/api
```

The file can the be run:

```shell
hitt run --var host=localhost:5000 file.http
```

### Server sent events (SSE)

A SSE listener can be started using the `hitt sse` command.

```shell
hitt sse https://sse.dev/test
```

### Shell completions

Shell completions can be generated using `mdsf completions <SHELL>`.

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(mdsf completions bash)"
```

#### Bash

Add the following to your `.zshrc`.

```bash
eval "$(mdsf completions zsh)"
```

#### Fish

Add the following to `~/.config/fish/config.fish`.

```fish
mdsf completions fish | source
```

#### PowerShell

Add the following to your PowerShell configuration (Can be found by running `$PROFILE`).

```powershell
Invoke-Expression (&mdsf completions powershell)
```

#### Elvish

Add the following to `~/.elvish/rc.elv`.

```elvish
eval (mdsf completions elvish)
```

## Neovim

hitt can be run directly from Neovim.

> [!NOTE]
> The `hitt` executable must be available in your path for the plugin to work.

### Install

#### Lazy

```lua
local hitt_plugin = {
    "hougesen/hitt",
    opts = {},
}
```

### Usage

The plugin exposes a single command `:HittSendRequest`, which can be bound to a keymap like this:

```lua
-- ~/.config/nvim/after/plugin/hitt.lua

local hitt = require("hitt")

vim.keymap.set("n", "<leader>rr", hitt.HittSendRequest, {})
```

![hitt neovim window](/docs/public/hitt-neovim-window.png)

### Configuration

| Name          | Default | Description                       |
| ------------- | ------- | --------------------------------- |
| window_width  | 80      | Window width in percentage        |
| window_height | 80      | Window height in percentage       |
| fail_fast     | false   | Enables the `--fail-fast` options |

## Disclaimer

hitt is most likely not ready for main stream usage. I ([Mads Hougesen](https://mhouge.dk)) am primarily developing it based on features I believe to be useful, or fun to develop.
