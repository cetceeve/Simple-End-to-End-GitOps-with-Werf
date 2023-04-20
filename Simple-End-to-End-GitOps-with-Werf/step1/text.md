# Install werf

Refer to installation instruction here: [Werf installation guide](https://werf.io/installation.html?version=1.2&channel=stable&os=linux&arch=amd64&method=installer)

```
curl -sSLO https://werf.io/install.sh && chmod +x install.sh
./install.sh --version 1.2 --channel stable
```{{exec}}


Because we are running a local docker registry we need to allow werf to skip TLS certificate checks
```
export WERF_INSECURE_REGISTRY=1
```{{exec}}

Test with `werf version`{{exec}}