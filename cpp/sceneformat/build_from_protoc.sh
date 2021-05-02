#!/bin/bash
protoc -I=../../proto ../../proto/scene.proto --cpp_out=.