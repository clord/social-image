
# Publish the image
publish:
	docker buildx build --platform=linux/amd64,linux/arm64  -t clord/social-image:latest --push .

