#!/usr/bin/env bash

if [[ -f .rustfmt-triggered ]]; then
  rm .rustfmt-triggered
  export GIT_SHA="$(git rev-parse HEAD)"
  echo "# ($(date -Ihours)) $(git log --format=%ae -1 $GIT_SHA): $(git log --format=%B -1 $GIT_SHA)" >> .git-blame-ignore-revs
  echo "${GIT_SHA}" >> .git-blame-ignore-revs
  echo "" >> .git-blame-ignore-revs

  git add .git-blame-ignore-revs

  git commit -m "post-commit: add $(git rev-parse --short HEAD) to .git-blame-ignore-revs" \
             -m "Reason: reformatted code" \
             --no-verify
fi
