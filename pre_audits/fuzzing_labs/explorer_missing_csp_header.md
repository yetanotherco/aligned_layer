# Missing Content-Security-Policy in `router.ex`

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 09/08/2024

## **Executive Summary**

While auditing and fuzzing some verification functions, we discovered that there is a missing Content-Security-Policy
header in the `router.ex` file.

## Vulnerability Details

- **Severity:** Informational

- **Affected Components:** `lib/explorer_web/router.ex`

## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** 
  - Erlang/OTP 25 [erts-13.2.2.9] [source] [64-bit] [smp:32:32] [ds:32:32:10] [async-threads:1] [jit:ns]
  - Elixir 1.14.0 (compiled with Erlang/OTP 24)

## Recommendations

You can specify CSP for `put_secure_browser_headers` in the browser pipeline of `ExplorerWeb.Router`

```elixir
defmodule ExplorerWeb.Router do
  use ExplorerWeb, :router

  pipeline :browser do
    ...
    plug :put_secure_browser_headers, %{"content-security-policy" => "default-src 'self'"}
  end
  ...
end
``