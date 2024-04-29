# Minutes of Meeting Initializer

## Description

This project is a simple tool to help you create a new Minutes of Meeting (MoM) document. 
It will create a new text file with the metadata of the meeting and a template for the MoM.

## Features

- Create a new MoM document
- Add metadata to the document(see below for the list of metadata)
- Add a template for the MoM
- Save the document to a specified name and location

## Usage

### Windows

```bash
momi.exe [options] <filename> 
```

### Mac and Linux

```bash
./momi [options] <filename> 
```

### Options

- `-h, --help`: Show help message and exit
- `-V, --version`: Show version and exit
- `-a, --author <author>`: Add the author of the document
- `-v, --verbose`: Show verbose output
- `-o, --overwrite`: Overwrite the file if it already exists

#### Example

```bash
momi.exe -a "John Doe" "Meeting with the client 1"
```

And in the file `Meeting with the client 1.txt`:
```text
Meeting with the client 1

created: 2024-04-30 04:58:44
author: John Doe



```

## Configuration

The configuration file is located at `config.json` in the same directory as the executable.
It can hold the following settings:

- `author`: The default author of the document
- `extension`: The default extension of the document
- `header`: The default header of the document
- `footer`: The default footer of the document

#### Example

In `config.json`:
```json
{
    "author": "John Doe",
    "extension": ".txt",
    "header": "--------header--------",
    "footer": "--------footer--------"
}
```

As a result of the above configuration, the following bash command will create a file with the following content:

```bash
momi.exe "Meeting with the client 1"
```

In the file `Meeting with the client 1.txt`:
```text
Meeting with the client 1

created: 2024-04-30 05:01:39
author: John Doe
--------header--------

--------footer--------
```

## Supported Metadata

- `created`: The date and time the document was created
- `author`: The author of the document

## Supported Extensions

These are the extensions that the program can automatically add 
to the filename as the title of the document:

- `.txt`: Text file
- `.md`: Markdown file

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


### Not Implemented Yet But Planned

- Handle overwriting of existing files more safely
- Open the document in the default text editor
- Add attendees to the metadata
- Add the location of the meeting to the metadata
- Add the meeting chair to the metadata
- Add the meeting agenda to the metadata
- Add the organization name to the metadata
- Make the mechanism to add metadata more user-friendly
- Add a feature to edit the metadata
- Add sections to the template such as "Action Items", "Decisions Made", "Next Steps", "Conclusion", etc.
- Support for more markdown and text file formats
