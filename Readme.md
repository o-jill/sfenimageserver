[![CC0](https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1)![CC0](https://mirrors.creativecommons.org/presskit/icons/zero.svg?ref=chooser-v1)](ref="http://creativecommons.org/publicdomain/zero/1.0?ref=chooser-v1)
[![issues](https://img.shields.io/github/issues/o-jill/sfenimageserver.svg)](https://github.com/o-jill/sfenimageserver/issues/)
[![Rust](https://github.com/o-jill/sfenimageserver/actions/workflows/rust.yml/badge.svg)](https://github.com/o-jill/sfenimageserver/actions/workflows/rust.yml)

# sfenimageserver  
is a web server which serves svg and png from sfen.  

rsvg-convert (and inkscape) are supported to generate png.

# options:  
* --port \<port number\>  
  configure port number. default: 7582.  
  ex. ./sfenimageserver --port 12345  
* --log \<log path\>  
  configure log path. default: no log file.  
  ex. ./sfenimageserver -- log /var/log/to/my/log.txt  
* --rsvg  
  use rsvg-convert. selected by default.  
* --inkscape11  
  use inkscape ver.1.1.  
* --light  
  fgcolor:black, bgcolor:white. default.
* --dark  
  fgcolor:white, bgcolor:black.
* --bgcolor \<color\>  
  configure background color only for png. default: white.  
  black, white, red, ... #rrggbb  
* --fgcolor \<color\>  
  configure foreground color. default: black.  
  black, white, red, ... #rrggbb  

---
