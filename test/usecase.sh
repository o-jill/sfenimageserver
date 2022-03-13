#!/bin/sh -x

cargo build --release

./target/release/sfenimageserver &

curl http://127.0.0.1:7582/ > nosfen.log

curl http://127.0.0.1:7582/help > help.html

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1 > test01.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1&image=png > test01.png

pkill sfenimageserver
