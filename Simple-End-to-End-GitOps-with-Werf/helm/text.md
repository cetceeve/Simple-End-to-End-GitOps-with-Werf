Let's setup the helm charts for our simple web app.

We need to specify two kubernetes objects.
First let's create a deployment for the demo-app.

```
tee /root/demo-app/.helm/templates/deployment.yml << EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: demo-app-deployment
  labels:
    app: demo-app
spec:
  selector:
    matchLabels:
      app: demo-app
  replicas: 1
  template:
    metadata:
      labels:
        app: demo-app
    spec:
      containers:
      - name: demo-app
        image: {{ .Values.werf.image.demoapp }}
        ports:
        - containerPort: 3000
EOF
```{{exec}}

Note that the image we are using is specified as a value supplied by werf during the deployment.
Thats how werf automatically updates the image for us.

We also need a Service to make our application accessible from the outside.
```
tee /root/demo-app/.helm/templates/service.yml << EOF
kind: Service
metadata:
  name: demo-app
spec:
  type: NodePort
  selector:
    app: demo-app
  ports:
    - protocol: TCP
      port: 3000
      targetPort: 3000
      nodePort: 30081
EOF
```{{exec}}

Finally we need a [werf.yml](https://werf.io/documentation/v1.2/reference/werf_yaml.html) file that contains metadata about our application. Most importantly we need to define our project name and where werf can find the Dockerfiles' to build from.

```
tee /root/demo-app/werf.yml << EOF
configVersion: 1
project: demo-app
---
image: demoapp
dockerfile: ./Dockerfile
EOF
```{{exec}}

Now we are ready to go: