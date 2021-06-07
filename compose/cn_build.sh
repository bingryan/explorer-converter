#! /usr/bin/env bash

set_gitconfig()
{
echo "
[url \"https://github.com.cnpmjs.org/\"]
    insteadOf = https://github.com/

[http]
	sslVerify = false
	postBuffer = 1048576000
[submodule]
	recurse = true
" > ~/.gitconfig
}


set_cargo()
{
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
}

set_gitconfig
set_cargo