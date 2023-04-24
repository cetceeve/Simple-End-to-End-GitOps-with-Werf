Let's setup the helm charts for our simple web app.

We need to specify two kubernetes objects.
First let's create a deployment for the demo-app.

```
mkdir -p /root/demo-app/.helm/templates
echo 'apiVersion: apps/v1
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
' > /root/demo-app/.helm/templates/deployment.yml
```{{exec}}

Note that the image we are using is specified as a value supplied by werf during the deployment.
Thats how werf automatically updates the image for us.

We also need a Service to make our application accessible from the outside.
```
echo 'apiVersion: v1
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
' > /root/demo-app/.helm/templates/service.yml
```{{exec}}

Finally we need a [werf.yml](https://werf.io/documentation/v1.2/reference/werf_yaml.html) file that contains metadata about our application. Most importantly we need to define our project name and where werf can find the Dockerfiles' to build from.

```bash
echo 'configVersion: 1
project: demo-app
---
image: demoapp
dockerfile: ./Dockerfile
' > /root/demo-app/werf.yml
```{{exec}}

Now we are ready to go.
Before we can continue we need to commit what we have.

```
cd /root/demo-app
git add .
git commit -m "added helm charts"
```{{exec}}