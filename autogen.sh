#!/usr/bin/env bash

# Plugin name
PLUGINNAME=rustTemplate

# Plugin execution order, as 3-digit decimal
PLUGINORDER=999

# --------------------- DO NOT EDIT BELOW HERE --------------------------

T2BUILD_BACKEND=cargo
. "$(dirname "$0")/../autogen.sh"
