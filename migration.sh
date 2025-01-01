#!/bin/sh

if [[ -z $1 && -z $2 ]]; then
    echo "Provide service name and migration name"
    exit
fi

if [[ -z $1 ]]; then
    echo "Service name not provided"
    exit
fi

if [[ -z $2 ]]; then
    echo "migration name is not provided"
    exit
fi

case $1 in
  "account")
    echo "about to cd into account service to generate migration"
    cd ./account/src
    sea migrate generate "$2";;

  "reactions")
    echo "about to cd into reactions service to generate migration"
    cd ./reactions/src
    sea-orm-cli migrate generate "$2";;

  "posts")
    echo "about to cd into posts service to generate migration"
    cd ./posts/src
    sea-orm-cli migrate generate "$2";;

  "utils")
    echo "about to cd into utils service to generate migration"
    cd ./utils/src
    sea-orm-cli migrate generate "$2";;

  *)
    echo "No known service was chosen";;
esac