#!/bin/sh


NPM=`which npm`
NODE=`which node`

setup_deps () {
  if [ ! -d "./rinterface/lib/duu/parity" ] ; then
      git clone  "https://github.com/janus/parity.git" "./rinterface/lib/duu/parity"
  fi

  #cd ./rinterface/deps/parity
  #cargo build -p ethkey-cli --release
  #cd ../../../
  
  if [ ! -d "./rinterface/lib/duu/althea_rs" ] ; then
      git clone  "https://github.com/janus/althea_rs.git" "./rinterface/lib/duu/althea_rs"
  fi

}



if [ -z "$NPM" -o -z "$NODE" ]; then
  echo "Required tools are missing - install either npm or node"
  exit 1
fi

start_jsonserver() {	
	cd rinterface/lib
	cargo test -- --nocapture &
	cd ../../smartContract
	echo "Sleep for 150 seconds to allow Cargo to complete"
    sleep 150
}

test_smartcontract() {	
	npm install
	testrpc &
	sleep 20
	rm -rf /build
	npm run migrate
	npm run test
	cd ../../
}

setup_deps
start_jsonserver
test_smartcontract



echo "Done O_0"
