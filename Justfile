####!/usr/bin/env just --working-directory . --justfile
# vim: set ft=make :

install:
	cargo install -f --path .

git-pull:
	#!/bin/sh
	git-clone-or-pull

