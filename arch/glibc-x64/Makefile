SHELL:=/bin/bash
TAG="stretch-test"

setup:
	docker build . --platform linux/amd64 -t $(TAG)

shell:
	docker run --platform linux/amd64 --rm -it $(TAG) bash

