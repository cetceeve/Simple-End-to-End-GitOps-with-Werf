Now it's time to make some changes in the app.

Go to `src/` and check out the files there.
Find a place to make a small change that will be visible on the website.

For example you could replace in `src/main.rs`
```rust
async fn home() -> Html<&'static str> {
    println!("GET: /");
    Html("Hello, world!")
}
```

with

```rust
async fn home() -> Html<&'static str> {
    println!("GET: /");
    Html("Hey, check this out: <a href=\"https://dl.acm.org/doi/pdf/10.1145/3534857\">Visit KTH.se!</a>")
}
```{{copy}}

After making those changes go ahead and run werf:

`werf converge --repo localhost:5000/demo-app`{{exec}}

UPS! As you can see werf is enforcing GitOps best practices.
Without making a commit you cannot run a new deployment.

Go ahead, commit the changes and run werf converge again!