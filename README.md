# binchotan-frontend-sample

An example frontend implementation for [binchotan](http://github.com/sei
0o/binchotan-backend). Works on command line and provides with simple communication between backends.

## Installation

Make sure Rust and its toolchain (e.g. Cargo) are installed.

1. Clone this repository.
2. `$ cargo run -- --help`

## Usage

Check if the backend is working:

```bash
$ cargo r -- status
sending status request
{"id":"0e9f9f85-1bd3-4d79-9d91-b905aad95fbb","jsonrpc":"2.0","method":"v0.status"}
{
  "id": "0e9f9f85-1bd3-4d79-9d91-b905aad95fbb",
  "jsonrpc": "2.0",
  "result": {
    "version": "0.1.0"
  }
}
```

Add an account to the backend:

```bash
$ cargo r -- account-add
sending accounts add request
{"id":"30a93ef6-a971-43ca-bb68-3fa0a9be1c83","jsonrpc":"2.0","method":"v0.account.add"}
{
  "id": "30a93ef6-a971-43ca-bb68-3fa0a9be1c83",
  "jsonrpc": "2.0",
  "result": {
    "user_id": "(your twitter user id)"
  }
}
```

List all the accounts you have:

```bash
$ cargo r -- account-list
sending accounts list request
{"id":"eb666fda-3968-404c-a054-98a1a6508e06","jsonrpc":"2.0","method":"v0.account.list"}
{
  "id": "eb666fda-3968-404c-a054-98a1a6508e06",
  "jsonrpc": "2.0",
  "result": {
    "user_ids": [
      "(some twitter user id)",
      "(another twitter user id)",
      "(yet another twitter user id)"
    ]
  }
}
```

Show your home timeline:

```bash
$ cargo r -- home-timeline '{"user_id":"(your Twitter user id)", "api_params": {}}'
sending home_timeline request
{"id":"34bbf73b-f225-49b3-8d5b-d001f4c5a7a1","jsonrpc":"2.0","method":"v0.home_timeline","params":{"api_params":{},"user_id":"(your Twitter user id)"}}
{
  "id": "34bbf73b-f225-49b3-8d5b-d001f4c5a7a1",
  "jsonrpc": "2.0",
  "result": {
    "body": [
      {
        "author_id": "123456...",
        "conversation_id": "123456...",
        "created_at": "2022-08-30T11:12:11.000Z",
        "id": "123456...",
        "lang": "ja",
        "possibly_sensitive": false,
        "public_metrics": {
...
```
