# saucy

[![Build Status](https://travis-ci.org/dcchut/saucy.svg?branch=master)](https://travis-ci.org/dcchut/saucy)
[![codecov](https://codecov.io/gh/dcchut/saucy/branch/master/graph/badge.svg)](https://codecov.io/gh/dcchut/saucy)

saucy is a helper tool for finding Python virtual environments.

## Motivation

Tired of writing:

```shell script, norun
user@server:~/important_project/> . env/bin/activate
(env) user@server:~/important_project/> .
```

all the time?  Wouldn't it be easier if you _didn't_ have to type `env/bin/activate` all the time?

## Example

In comes _saucy_.  To install saucy, run:

```shell script, no_run
cargo install saucy
```


When you run saucy in a directory, it will print out the `activate`
script of the first Python virtual environment it finds.  For example:

```shell script, no_run
user@server:~/important_project/> saucy
/home/user/important_project/env/bin/activate
```

Thus instead of having to type `. env/bin/activate`, you could just type:
```shell script, no_run
user@server:~/important_project/> . $(saucy)
(env) user@server:~/important_project/> 
```

But even _that_ feels like too much typing.  Instead, insert
the following line (or something similar) into your `.bash_aliases` file:

```shell script, no_run
alias sa=". $(saucy)$"
```

Now it's as easy as:

```shell script, no_run
user@server:~/important_project/> sa
(env) user@server:~/important_project/> 
```

Enjoy!

### License
Licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
