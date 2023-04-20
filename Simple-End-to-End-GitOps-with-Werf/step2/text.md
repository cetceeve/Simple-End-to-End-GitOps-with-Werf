Make first deployment

```
docker push localhost:5000/demo-app:latest
k apply -f demo-app/k8s.yml
```{{exec}}
