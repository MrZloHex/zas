#!/bin/bash

######################
#  Made by MrZloHex  #
#     17.04.2022     #
######################

NAME="zas"
COMP_PATH="./target/release/"

load() {
	while [ 1 ]
	do
		echo >&6 -ne "."
		sleep 0.5
	done
}

compile() {
	echo >&6 -n "Compiling"

	load &
	PID=$!
	
	cargo build --release

	echo >&6 ""

	kill $PID
}

check_exec() {
	if [ -f "${COMP_PATH}${NAME}" ]
	then
		echo >&6 "Compiling finished succesfully"
	else
		echo >&6 "Compiling failed"
		exit 1
	fi
}

install() {
	echo >&6 -n "Installing"

	load &
	PID=$!

	sudo cp "${COMP_PATH}${NAME}" /usr/local/bin

	sleep 1	
	echo >&6 ""

	kill $PID
}

uninstall() {
	echo >&6 -n "Uninstalling"

	load &
	PID=$!

	sudo rm /usr/local/bin/$NAME
	
	sleep 1
	echo >&6 ""

	kill $PID
}

main() {
	exec 6>&1 >/dev/null
	exec 2>/dev/null

	sudo ls


	case $1 in
		"-i" | "--install") 
			compile
			check_exec
			install
			echo >&6 "DONE!!!"
			;;
		"-u" | "--uninstall")
			uninstall
			echo >&6 "DONE!!!"
			;;
		*)
			echo >&6 "For installation run:"
			echo >&6 "	./install.sh -i"
			echo >&6 "	./install.sh --install"
			echo >&6 "For uninstallation run:"
			echo >&6 "	./install.sh -u"
			echo >&6 "	./install.sh --uninstall"
			;;
	esac
}

main $@