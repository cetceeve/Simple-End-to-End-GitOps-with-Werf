We could not really cover the usage of werf as part of a ci/cd pipeline much.
In an actual deployment you would run werf from a ci tool like github actions.
It integrates with [differnet ci tools](https://werf.io/documentation/v1.2/usage/integration_with_ci_cd_systems.html). Instead of deploying into the local cluster you would deploy to a cluster running in the cloud.

# Commit hook

A way of simulating this locally is to use a `post-commit` hook in git.
Below is an example file for how this could look.

If we are commiting on the `master` branch, we deploy into the `production` environment.
If we commit to any other branch, that state will be deployed into the `development` environment.

```bash
echo '#!/bin/bash

ENVIRONMENT="development"

# Set environment to production if we are on the master branch
BRANCH="$(git branch --show-current)"
if [[ "$BRANCH" == "master" ]]; then
  ENVIRONMENT="production"
fi

# Call werf converge 
werf converge --dir="/root/demo-app" --repo localhost:5000/demo-app --env "$ENVIRONMENT"

' > /root/demo-app/.git/hooks/post-commit
chmod +x /root/demo-app/.git/hooks/post-commit
```{{exec}}

> Unfortunatly there is an [open issue](https://github.com/werf/werf/issues/3284) in werf right now, that stops this from working. Nonetheless, werf does work without problems in github actions etc. and provides [documentation on how to do that](https://werf.io/documentation/v1.2/usage/integration_with_ci_cd_systems.html).


# Check out the application

Now access our demo app via:

[ACCESS DEMO AP Development]({{TRAFFIC_HOST1_30081}})

[ACCESS DEMO AP Production]({{TRAFFIC_HOST1_30080}})

or here by entering port the port:

[ACCESS PORTS]({{TRAFFIC_SELECTOR}})