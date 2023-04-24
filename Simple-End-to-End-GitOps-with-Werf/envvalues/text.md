Let's make the external access port of our web-app dependent on the environments we are deploying into.

We can implement this easily with [Environment-dependent template parameters](https://werf.io/documentation/v1.2/usage/deploy/environments.html#environment-dependent-template-parameters-werf-only).

We start by creating a `values.yml` file for our helm charts with two different ports for the `production` and `development` environment.

```bash
tee -a /root/demo-app/.helm/values.yml << EOF
nodePort:
  development: 30081
  production: 30080
EOF
```{{exec}}

We can access those values via `$.Values` in our helm templates.
The current environment is stored in `$.Values.werf.env`.

With this let's change the `.helm/templates/services` definition.
Change the last line

`nodePort: 30081`

to

`nodePort: {{ index $.Values.nodePort $.Values.werf.env }}`

Now both deloyments can easily be accessed from the browser.

# Check out the application

Now access our demo app via:

[ACCESS DEMO AP Development]({{TRAFFIC_HOST1_30081}})
[ACCESS DEMO AP Production]({{TRAFFIC_HOST1_30080}})

or here by entering port the port:

[ACCESS PORTS]({{TRAFFIC_SELECTOR}})