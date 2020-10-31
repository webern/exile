#!/usr/bin/env bash
set -eou pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

# we need the script to be placed in the same parent dir that houses
# the xmltestgen java sourcecode directory.
cd "${script_dir}/xmltestgen"

# use maven to compile the program
mvn package

# assign the command line options to variables in order to see them
# in a wrapped format, before passing them with no nelines to maven
a="--w3c-xml /data/xmlconf/xmlconf.xml"
b="--w3c-schema /data/xmlschema/suite.xml"
c="--custom-xml /fake"
d="--custom-schema /fake"
e="--xml-outdir /workspace/exile/tests"
f="--schema-outdir /fake"
g="--rust-root /workspace"
h="--exile-tests /workspace/xtest/data" # TODO - this will move

# use maven to execute the program
mvn exec:java \
  -Dexec.mainClass="com.matthewjamesbriggs.xmltestgen.App" \
  -Dexec.args="${a} ${b} ${c} ${d} ${e} ${f} ${g} ${h}"
