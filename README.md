# PNG PANG BAM

An implementation of Picklenerd's [PNGme](https://picklenerd.github.io/pngme_book) project. 
It incorporates some test code from this project.

PNG PANG BAM will modify a PNG image file and embed a "secret" message. It can also display or remove these messages.

This repository contains the functional library as well as the command-line application front end.


## Supported Commands

### encode
`pngpangbam encode <FILENAME> <CHUNK_TYPE> <MESSAGE> [OUT_FILENAME]`

Encode a message with the given chunk type, writing the output back to the original file or designated
output file if specified.

### decode
`pngpangbam decode <FILENAME> <CHUNK_TYPE>`

Decode a message with the specified chunk type embedded in the file.

### remove
`pngpangbam remove <FILENAME> <CHUNK_TYPE> [OUT_FILENAME]`

Remove the message with the given chunk type.

### print
`pngpangbam print <FILENAME>`

Generate a list of chunk types and their counts.

## Future Goals
- Some form of obfuscation or proper encryption would be fun.
- Additional validation of input and PNG files would be ideal.
- A graphical UI.