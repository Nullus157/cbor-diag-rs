# cbor-diag-cli

A diagnostic tool for working with [Concise Binary Object Representation
(CBOR)](https://cbor.io). This tool can parse binary, hex, and [diagnostic
notation][] representations of CBOR items; then output them as binary, hex (with
or without annotations), and diagnostic notation (compact or "pretty").

## Installation

Currently `cbor-diag-cli` is only distributed through crates.io, [install
Rust][] then install with:

```console
> cargo install cbor-diag-cli
Updating crates.io index
[...]

> cbor-diag --help
cbor-diag 0.1.0
A utility for converting between binary, diagnostic, hex and annotated hex formats for CBOR.
[...]
```

## Examples

### Parsing a hex-encoded payload into diagnostic notation

```console
> cbor-diag --to diag <<-END
a26568656c6c6f65776f726c64a163796f756673686f756c64a165766973
6974d820781868747470733a2f2f63626f722e6e656d6f3135372e636f6d
END
{
    "hello": "world",
    { "you": "should" }: { "visit": 32_0("https://cbor.nemo157.com") },
}
```

### Parsing a hex-encoded payload into annotated hex

```console
> cbor-diag --to annotated <<-END
a26568656c6c6f65776f726c64a163796f756673686f756c64a165766973
6974d820781868747470733a2f2f63626f722e6e656d6f3135372e636f6d
END
a2                                                           # map(2)
   65                                                        #   text(5)
      68656c6c6f                                             #     "hello"
   65                                                        #   text(5)
      776f726c64                                             #     "world"
   a1                                                        #   map(1)
      63                                                     #     text(3)
         796f75                                              #       "you"
      66                                                     #     text(6)
         73686f756c64                                        #       "should"
   a1                                                        #   map(1)
      65                                                     #     text(5)
         7669736974                                          #       "visit"
      d8 20                                                  #     uri, tag(32)
         78 18                                               #       text(24)
            68747470733a2f2f63626f722e6e656d6f3135372e636f6d #         "https://cbor.nemo157.com"
                                                             #       valid URL (checked against URL Standard, not RFC 3986)
```

### Dumping diagnostic notation out to bytes
```console
> cbor-diag --to bytes <<-END | xxd
{"hello":"world","5 + 5 =": 10}
END
00000000: a265 6865 6c6c 6f65 776f 726c 6467 3520  .ehelloeworldg5
00000010: 2b20 3520 3d0a                           + 5 =.
```

[install Rust]: https://www.rust-lang.org/tools/install
[diagnostic notation]: https://tools.ietf.org/html/rfc7049#section-6
