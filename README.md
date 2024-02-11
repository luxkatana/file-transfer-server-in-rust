# A server that streams only one file in rust

I wanted to practice my rust skills so here I am! 

## Warning

This program should not be used in production (it's not really optimized and its just for educative use case)

## How magic works 

<ol>
<li>It parses the second argument when calling the executable (is directly the file that'll be streamed to each client)</li>
<li>One packet is built like METADATA\0DATA where METADATA the data can be about the file & DATA the file data self</li>
<li>Because the SERVER METADATA is serialized in json, the client must have the same metadata-structure </li>
<li>Currently only the filename is given in the metadata but feel free to stick whatever you want</li>
<li> and boom! it's sent to the client!</li>
</ol>

<a href="https://github.com/luxkatana/file-transfer-client-in-rust">The corresponding client for this server is here</a>

## Run

Easy peasy, just 
```bash
cargo r -- <PATH_TO_FILE>
```