Werf supports multiple deployment environments by setting the `--env` option.
For every environment werf will create a new namespace in the kubernetes cluster.
For example, if we run `werf converge --repo localhost:5000/demo-app --env test` this deployment will live in the namespace *demo-app-test*.

However, as long as you don't manually change the external port for the kubernetes service, you will only be able to access one of the deployed apps from your browser. Let's solve this.

# Environment-dependent template parameters

Let's make the external access port of our web-app dependent on the environments we are deploying into.

We can implement this easily with [Environment-dependent template parameters](https://werf.io/documentation/v1.2/usage/deploy/environments.html#environment-dependent-template-parameters-werf-only).

We start by creating a `values.yaml` (has to be .yaml) file for our helm charts with two different ports for the `production` and `development` environment.

```
echo 'nodePort:
  development: 30081
  production: 30080
' > /root/demo-app/.helm/values.yaml
```{{exec}}

We can access those values via `$.Values` in our helm templates.
The current environment is stored in `$.Values.werf.env`.

With this let's change the `.helm/templates/service.yaml` definition.
Change the last line

`nodePort: 30081`

to

`nodePort: {{ index $.Values.nodePort $.Values.werf.env }}`{{copy}}

# Multiple branches

This feature is especially useful in combination with multiple branches.
You could have one production deployment running reflecting the `master` branch and one deployment running reflecting a `dev` branch.

Start by deploying your current application state to the production environment.

`werf converge --repo localhost:5000/demo-app --env production`{{exec}}

Now create a new branch and checkout that branch.

```
git branch development
git checkout development
```{{exec}}

Make some changes in this new branch and commit them.
Deploy the state of this branch into the depoyment environment `development`.

`werf converge --repo localhost:5000/demo-app --env development`{{exec}}

Both deployments are now running in parallel in their respective namespaces.

> Useful command for exploring the result:

`k get namespaces`{{exec}}

# Check out the application

Now access our demo app via:

[ACCESS DEMO AP Development]({{TRAFFIC_HOST1_30081}})

[ACCESS DEMO AP Production]({{TRAFFIC_HOST1_30080}})

or here by entering port the port:

[ACCESS PORTS]({{TRAFFIC_SELECTOR}})