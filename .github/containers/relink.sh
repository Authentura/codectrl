#!/usr/bin/env bash

for d in *; do
  [[ ! -d $d ]] && continue

  cd $d

  [[ -f bootstrap-action.sh ]] && \
    rm bootstrap-action.sh
  [[ -f find-and-rename-pkg.sh ]] && \
    rm find-and-rename-pkg.sh
 
  ln ../../../bootstrap-action.sh .
  ln ../../../find-and-rename-pkg.sh .

  cd ..
done

ls -lR
