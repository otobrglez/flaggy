# Flaggy is here to help with flags

Flaggy is a CLI tool used for generation of settings and feature flags for applications that use environment variables
to operate - for example with [12 factor app principles](https://12factor.net/).

## Usage

```
$ cargo build --release
./target/release/flaggy --flags-file flags/lucy.yml
```

Will take the given YAML file (that can also be found [here](./flags/lucy.yml)):

```yaml
flags:
  ENV:
    - staging
    - production
    - development
  FLAG_A: "a"
  FLAG_B: [ b,c ]
  FLAG_C: "https://epic.blog"
  FLAG_D: [ true, false ]
```

And will generate the following output (permutation of all flags):

```
ENV=staging;FLAG_A=a;FLAG_B=b;FLAG_C=https://epic.blog;FLAG_D=true
ENV=staging;FLAG_A=a;FLAG_B=b;FLAG_C=https://epic.blog;FLAG_D=false
ENV=staging;FLAG_A=a;FLAG_B=c;FLAG_C=https://epic.blog;FLAG_D=true
ENV=staging;FLAG_A=a;FLAG_B=c;FLAG_C=https://epic.blog;FLAG_D=false
ENV=production;FLAG_A=a;FLAG_B=b;FLAG_C=https://epic.blog;FLAG_D=true
ENV=production;FLAG_A=a;FLAG_B=b;FLAG_C=https://epic.blog;FLAG_D=false
ENV=production;FLAG_A=a;FLAG_B=c;FLAG_C=https://epic.blog;FLAG_D=true
ENV=production;FLAG_A=a;FLAG_B=c;FLAG_C=https://epic.blog;FLAG_D=false
ENV=development;FLAG_A=a;FLAG_B=b;FLAG_C=https://epic.blog;FLAG_D=true
ENV=development;FLAG_A=a;FLAG_B=b;FLAG_C=https://epic.blog;FLAG_D=false
ENV=development;FLAG_A=a;FLAG_B=c;FLAG_C=https://epic.blog;FLAG_D=true
ENV=development;FLAG_A=a;FLAG_B=c;FLAG_C=https://epic.blog;FLAG_D=false
```

## More options?

```
Usage: flaggy [OPTIONS] --flags-file <FLAGS_FILE>

Options:
  -f, --flags-file <FLAGS_FILE>
  -s, --separator <SEPARATOR>    [default: ;]
      --assigment <ASSIGMENT>    [default: =]
      --newline <NEWLINE>        [default: "\n"]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Author

- [Oto Brglez](https://github.com/otobrglez)

