#!/bin/bash

registry=public.ecr.aws/g0r8j2n2
repository=lekko/sidecar

echo What version would you like to tag this release? e.g. v0.1.0:

read version

echo Tagging version $version
git tag -a $version -m $version
git push origin --tags

echo Pushing $version to $registry/$repository 
docker build -t $registry/$repository:$version -f Dockerfile.sidecar --platform=linux/amd64 .
aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin $registry
docker push $registry/$repository:$version
