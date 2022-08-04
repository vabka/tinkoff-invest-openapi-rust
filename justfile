# set shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-c"]
protoc_version := "21.4"
protoc_releases := "https://github.com/protocolbuffers/protobuf/releases/download/v" + protoc_version

win_platform := "win64"
linux_platform := "linux-x86_64"
osx_platform := "osx-universal_binary"

platform := `echo $OSTYPE`
protoc_platform := if platform == "msys" { win_platform } else if platform == "darwin" { osx_platform } else if platform == "linux-gnu" { linux_platform } else { "ERROR" }

protoc_uri := protoc_releases / "protoc-" + protoc_version + "-"+protoc_platform+".zip"


protoc_exe := if os_family() == "windows" { "protoc.exe" } else { "protoc" }

protos:
    git submodule init
    git submodule update

update-protos:
    git pull --recurse-submodules

# test:
#    cargo test

# build:
#    PROTOC="./protoc/bin/{{protoc_exe}}" cargo build

# install-protoc:
#    mkdir ./protoc/ && curl -L -o ./protoc/protoc.zip {{protoc_uri}} && unzip ./protoc/protoc.zip -d ./protoc/

# prepare: protos install-protoc build
