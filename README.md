As a language learner, it's nice to be able to read transcripts of things I watch to help parse it better. I find it much easier to read things on e-ink, so I put together this tool to merge subtitle files into an ebook so that I can load it onto my ereader.

# Arguments
The program accepts `.srt` subtitle files as input, and produces a `.epub` e-book as output.

`--in_dir`, `-i`: Path to the directory containing `.srt` files to merge. 
`--out_file`, `-o` (optional): Path where the output file will be created. If not specified, the file will be named `SubBook.epub` and placed in the current directory.
`--title`, `-t`: Sets the title of the ebook.
