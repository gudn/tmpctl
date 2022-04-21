# tmpctl

This is simple CLI tool for fast clean temporal files. Just enter directory and
run `tmpctl --force`.

## .tmpignore files
When tmpctl scan file tree, it also find `.tmpignore` files in parent
directories (but not in childs). Syntax is almost like gitignore files except
three parts:
- No `!` syntax
- Matching based on regexps, so `s*.patt` will match `234sbla.patt`
- Supports raw regexp matching. Just start line with `:` colon and write your
  regular expressions. All paths is relative to `.tmpignore` directory
  
Example:

``` ignore-list
# Comment
src/
Cargo.*
# will match .git folder and .gitignore file
.git
```

`tmpctl` will read all ignore files in parent like this:

```
/.tmpignore # will be used
/home
    /.tmpignore # will be used
    /user
        /tmp # run tmpctl here
            .tmpignore # will be used
            /sub
                .tmpignore # will not be used
```

## Force argument
By default, tmpctl just prints file names to remove. When `--force` flag is
provided it will actually remove.
