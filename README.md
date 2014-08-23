# BEAR

Well, I was trying to learn Rust, looking around the internets, then I see the Rust example at [the Rust homepage][rust], and having come in contact with Brainfsck at an earlier time, this came into being. After quickly fixing [hammer.rs][hammer], I implemented this language. I purposedly changed a few opcodes from Brainfsck, just to throw you off. ;-) 

Five examples are included:

- bref.br (Technically, this is just an op listing...)
- bf.br (My first real example.)
- hello_loop.br (Who wants to see "Hello" printed to their console 1,000 times?)
- hello_name.br (A stalkerish program...)
- hello_name_ptr.br (Exactly like hello_name.br, except we use the memory pointer for constants rather than hardcoding memory.)

Excerpt from bref.br (about op codes):

    > - go forward one cell - FWD
    < - go back one cell - BCK
    + - increase the value of the current cell by one - INC
    - - decrease the value of the current cell by one - DCR
    ! - print the value of the current cell as a character - PCC
    ) - go forward ten cells - ADV
    ( - go back ten cells - RWD
    * - increase the value of the current cell by ten - LFT
    / - decrease the value of the current cell by ten - DWN
    [ - go over to the next ] or the end of the file if the current cell's value is 0 - LBL
    ] - go back to the last [ or the beginning of the file if the current cell's calue is not 0 - RBL
    ? - print the location of the memory pointer - PPL
    & - print the location of the memory pointer as a char - PPC
    . - print the value of the current cell - PCL
    ` - get a character as input and store its value in the current cell - INP
    ^ - reset the memory pointer to 0 - RPT
    # - Set current cell's value to 0 - RCV
    
Anyways, I hope you have as much time playing around with this as I had making this. This was done in one afternoon, so it's a bit rough around the edges.

[rust]: http://rust-lang.org
[hammer]: https://github.com/Bobhostern/hammer.rs