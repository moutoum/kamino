# Kamino

Kamino is a HTTP server mock engine.
It provides easy CLI to mock HTTP responses from a server starting from the status.

## Usage

Simplest usage is to run without argument. The server answers `200 OK`
to every HTTP request.

```
> kamino status
200
200
200
...
```

### Status

Configuring multiple codes. The server iterates over the statuses in the
given order and returns the codes one after the others. Duplicating codes
is possible to create "weight" on a value.

```
> kamino status 200 404 500
200
404
500
200
...
```

`--wait`, `-w` Wait some time before sending back the response.

```
> kamino status -w 5s
waiting 5 seconds...
200
...
```