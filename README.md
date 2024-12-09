# opentype-tag

A simple utilty for converting OpenType-style integer (`u32`) tags into strings, and back.

## Input Style

It is implicitly assumed that the `u32` input is formatted in such a way to be directly parsed using the
`u32::from_str` implementation in the Rust stdlib. This means that the input `u32`-value should be given
in its decimal representation.

Support for hexadecimal input is not yet offered, but may be enabled in future, though it will probably require
a command-line-flag to be set.

## Output Style


As per the standard interpretation of OpenType tag-words, the `u32` is converted into a `byte[4]` array (big-endian),
and the output string consists of the display-form of these bytes, in order from left-to-right:

```
1146308935
=(hex)=> 0x44534947
=(bytes)=> [0x44, 0x53, 0x49, 0x47]
=(string)=> "DSIG"
```

Currently, there is only one stringification style (though more may be added in future). In this style,
all bytes corresponding to ASCII alphanumeric characters (`[0-9][A-Z][a-z]`) are output as a single,
verbatim character, and any other bytes are shown in raw hexadecimal form (e.g. `b' ' => "20"`). There are
no delimiters or other markers to disinguish a verbatim character from a raw-hex sequence, meaning that certain
uncommon tag-values may be ambiguous: "AB222" could be read as 'verbatim(A) verbatim(B) verbatim(2) raw(22)'
or 'verbatim(A) verbatim(B) raw(22) verbatim(2)', as an example. This ambiguity may not appear very often,
but complicates the reverse-translation process enough that even unambiguous strings may not be reversed
due to the difficulty in implementing logic that can identify that they are unambiguous, and process them accordingly.

In order to alleviate this, alternate output styles may be added that create unambiguous output, either by making all characters
verbatim (even if this leads to output that isn't as easy for humans to read), or including distinguishing features
such as `\xNN`-style escape tokens before bytes that are not output verbatim.

## Running

The simplest way to run this program is via `cargo`:

```
cargo run <args>
```

### Forward-conversion (`u32->String`)

When no flags are specified, the default behavior is to capture the first argument, attempt to parse it as a `u32`,
and print a traced stringification to stdout.

```
$ cargo run 1146308935
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/opentype-tag 1146308935`
1146308935 => "DSIG"
```

### Backward-conversion (`String->u32`)

Due to mild ambiguity of the default (and currently, only) output style for forward-conversion, not every conversion will necessarily round-trip.
In future, it may be possible to identify which conversions are truly ambiguous, and handle every unambiguous case; or, more directly, to change
the style of the string-output to eliminate ambiguity for reverse-conversion.

Here is an example of a successful reverse conversion:

```
$ cargo run -- --reverse DSIG
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/opentype-tag --reverse DSIG`
`DSIG` <= 1146308935
```

For backward-conversion, it is important to use quotes on the argument to the conversion when it would otherwise be improperly captured.
For example, `"SRB "` includes a space, which would be treated as a trailing whitespace separator by the shell and discarded from the string
if it were not quoted. However, any string which consists entirely of alphanumeric characters (or otherwise, which would be correctly captured
even without quotes) can be left unquoted.

Backward-conversion is semi-agnostic to the input-style, and can accept sequences that show all characters verbatim:

```
$ cargo run -- --reverse "SRB "
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/opentype-tag --reverse 'SRB '`
`SRB ` <= 1397899808
```
