Next, we need to install werf. You can refer to the installation instructions here: [Werf installation guide](https://werf.io/installation.html?version=1.2&channel=stable&os=linux&arch=amd64&method=installer)

We are downloading the installer and running werf with all default options.
```
curl -sSLO https://werf.io/install.sh && chmod +x install.sh
./install.sh --version 1.2 --channel stable --no-interactive
```{{exec}}

After the installation, we need to activate werf with the command below:
```
echo "Activating werf..."
source $("/root/bin/trdl" use werf "1.2" "stable")
rm ./install.sh
echo "Activation complete"
```{{exec}}

Because we are running a local docker registry (more on that later) we need to allow werf to skip TLS certificate checks:
```
export WERF_INSECURE_REGISTRY=1
```{{exec}}

Test with `werf version`{{exec}}