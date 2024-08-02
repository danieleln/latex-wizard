# latex
`latex` is a command line tool to manage, you guessed, LaTeX projects.
It allows to initialize a new LaTeX project and to compile it.



## Requirements
The installation of `latex` requires compiling its source code (`rust`).
So, make sure to have `cargo` installed first.

Once installed, `latex` relies on the following commands to compile
`LaTeX` projects:

- `pdflatex` to generate the output `.pdf`
- `biber` to compile the bibliography
- `makeglossaries` to generate the glossary



## Installation
```bash
git clone https://github.com/danieleln/latex.git
cd latex
cargo build --release
cargo install --path .
```
Also, make sure to include cargo's bin directory (`$HOME/.cargo/bin`
by default) in the `PATH` variable.



## Usage
Quick overview of the available commands:

```bash
# Initialize a new LaTeX project
latex init my-latex-proj

# Compile an existing project
latex compile [--clean]
latex compile /path/to/my-latex-proj [--clean]
latex compile /path/to/my-latex-proj/main.tex [--clean]
```



### Init command
Running `latex init my-latex-proj` creates the following directories
and files:

```
./my-latex-proj
│
├── main.tex
│
├── out/
│
└── .git/
```

Where:

- `main.tex` is the main `.tex` file
- `out/` is the output directory. When compiling, all the compilation
      files (as well as the output `.pdf` file) will be stored in this
      directory
- `.git`: a git repository is automatically initialized



### Compile command
The easiest way to compile a project is by running `latex compile`.
This command works when inside the `./my-latex-proj` directory or any
of its sub-directories.
Other ways to compile the project are:

```bash
latex compile /path/to/my-latex-proj
latex compile /path/to/my-latex-proj/main.tex
```

Note that, specifying any other path (like other `.tex` files inside
`my-latex-proj/` directory) won't work.

Additional flags:

- `--clean`: it removes all files inside the `out/` directory,
      except for the output `.pdf` file (`main.pdf`), which is left
      in case the compilation fails.

