@echo off

:: run this once: rustup target add thumbv7em-none-eabihf

cargo build --target thumbv7em-none-eabihf
