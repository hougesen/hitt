# hitt

hitt is a command line HTTP testing tool focused on speed and simplicity.

## Usage

To send a request create a file ending in `.http`.

The syntax of `.http` files is pretty straightforward:

```text
GET https://mhouge.dk/
```

The file can then be run using the following command:

```sh
hitt <PATH_TO_FILE>
```

That is all that is need to send a request.

### Request headers

Request headers can be added by writing key value pairs (`KEY:VALUE`) on a new line after the method and URL:

```text
GET https://mhouge.dk/
key:value
```

Leading spaces in the header value is ignored, so `KEY: VALUE` and `KEY:VALUE` will both have the value `VALUE`.

### Request body

A body can be sent with the request by creating a blank line, followed by the desired body input.

Please note, hitt **does not** infer content type. That has to be written as a header.

```text
POST https://mhouge.dk/
content-type:application/json

{
    "key": "value"
}
```

### Multiple request in single file

Multiple requests can be written in a single file by adding a line with `###` as a separator:

```text
GET https://mhouge.dk/

###

GET https://mhouge.dk/
```

### Exiting on 4XX and 5XX status codes

By default, hitt does not exit on error status codes. That behavior can be changed by supplying the `--fail-fast` argument.

```sh
hitt --fail-fast <PATH_TO_FOLDER>
```

### Running all files in directory

The `--recursive` argument can be passed to run all files in a directory:

```sh
hitt --recursive <PATH_TO_FOLDER>
```

The order of each file execution is platform and file system dependent. That might change in the future, but for now you **should not** rely on the order.

### Hiding response headers

The `--hide-headers` argument can be passed to hide the response headers in the output:

```sh
hitt --hide-headers <PATH_TO_FILE>
```

### Hiding response body

The `--hide-body` argument can be passed to hide the response body in the output:

```sh
hitt --hide-body <PATH_TO_FILE>
```

### Disabling pretty printing

The `--disable-formatting` argument can be passed to disable pretty printing of response body:

```sh
hitt --disable-formatting <PATH_TO_FILE>
```

## Disclaimer

hitt is most likely not ready for main stream usage. I ([Mads Hougesen](https://mhouge.dk)) am primarily developing it based on features I believe to be useful, or fun to develop.
