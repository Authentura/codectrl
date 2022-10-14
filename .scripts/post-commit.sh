#!/usr/bin/env bash

if [[ -f .rustfmt-triggered ]]; then
  rm .rustfmt-triggered
  export GIT_SHA="$(git rev-parse HEAD)"
  echo "# $(git rev-list --format=%ae --max-count=1 $GIT_SHA | tail -n1): $(git rev-list --format=%B --max-count=1 $GIT_SHA | tail -n1)" >> .git-blame-ignore-revs
  echo "${GIT_SHA}" >> .git-blame-ignore-revs
  echo "" >> .git-blame-ignore-revs

  git add .git-blame-ignore-revs
  git commit --amend --no-edit
fi
