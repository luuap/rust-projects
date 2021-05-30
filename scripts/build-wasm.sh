#!/bin/bash

if [[ "$1" == "" ]] 
then
    echo "Need to provide the scope for the packages as the first argument. Omit the @symbol."
else
  for dir in wasm/*/
  do
    # Note: we are building the packages sequentially because the cargo cache folder gets blocked when in use, so doing
    #       it in parallel seems like wasted effort
    ( 
      cd "$dir"
      wasm-pack build --scope "$1"
      retVal=$?
      if [ "$retVal" -ne 0 ]
      then
        echo "Error building $dir"
        return ${retVal} 2>/dev/null
      fi
      # workaround until https://github.com/rustwasm/wasm-bindgen/issues/1614 gets resolved
      find ./pkg -type f -name '*_bg.js' | xargs sed -i 's/instanceof CanvasRenderingContext2D/instanceof OffscreenCanvasRenderingContext2D/g'
      echo "Successfully built $dir"
      echo ""
    )
  done
fi

