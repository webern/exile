#!/usr/bin/env bash
set -eo pipefail

usage=$(
  cat <<EOF
--
usage:
    ./pull-data.sh --outdir /some/path

parameters:
    -o|--outdir the directory to write the test data into (required).
--
EOF
)

while [[ $# -gt 0 ]]; do
  key="$1"

  case $key in
  -o | --outdir)
    outdir="$2"
    shift # past argument
    shift # past value
    ;;
  *) # unknown option
    echo "--"
    echo "unexpected argument '${1}'"
    echo "${usage}"
    exit 1
    shift # past argument
    ;;
  esac
done

echo "outdir is ${outdir}"

declare -a required_tools=(curl tar unzip)
for tool in "${required_tools[@]}"; do
  if ! [ -x "$(command -v ${tool})" ]; then
    echo "required command '${tool}' is not available"
    exit 2
  fi
done

mkdir -p "${outdir}"
cd "${outdir}" || exit 3

rm -rf xmlconf
rm -f xmlts20031210.zip
rm -rf xmlschema
rm -rf xmlschema2006-11-06
rm -f xsts-2007-06-20.tar.gz

curl -L https://www.w3.org/XML/Test/xmlts20031210.zip -o xmlts20031210.zip
unzip xmlts20031210.zip
rm -f xmlts20031210.zip

rm -f xsts-2007-06-20.tar.gz
curl -L https://www.w3.org/XML/2004/xml-schema-test-suite/xmlschema2006-11-06/xsts-2007-06-20.tar.gz \
  -o xsts-2007-06-20.tar.gz
tar -xvf xsts-2007-06-20.tar.gz
mv xmlschema2006-11-06 xmlschema
rm -f xsts-2007-06-20.tar.gz

