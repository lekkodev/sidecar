#!/bin/bash

registry=public.ecr.aws/lekko
repository=lekko/sidecar

# First, check for credentials so we can later proceed to change state safely.
aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin $registry

git_commit=`git rev-parse HEAD`

echo What version would you like to tag this release? e.g. v0.1.0:

read version

echo Releasing version $version

for arch in amd64 arm64
do
    echo Building for architecture linux/$arch
    docker build -t $registry/$repository:$version-$arch --build-arg SIDECAR_VERSION=$version --build-arg SIDECAR_GIT_COMMIT=$git_commit -f Dockerfile.sidecar --arch=linux/$arch .
done

echo Tagging version $version
git tag -a $version -m $version -f
git push origin --tags

for arch in amd64 arm64
do
    echo Pushing $version-$arch to $registry/$repository
    docker push $registry/$repository:$version-$arch
done

echo Creating manifest list for $version
docker manifest create $registry/$repository:$version \
    $registry/$repository:$version-amd64  \
    $registry/$repository:$version-arm64

for arch in amd64 arm64
do
    docker manifest annotate --arch $arch $registry/$repository:$version $registry/$repository:$version-$arch    
done

docker manifest push $registry/$repository:$version

echo Released $version. View the release here: https://gallery.ecr.aws/lekko/lekko/sidecar
