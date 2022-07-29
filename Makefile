build:
	podman build -t todo-app-api_api -f ./docker/rust.Dockerfile .

up:
	podman play kube api.yml

down:
	podman play kube --down api.yml