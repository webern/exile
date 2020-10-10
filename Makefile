.PHONY: w3cdata zip

w3cdata:
	rm -rf ${PWD}/data && \
	mkdir -p ${PWD}/data && \
	docker run --network=host \
			--user "$(id -u):$(id -g)" \
			-v ${PWD}/data:/volume/target \
			--rm -t matthewjamesbriggs/xmltestdata:v0.2.0 \
			cp -a /data /volume/target

zip: build ./target/x86_64-unknown-linux-musl/release/bootstrap
	zip -j build/telemetry-data-processor.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
