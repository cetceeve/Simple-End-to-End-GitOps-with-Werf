Let's go ahead and run werf for the first time. Make sure you are in the `demo-app/` folder. `pwd`{{exec}}:

`werf converge --repo localhost:5000/demo-app`{{exec}}

Now werf is building our application, deploying it to the registry, updating the manifests and then applying the new configuration to the Kubernetes cluster.

> *Notes on the image registry*: In order to avoid a login to dockerhub or any other cloud registry we are running a local registry here in our instance at `localhost:5000`. This registry is accessed via http instead of https, this is why we have specified in the previous step that werf is allowed to use an insecure registry.

TODO: add flowchart here

Werf pushes every build that it has created to the same image in the registry, differentiating them by tags.

Check out the images in the local registry by running:

`curl -X GET http://localhost:5000/v2/_catalog`{{exec}}

Check out the tags for this image by running:

`curl -X GET http://localhost:5000/v2/demo-app/tags/list`{{exec}}

Now access our demo app via:

[ACCESS DEMO APP]({{TRAFFIC_HOST1_30081}})

or here by entering port: `30081`

[ACCESS PORTS]({{TRAFFIC_SELECTOR}})