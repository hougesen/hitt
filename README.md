# hitt

<a href="https://crates.io/crates/hitt"><img src="https://img.shields.io/crates/v/hitt.svg"></a>
<a href="https://github.com/hougesen/hitt/actions/workflows/validate.yml"><img src="https://github.com/hougesen/hitt/actions/workflows/validate.yml/badge.svg"></a>
<a href="https://codecov.io/gh/hougesen/hitt"><img src="https://codecov.io/gh/hougesen/hitt/branch/main/graph/badge.svg"/></a>

hitt is a command line HTTP testing tool focused on speed and simplicity.

![hitt example](/docs/images/hitt-cli-example.png)

<!-- START_SECTION:base-command-help -->

```
hitt 0.0.18
command line HTTP testing tool focused on speed and simplicity
Mads Hougesen <mads@mhouge.dk>

Usage: hitt <COMMAND>

Commands:
  run          Send http requests
  sse          Listen to sse events
  completions  Generate shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:base-command-help -->

## Install

### Linux & MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/hougesen/hitt/releases/latest/download/hitt-installer.sh | sh
```

### Windows

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/hougesen/hitt/releases/latest/download/hitt-installer.ps1 | iex"
```

### Cargo

hitt can be installed using Cargo.

```shell
cargo install hitt --locked
```

If you do not have Cargo installed, you need to [install it first](https://www.rust-lang.org/learn/get-started).

### npm/npx

You can install hitt using [npm](https://www.npmjs.com/package/hitt-cli):

```shell
npm install -g hitt-cli

hitt run hello-world.http
```

or run it directly using npx:

```shell
npx hitt-cli run hello-world.http
```

### Homebrew

```shell
brew install hougesen/tap/hitt
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

<!-- START_SECTION:run-command-help -->

```
Send http requests

Usage: hitt run [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to .http file, or directory if supplied with the `--recursive` argument

Options:
      --timeout <TIMEOUT_MS>  Request timeout in milliseconds
      --var <KEY>=<VALUE>     Variables to pass to request
  -r, --recursive             Enable to run directory recursively
      --fail-fast             Exit on error response status code
      --hide-body             Whether or not to show response body
      --hide-headers          Whether or not to show response headers
      --disable-formatting    Disable pretty printing of response body
  -h, --help                  Print help
  -V, --version               Print version

```

<!-- END_SECTION:run-command-help -->

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

<!-- START_SECTION:sse-command-help -->

```
Listen to sse events

Usage: hitt sse <URL>

Arguments:
  <URL>

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:sse-command-help -->

### Shell completions

Shell completions can be generated using `hitt completions <SHELL>`.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completions

Usage: hitt completions <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, nushell, powershell, zsh]

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:completions-command-help -->

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(hitt completions bash)"
```

#### Bash

Add the following to your `.zshrc`.

```bash
eval "$(hitt completions zsh)"
```

#### Fish

Add the following to `~/.config/fish/config.fish`.

```fish
hitt completions fish | source
```

#### PowerShell

Add the following to your PowerShell configuration (Can be found by running `$PROFILE`).

```powershell
Invoke-Expression (&hitt completions powershell)
```

#### Elvish

Add the following to `~/.elvish/rc.elv`.

```elvish
eval (hitt completions elvish)
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

![hitt neovim window](/docs/images/hitt-neovim-example.png)

### Configuration

| Name          | Default | Description                       |
| ------------- | ------- | --------------------------------- |
| window_width  | 80      | Window width in percentage        |
| window_height | 80      | Window height in percentage       |
| fail_fast     | false   | Enables the `--fail-fast` options |

#### HTTP syntax highlighting

Syntax highlighting can be enabled by installing the `http` treesitter parser (`:TSInstall http`) and adding a file association for `.http` files.

```lua
vim.filetype.add({
	extension = {
		http = "http",
	},
})
```

## Disclaimer

hitt is most likely not ready for main stream usage. I ([Mads Hougesen](https://mhouge.dk)) am primarily developing it based on features I believe to be useful, or fun to develop.
