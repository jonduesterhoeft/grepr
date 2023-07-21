# grepr

[![Rust](https://github.com/jonduesterhoeft/mgrep/actions/workflows/rust.yml/badge.svg)](https://github.com/jonduesterhoeft/mgrep/actions/workflows/rust.yml)

A minimal version of grep implemented in Rust.

# Overview #
**grepr** is a simple command line search tool. A search string and 
file path are input as arguments, along with several optionals 
to fine tune the search. The program iterates through each line in the
specified file and will return any lines matching the search criteria.

# Options #
Optional arguments are shown via the --help flag.
```console
$ grepr --help
A simple to use command line search tool, à la grep.

Usage: mgrep [OPTIONS] <QUERY> <PATH>

Arguments:
  <QUERY>  Search query
  <PATH>   File path

Options:
  -i, --ignore-case   Ignores case whiles searching
  -v, --invert-match  Inverst search results
  -w, --word          Matches exact words only
  -l, --line          Matches exact lines only
  -h, --help          Print help
  -V, --version       Print version
```

# Examples #
A simple search example.
```console
$ grepr sunbeam tests/pale_blue_dot.txt

test/pale_blue_dot.txt
11: on a mote of dust suspended in a sunbeam.
```

Search for an exact word. In this case any non-alphanumeric characters
are ignored.
```console
$ grepr "in" tests/pale_blue_dot.txt -w

test/pale_blue_dot.txt
6: civilization, every king and peasant, every young couple in love, 
9: "superstar," every "supreme leader," every saint and sinner in 
11: on a mote of dust suspended in a sunbeam.

```

Inverting the search results. All lines without a match are returned.
```console
$ grepr a tests/pale_blue_dot.txt -v

test/pale_blue_dot.txt
1: On it everyone you love, everyone you know, everyone you ever 
10: the history of our species lived there--
12: 

```
