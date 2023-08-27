target:
	cargo build --release

image:
	docker build . -t dmux/cotametais-lme-api-rust

push-image:
	docker push dmux/cotametais-lme-api-rust