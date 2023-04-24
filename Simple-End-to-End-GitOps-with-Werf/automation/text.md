In an actual deployment you would run werf from a ci tool like github actions.
We cannot really simulate this here locally but here is an example in the form of a shell script.

# Multiple branches

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


# Check out the application

Now access our demo app via:

[ACCESS DEMO AP Development]({{TRAFFIC_HOST1_30081}})

[ACCESS DEMO AP Production]({{TRAFFIC_HOST1_30080}})

or here by entering port the port:

[ACCESS PORTS]({{TRAFFIC_SELECTOR}})