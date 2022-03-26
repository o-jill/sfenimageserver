#!/bin/sh -x

cargo build --release

./target/release/sfenimageserver --log usecase.log &

curl http://127.0.0.1:7582/ > nosfen.log

curl http://127.0.0.1:7582/help > help.html

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1 > test01.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=0093FU > test02.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=13\&title=hello+world!! > test03.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&sname=o-jill\&gname=%e3%81%a2%e3%82%8b > test04.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&image=png > test01.png

pkill sfenimageserver
