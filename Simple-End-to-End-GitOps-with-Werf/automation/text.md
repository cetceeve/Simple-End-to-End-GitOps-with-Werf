Let's now automate our deployments localy.

We can use a [git-hook](https://git-scm.com/docs/githooks) to call `werf converge` automatically on commit.
Let's create a `post-commit` hook that will build and deploy automatically.

We could just run `werf converge --repo localhost:5000/demo-app`, but let's take it a bit further.
Werf supports multiple deployments by setting the `--env` option.
For every environment werf will create a new namespace in the kubernetes cluster.
For example if we run `werf converge --repo localhost:5000/demo-app --env test` this deployment will live in the namespace `demo-app-test`.

With our `post-commit` hook we will deploy into two different environments.
If we are commiting on the `main` branch, we deploy into the `production` environment.

If we commit to any other branch, that state will be deployed into the `development` environment.
```bash
tee /root/demo-app/.get/hooks/post-commit << EOF
#!/bin/bash

ENVIRONMENT="development"

# Set environment to production if we are on the main branch
BRANCH="$(git branch --show-current)"
if [[ "$BRANCH" == "main" ]]; then
  ENVIRONMENT="production"
fi

# Call werf converge 
werf converge --repo localhost:5000/demo-app --env "$ENVIRONMENT"

EOF
```{{exec}}

# Experiment

Go ahead and experiment with this a little bit.
Make some changes to the code and make some deployments.
How about changing something in the helm charts.

# Problem

While experimenting you might have come across a problem.
As long as you don't manually change the extermal port for the kubernetes service, you will only be able to access one of the deployed apps from your browser.

# Check out the application

Access our demo app via:

[ACCESS DEMO APP]({{TRAFFIC_HOST1_30081}})

or here by entering port: `30081`

[ACCESS PORTS]({{TRAFFIC_SELECTOR}})