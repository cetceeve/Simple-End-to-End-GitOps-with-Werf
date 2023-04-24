# Demo Project

Our demo project is a simple web-app called **demo-app** and resides in the folder `demo-app`.
The application was developed in Rust and serves static HTML pages.
You can checkout the application Code in `src/main.rs`.
It comes with a Dockerfile already provided in the folder.

# Git repository

To perform actual GitOps we need to initialize a git repository for our demo project:

```
cd demo-app/
git init
git add .
git commit -m "Initial commit"
```{{exec}}
