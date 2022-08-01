# set shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-c"]
protoc_version := "21.4"
protoc_releases := "https://github.com/protocolbuffers/protobuf/releases/download/v" + protoc_version

protoc_win := protoc_releases / "protoc-" + protoc_version + "-win64.zip"
protoc_linux := protoc_releases / "protoc-" + protoc_version + "-linux-x86_64.zip"
protoc_osx := protoc_releases / "protoc-" + protoc_version + "-osx-universal_binary.zip"

platform := `echo $OSTYPE`
protoc_zip := if platform == "msys" { protoc_win } else if platform == "darwin" { protoc_osx } else if platform == "linux-gnu" { protoc_linux } else { "ERROR" }
protoc_exe := if os_family() == "windows" { "protoc.exe" } else { "protoc" }

protos:
    git submodule init
    git submodule update

update-protos:
    git pull --recurse-submodules

test:
    cargo test

build:
    PROTOC="./protoc/bin/{{protoc_exe}}" cargo build

install-protoc:
    mkdir ./protoc/ && curl -L -o ./protoc/protoc.zip {{protoc_zip}} && unzip ./protoc/protoc.zip -d ./protoc/

prepare: protos install-protoc build
