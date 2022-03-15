
# Publish the image
publish:
	echo $$(git describe --dirty --tags --long --always)
	cargo test # check for errors
	cargo package # check for errors
	cargo clippy # check for errors
	docker buildx build \
		--platform=linux/amd64,linux/arm64 \
		--tag clord/social-image:latest \
		--tag clord/social-image:$$(git describe --dirty --tags --long --always)  \
		--push .
	cargo publish

