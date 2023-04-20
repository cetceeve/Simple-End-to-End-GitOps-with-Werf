#!/bin/bash

if [ echo "$WERF_INSECURE_REGISTRY" != "1" ]; then exit 1; fi