#!/usr/bin/env bash


package_file=$(find "$1"/target/{debian,generate-rpm} | egrep '*\.(deb|rpm)' | head -n1)
