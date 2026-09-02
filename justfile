
version := `cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version'`

build-img-as-latest: build-img
    docker tag themata-api:{{ version }} themata-api:latest

build-img:
    docker build . -t themata-api:{{ version }}

save:
    docker save themata-api:{{ version }} | xz > themata-api-{{ version }}.img.tar.xz
