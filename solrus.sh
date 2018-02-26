#!/bin/sh


NPM=`which npm`
NODE=`which node`

if [ -z "$NPM" -o -z "$NODE" ]; then
  echo "Required tools are missing - install either npm or node"
  exit 1
fi

start_jsonserver() {	
	cd rinterface/lib
	cargo test -- --nocapture &
	cd ../../smartContract
	echo "Sleep for 50 seconds to allow Cargo to complete"
	sleep 50
}

test_smartcontract() {	
	npm install
	testrpc &
	sleep 15
	rm -rf /build
	npm run migrate
	npm run test
	cd ..
}

start_jsonserver
test_smartcontract

echo "Done O_0"
