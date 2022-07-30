#!/bin/bash

export PATH="$PATH:/root/.cargo/bin"

echo -e "\nExecuting \"$@\""
$@
