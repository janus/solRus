#!/bin/sh

XTERM=`which xterm`
KONSOLE=`which konsole`
NPM=`which npm`
NODE=`which node`

if [ -z "$XTERM" -o -z "$KONSOLE" -o -z "$NPM" -o -z "$NODE" ]; then
  echo "Required tools are missing - install either xterm or konsole"
  exit 1
fi

start_jsonserver() {	
	cd rinterface/lib
	if [ -x "$XTERM" ]; then
		`"$XTERM" -hold -e cargo test -- --nocapture` &
	else
		`"$KONSOLE" --noclose -e cargo test -- --nocapture` &
	fi
	cd ../../
	echo "Sleep for 90 seconds to allow Cargo to complete"
	sleep 90
}

start_testrpcserver() {	
	if [ -x "$XTERM" ]; then		
		`"$XTERM" -hold -e truffle develop` &
	else
		`"$KONSOLE" --noclose -e truffle develop` &
	fi
	echo "Sleep for 30 seconds to allow test rpc server to launch"
	sleep 30
}

test_smartcontract() {	
	cd smartContract
	#npm install
	npm run migrate
	npm run test
	cd ..
}

start_jsonserver
start_testrpcserver
test_smartcontract

echo "Done O_0"
