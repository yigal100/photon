set shell := ["pwsh.exe", "-NoProfile", "-Command"]

default:
    just --list

build: 
    echo  build
