# Welcome

A warm welcome to this [GitOps](https://www.redhat.com/en/topics/devops/what-is-gitops) and [werf](https://werf.io) tutorial.
Before we get started, please wait while the setup is running in the console.
While this is going on, let's look at some core concepts of GitOps and what you can expect from this tutorial.

# Prerequisites

You should be familiar with *Docker* and *Git* and have a basic understanding about *Kubernetes* and *Helm*.

# Learning Outcomes

In this tutorial, you will get to know GitOps and the GitOps agent [werf](https://werf.io).

After finishing this tutorial, you will know about the core ideas of GitOps.
You will know how to setup an end-to-end GitOps workflow using werf and will be able to deploy applications to different environments (development, production) with a simple commit.

# GitOps

The core idea behind GitOps is to use a git-based workflow in order to manage your application deployment.
We want to utilize common git commands, like committing, as the triggers for our deployments.
Additionally, our git repository becomes the single source of truth for our deployments.
That means that our deployment will always reflect what is defined in our git repository.

In technical terms, this means that, in addition to code, we commit declarative configuration files (e.g. for Kubernetes) or *Infrastructure as Code* configuration files to our git repository.
From there, we utilize a GitOps agent (e.g. flux or werf) to take the configurations from our repository and apply them to our production or test environment.
To give some further examples, you can use git reset to perform rollbacks and utilize git branches to support different deployment environments (development, production).

# Motivation

Utilizing a Git-based workflow (e.g. Pull Requests, Code Reviews), allow developers to use familiar tools for deployments.
By using git as the single source of truth, you automatically create a history of your deployments.
Having a complete history of your deployments provides strong transparency about when and by whom changes were made which reduces the overall risk of unwanted changes and helps eliminate issues quickly.