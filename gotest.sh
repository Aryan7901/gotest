#!/bin/bash

openCmd() {
    case "$OSTYPE" in
       cygwin*)
          cmd /c start cover.html
          ;;
       linux*)
          xdg-open cover.html
          ;;
       darwin*)
          open cover.html
          ;;
    esac
}

if [ $1 -eq '0' ]; then
    go test -coverprofile=cover.out $2 ; \
    cat cover.out | \
    awk 'BEGIN {cov=0; stat=0;} \
            $3!="" { cov+=($3==1?$2:0); stat+=$2; } \
        END {printf("Total Coverage: %.2f%% of statements\n", cov/stat*100);}'
fi

if [ $1 -eq '1' ]; then
    go test -coverprofile=cover.out $2 ; \
    go tool cover -html=cover.out -o cover.html
    if [ $3 -eq '1' ]; then
        openCmd
    fi
fi
