# momi

[![Crates.io](https://img.shields.io/crates/v/momi)](https://crates.io/crates/momi)  [![Documentation](https://docs.rs/image/badge.svg)](https://docs.rs/momi/)

This project is a simple tool to help you create a new Minutes of Meeting (MoM) document.
It will create a new text file with the metadata of the meeting and a template for the MoM.

### Features

- Create a new MoM document
- Add metadata to the document(see below for the list of metadata)
- Add a template for the MoM
- Save the document to a specified name and location

## Installation

#### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

#### How to Install

```bash
cargo install momi
```

### Usage

#### Windows

```bash
momi.exe [options] <filename>
```

#### Mac and Linux

```bash
./momi [options] <filename>
```

#### Options

- `-o, --overwrite` Overwrite the file if it already exists
- `-v, --verbose` 
- `-a, --author <AUTHOR>`  The author of the document
- `-p, --open` Open all files after creating them
- `-e, --enrich` Add additional metadata to the document
- `-h, --help` Print help
- `-V, --version` Print version

#### Additional Options

- `--create-config` Create a default configuration file

##### Example

```bash
momi.exe -a "John Doe" "Meeting with the client 1.md"
```

And the file `Meeting with the client 1.md` contains the following content:
```
# Meeting with the client 1

created: 2024-04-30 04:58:44
author: John Doe



```

### Configuration

The configuration file is located at `config.json` in the same directory as the executable.
It can hold the following settings:

- `author`: The default author of the document
- `extension`: The default extension of the document
- `header`: The default header of the document
- `footer`: The default footer of the document
- `rich`: The custom metadata that can be added to the document

##### Example

In `config.json`:
```json
{
  "author": "John Doe",
  "extension": "txt",
  "header": "--------header--------",
  "footer": "--------footer--------",
  "rich": {
    "extra_metadata": [
      "location",
      "attendees",
      "meeting chair",
      "agenda"
    ]
  }
}
```

This json file will set the default values for metadata:
- The author of the document will be "John Doe"
- The extension of the document will be `txt`
- The header of the document will be "--------header--------"
- The footer of the document will be "--------footer--------"
- The extra metadata labels that can be added to the document are `location`, `attendees`, `meeting chair`, and `agenda`
  - These labels are optional and can be added to the document with the `-e` option

As a result of the above configuration, the following bash command will create a file with the following content:

```bash
momi.exe "Meeting with the client 1"
```

In the file `Meeting with the client 1.txt`:
```
Meeting with the client 1

created: 2024-04-30 05:01:39
author: John Doe

--------header--------

--------footer--------
```

If `-e` or `--enrich` option is provided, the program will ask for the extra metadata labels and values to be added to the document.

```bash
momi.exe -e "Meeting with the client 1"
```

```text
Meeting with the client 1

created: 2024-04-30 05:01:39
author: John Doe

location: 
attendees: 
meeting chair: 
agenda: 

--------header--------

--------footer--------
```

If `-o` or `--overwrite` options are provided on the command line,
the configuration file is overridden by the command line options.

### Supported Metadata

- `created`: The date and time the document was created
- `author`: The author of the document

### Supported Extensions

These are the extensions that the program can automatically add
to the filename as the title of the document:

- `.txt`: Text file
- `.md`: Markdown file

### How to determine which metadata is written to the document?

#### Order of Precedence

1. CLI Option
2. Configuration File
3. Nothing or Default Value

The program will write the metadata to the document if the author is provided as an CLI option.
If the author is not provided, the program will use the default author from the configuration file.
If the author is not provided in the configuration file, the program will write the current user's name as the author(`$USER`).

If the author is provided as an option at the same time as the configuration file,
the program will use the author provided as an option and ignore the author in the configuration file.

The rest of the metadata will be written to the document by same rules as the author.

### How to Build

#### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

#### Steps

1. Clone the repository
2. Open a terminal in the project directory
3. Run the following command:

```bash
cargo build --release
```

4. The executable will be located at `target/release/momi`
5. You can copy the executable to a directory in your PATH(optional)

You can also run the executable from the project directory with the following command:

```bash
cargo run -- [options] <filename>
```

### Not Implemented Yet But Planned

See [Todo.md](Todo.md) for the list of features that are planned but not implemented yet.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

