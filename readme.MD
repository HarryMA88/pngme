# PNGme

This CLI program allows you to encode secret messages in a PNG file.

## Install

Clone this repo and then run:

    cargo install --path .

## Running

Add a secret message to a PNG in a "RuST" chunk:

    pngme encode ./something.png RuST "Secret message here"

Add a secret message without overwriting the original file:

    pngme encode ./input.png RuST "Secret message here" ./output.png

Show your secret message:

    pngme decode ./something.png RuST

Remove the secret message:

    pngme remove ./something.png RuST

Print out every chunk in a PNG:

    pngme print ./something.png