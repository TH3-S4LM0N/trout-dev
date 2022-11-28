#!/bin/bash
# shell version of trout-frontend

name="Trout"
author="TH3-S4LM0N"
version=''
about="CLI music player downloading from Spotify"
cmd="busctl --user call org.trout.Backend /org/trout/Backend org.trout.BackendI"

# subcommands
play() {
    playlist=""
    site=""
    while [[ "$1" ]]; do
        case $1 in
            "-p") playlist="$2" ;;
            "--playlist") playlist="$2" ;;
            "spotify") site="spotify" ;;
        esac
        shift
    done
    
}
gen() {
    if [ $2 == "" ]; then
        2="$HOME/trout"
    fi
    $cmd "Gen" "s" "$2"
}
new() {
    echo
}
test() {
   echo 
}



while [[ "$1" ]]; do
    case $1 in 
        # here we parse all args and set eq vars
        "play") play "$@" ;;
        "p") play "$@" ;;
        "gen") gen "$@" ;;
        "new") new "$@" ;;
        "n") new "$@" ;;
        "test") test "$@" ;;
    esac
    shift
done