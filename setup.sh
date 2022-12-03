#!/usr/bin/env bash

cargo build
cd godot

if [ "$OS" = "Windows_NT" ]; then
	rustlib="tutorial3d-gd-native.dll"
else
	case $(uname -sm) in
	"Darwin x86_64") rustlib="libtutorial3d_gd_native.dylib" ;;
	*) rustlib="libtutorial3d_gd_native.so" ;;
	esac
fi

ln -s "../target/debug/${rustlib}" "${rustlib}"