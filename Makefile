.PHONY: clean

## data/w3cdata: creates the data directory by pulling testdata from dockerhub
data/w3cdata:
	rm -rf data && \
	mkdir -p data && \
	docker run --network=host \
			--user "$(id -u):$(id -g)" \
			-v ${PWD}:/volume/target \
			--rm -t matthewjamesbriggs/xmltestdata:v0.2.0 \
			/bin/sh -c 'cp -a /data/xmlconf /volume/target/data && cp -a /data/xmlschema /volume/target/data' && \
			touch data/w3cdata

## testdata: pulls testdata from dockerhub into the gitignored data directory
testdata: data/w3cdata

clean:
	rm -rf data && \
	rm -rf target
