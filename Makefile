
# Publish the image
publish:
	echo $$(git describe --dirty --tags --long --always)
	docker buildx build \
		--platform=linux/amd64,linux/arm64 \
		--tag clord/social-image:latest \
		--tag clord/social-image:$$(git describe --dirty --tags --long --always)  \
		--push .

