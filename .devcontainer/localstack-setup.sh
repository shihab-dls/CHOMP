#!/bin/bash

awslocal s3api create-bucket \
    --bucket xchemlab-targeting \
    --create-bucket-configuration LocationConstraint=eu-west-1 \
    --region eu-west-1
