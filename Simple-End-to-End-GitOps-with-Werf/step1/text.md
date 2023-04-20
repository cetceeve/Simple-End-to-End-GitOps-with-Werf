Refer to installation instructions here: [Werf installation guide](https://werf.io/installation.html?version=1.2&channel=stable&os=linux&arch=amd64&method=installer)

```
curl -sSLO https://werf.io/install.sh && chmod +x install.sh
./install.sh --version 1.2 --channel stable --no-interactive
```{{exec}}

```
Current login shell is "bash". Press ENTER to setup werf for this shell or choose another one.
trdl is going to be installed in "/root/bin/". Add this directory to your $PATH in "/root/.bashrc" and "/root/.profile"? (strongly recommended)
Add automatic werf activation to "/root/.bashrc" and "/root/.profile"? (recommended for interactive usage, not recommended for CI)
```

Activate werf with:
```
source $("/root/bin/trdl" use werf "1.2" "stable")
```{{exec}}

Because we are running a local docker registry we need to allow werf to skip TLS certificate checks
```
export WERF_INSECURE_REGISTRY=1
```{{exec}}

Test with `werf version`{{exec}}