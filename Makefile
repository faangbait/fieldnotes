#!/bin/bash

.PHONY: build

.DEFAULT_GOAL := build

build:
	cargo build --release

run:
	target/release/fieldnotes