#!/bin/sh -x

cargo build --release

./target/release/sfenimageserver --log ./test/usecase.log &

curl http://127.0.0.1:7582/ > ./test/nosfen.log

curl http://127.0.0.1:7582/help > ./test/help.html

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1 > ./test/test01.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=0093FU > ./test/test02.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=13\&title=hello+world!! > ./test/test03.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&sname=o-jill\&gname=%e3%81%a2%e3%82%8b > ./test/test04.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&image=png > ./test/test01.png

pkill sfenimageserver

./target/release/sfenimageserver --log ./test/usecase.log --dark &

curl http://127.0.0.1:7582/ > ./test/dk_nosfen.log

curl http://127.0.0.1:7582/help > ./test/dk_help.html

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1 > ./test/dk_test01.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=0093FU > ./test/dk_test02.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=13\&title=hello+world!! > ./test/dk_test03.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&sname=o-jill\&gname=%e3%81%a2%e3%82%8b > ./test/dk_test04.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&image=png > ./test/dk_test01.png

pkill sfenimageserver

./target/release/sfenimageserver --log ./test/usecase.log --fgcolor red --bgcolor green &

curl http://127.0.0.1:7582/ > ./test/rg_nosfen.log

curl http://127.0.0.1:7582/help > ./test/rg_help.html

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1 > ./test/rg_test01.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=0093FU > ./test/rg_test02.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&lm=13\&title=hello+world!! > ./test/rg_test03.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&sname=o-jill\&gname=%e3%81%a2%e3%82%8b > ./test/rg_test04.svg

curl http://127.0.0.1:7582/?sfen=lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL+b+-+1\&image=png > ./test/rg_test01.png

pkill sfenimageserver
