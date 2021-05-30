#!/bin/bash

for dir in wasm/*/
do
  ( 
    # Creates a global link based on the 'name' property in package.json 
    cd "${dir}/pkg"
    npm link
  )
done