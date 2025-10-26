# polar-rs
Polar SDK for Rust

[API Overview](https://polar.sh/docs/api-reference/introduction)

example `.envrc.local` file:

```sh
export POLAR_API_KEY=my_fancy_api_key
export POLAR_SANDBOX_API_KEY=my_fancy_sandbox_api_key
```

to test that it's set up properly, try:
<https://polar.sh/docs/api-reference/introduction#quick-examples>

```nu
http get https://sandbox-api.polar.sh/v1/products/ --headers [{Authorization: $"Bearer ($env.POLAR_SANDBOX_API_KEY)", Accept: application/json}]
```
