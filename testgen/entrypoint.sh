#!/usr/bin/env bash
set -eou pipefail
cd /xmltestgen
mvn package
java -cp target/xmltestgen-1.0-SNAPSHOT.jar com.matthewjamesbriggs.xmltestgen.App --testdata /data
