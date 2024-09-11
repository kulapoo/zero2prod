**Learning “Zero to Production in Rust”**

This README serves as a brief overview of my personal notes taken while learning from the book Zero to Production in Rust by Luca Palmieri. It captures key concepts, practical examples, and insights gathered throughout the learning process.

# App components

- Telemetry - **Instrumenting** the application
- Application - handles the startup of the project and global state
- Routes - handle routes
- Tests - handle Testing
- Configuration - Resolved all configuration into single configuration object. Settings input may came from different sources. Env, Host Provider, etc.
- Scripts - initialize the whole application from configuraition to docker setup
- Migrations - Contains all db structures

# Ch1 - Getting Started

speed up development

```toml
# .cargo/config.toml

[target.x86_64-unknown-linux-gnu]

rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]



[target.x86_64-pc-windows-msvc]

rustflags = ["-C", "link-arg=-fuse-ld=lld"]



[target.x86_64-pc-windows-gnu]

rustflags = ["-C", "link-arg=-fuse-ld=lld"]



[target.x86_64-apple-darwin]

rustflags = ["-C", "link-arg=-fuse-ld=lld"]



[target.aarch64-apple-darwin]

rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]
```

## Toolchains

- `lld` - developed by LLVM project
- `rustup` - similar to nvm but with other toolchains
- `cargo` - similar to npm
- `cargo-watch` - watching the source code, will run the configured command upon code change
- `cargo test` - obviously for testing with rust
- `cargo-tarpaulin` - cover code coverage
- `clippy` - similar to eslint
- `rustfmt` - similar to prettier
- `cargo-audit` - similar to `npm audit` to check security vulnerabilities, alternative to this is `cargo-deny`
- `cargo expand` - able to expand the macro definition to see in details

## CI

CI pipelines often go beyond ensuring code health: they are a good place to perform a series of additional

important checks - e.g. scanning our dependency tree for known vulnerabilities, linting, formatting, etc.

### CI providers rust setup

• [GitHub Actions - Rust setup · GitHub](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3)
• [CircleCI - Rust setup · GitHub](https://gist.github.com/LukeMathWalker/6153b07c4528ca1db416f24b09038fca)
• [GitLab CI - Rust setup · GitHub](https://gist.github.com/LukeMathWalker/d98fa8d0fc5394b347adf734ef0e85ec)
• [Travis CI - Rust setup · GitHub](https://gist.github.com/LukeMathWalker/41c57a57a61c75cc8a9d137a8d41ec10)

# Ch2 - Building an email newsletter

## User stories

- As a **blog visitor**,
  I want to **subscribe** to the **newsletter**,
  So that I can **receive email** updates when new content is published on the **blog**

- As the **blog author**,
  I want to send an email to all my subscribers,
  So that I can notify them when new content is published.

# what do I need

- Subcription
  - authentication
  - signup
- Notification service
  - author - send email
  - visitor - received email
- Content or Blog Posts
  - author blog CRUD?

# Ch3 - Signup a new subscriber

## Strategy

- choose a web framework and get familiar with it

- define our testing strategy

- choose a crate to interact with our database (we will have to save those emails somewhere!)

- define how we want to manage changes to our database schemas over time (a.k.a. migrations)

- actually write some queries.

Builder pattern = Fluent Api = Chaining

## Actix-Web Frameowork

- `HttpServer` - transport layer
- `Application` - application logic lives: routing, middlewares, request handlers, etc
- `Endpoint` - Route
  - Guards - matching which handler will be called

## Rust gives you three options when it comes to writing tests

• next to your code in an embedded test module, e.g.💯⭐️

```rust
// Some code I want to test
#[cfg(test)]
mod tests {
// Import the code I want to test
use super::*;
// My tests
}
```

- in an external tests folder, i.e.

```rust
src/
tests/
Cargo.toml
Cargo.lock
...
```

- as part of your public documentation (doc tests), e.g.

````rust
/// Check if a number is even.
/// ```rust
/// use zero2prod::is_even;
///
/// assert!(is_even(2));
/// assert!(!is_even(1));
/// ```
pub fn is_even(x: u64) -> bool {
x % 2 == 0
}
````

# Ch4 - Telemetry

_The goal is to have an observable application_

As a rule of thumb, everything we use in our application should be reflected in our integration tests.

The only thing we can rely on to understand and debug an unknown unknown is **telemetry** data

**Telemetry data** - information about our running applications that is collected automatically and can be later inspected to answer
questions about the state of the system at a certain point in time.

To build an observable system we need:

- to instrument our application to collect high-quality telemetry data;
- access to tools and systems to efficiently slice, dice and manipulate the data to find answers to our
  questions

## Log Levels

- Trace
- Debug
- Info
- Warn
- Error

## Known unknowns

examples:

- what happens if we lose connection to the database? Does sqlx::PgPool try to automatically recover
  or will all database interactions fail from that point onwards until we restart the application?

- what happens if an attacker tries to pass malicious payloads in the body of the POST /subscriptions
  request (i.e. extremely large payloads, attempts to perform SQL injection, etc.)?

## Tracing

**Tracing** expands upon logging-style diagnostics by allowing libraries and applications to record structured events with additional information about temporality and causality

**Span** in tracing has a beginning and end time, may be entered and exited by the flow of execution, and may exist within a nested tree of similar spans

## crates used

- env_logger
- log
- tracing_subscriber
- tracing_bunyan_formatter
- tracing_bunyan_formatter
- tracing-log
- bunyan
- secrecy
- tracing-actix-web

# Ch5 - Going Live

## crates

- cargo sqlx prepare
- cargo chef
- serde-aux

Must update the DATABASE_URL manually, get the connection string from digital ocean

# Ch6 - Reject Invalid Subscriber

**Type-driven development** - is a powerful approach to encode the constraints of a domain we are trying to model inside the type system, leaning on the compiler to make sure they are enforced

**Property-based testing** - instead of verifying that a certain set of inputs is
correctly parsed, we could build a random generator that produces valid values and check that our parser does not reject them

> When should you implement `AsRef<T>` for a type?
>
> When the type is similar enough to T that we can use a &self to get a reference to T itself!

[**Defense in depth**](<https://en.wikipedia.org/wiki/Defense_in_depth_(computing)>) - this a guide for security

> **Trust** but **verify**

## Domain modeling

- [Parse, don’t validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Domain Modeling Made Functional: Tackle Software Complexity with Domain-Driven Design and F# by Scott Wlaschin](https://pragprog.com/titles/swdddf/domain-modeling-made-functional/)

> If your Rust application panics in response to any user input, then the following should be true: your application has a bug, whether it be in a library or in the primary application code

> A panic in a request handler does not crash the whole application. actix-web spins up multiple workers to deal with incoming requests and it is resilient to one or more of them crashing: it will just spawn new ones to replace the ones that failed

<br />

## Crates

- unicode_segmentation
- fake
- claims
- validator

# Ch7 - Reject Invalid Subscriber 2

## Flow

### User subscribe

```mermaid
sequenceDiagram
    participant User
    participant API
    participant Database
    participant Email

    User->>API: POST /subscriptions
    API->>Database: Add user details with status pending_confirmation
    API->>Database: Generate and store subscription_token
    API->>Email: Send confirmation email with token link
    API-->>User: 200 OK
```

<br />

### User confirmation

```mermaid
sequenceDiagram
    participant User
    participant API
    participant Database

    User->>API: GET /subscriptions/confirm?token=<subscription_token>
    API->>API: Retrieve subscription_token from query parameters
    API->>Database: Retrieve subscriber id using subscription_token
    Database-->>API: Return subscriber id
    API->>Database: Update subscriber status to active in subscriptions table
    Database-->>API: Acknowledgment
    API-->>User: 200 OK
```

`reqwest` is no different - every time a Client instance is created reqwest initialises a connection pool under the hood.

## Email service

- Most email service providers uses SMTP and REST API

[Postmark: Fast, Reliable Email Delivery Service | SMTP | API](https://postmarkapp.com/)

> As a rule of thumb: every time you are performing an IO operation,
> always set a **timeout**!

## Tests

> A good test suite is, first and foremost, a risk-mitigation measure

> There is no need to define a main function in main.rs - the Rust test framework adds one for us behind the scenes

## Test code is still code

It has to be modular, well-structured, sufficiently documented. It requires maintenance.

> If we do not actively invest in the health of our test suite, it will rot over time.
>
> Coverage goes down and soon enough we will find critical paths in our application code that are never exercised by automated tests.

in short, apply the same separation of concerns principles to our **test suite** as we do to our **application**

## Test phases

- Execute test-specific setup (i.e. initialise a tracing subscriber);
- Randomise the configuration to ensure tests do not interfere with each other (i.e. a different logical
  database for each test case);
- Initialise external resources (e.g. create and migrate the database!);
- Build the application;
- Launch the application as a background task and return a set of resources to interact with it.

## Zero downtime

### Strategies:

- Rolling update - used by Digital Ocean
- Blue-green
- Canary

> **State** is hard.

### CSPRNG

- [cryptographically secure pseudo-random number generator](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator)

## crates

- linkify

# Ch8 - Error handling

> Look at the library Error enum might help

> We expect errors to carry enough context about the failure to produce a **report** (logs, --verbose) for an operator (e.g. the developer) that contains enough details to go and troubleshoot the issue

**Errors** serve two main purposes:

- Control flow (i.e. determine what do next);
- Reporting (e.g. investigate, after the fact, what went wrong on).

| Purpose      | Internal               | At the Edge   |
| ------------ | ---------------------- | ------------- |
| Control Flow | Types, methods, fields | Status codes  |
| Reporting    | Logs/traces            | Response body |

We can also distinguish errors based on their location:

- Internal (i.e. a function calling another function within our application);
- At the edge (i.e. an API request that we failed to fulfill).

Rust community to standardise on the minimum requirements for a **good error**:

- it should provide different representations (Debug and Display), tuned to different audiences;
- it should be possible to look at the underlying cause of the error, if any (source).

## AVOID “BALL OF MUD” ERROR ENUMS

- Use library to avoid too many error enums that implement display, source to wrapped the error type
- We do not want to expose the implementation details of the fallible routines/function that get mapped to `UnexpectedError` - it must be **opaque**

> Use an **opaque** error, do not give the caller programmatic access to the error inner details

> **anyhow** is for applications, **thiserror** is for libraries.

> Implement `Box<dyn std::error::Error>` if needed as it can handle any kind of error that implements the std::error::Error trait

> Reason carefully about your usecase and the assumptions you can afford to make in order to design the most appropriate error type - sometimes `Box<dyn std::error::Error>` or `anyhow::Error` are the most appropriate choice, even for libraries.

## Additional Resources on Error Handling

[RustConf 2020 - Error handling Isn't All About Errors by Jane Lusby - YouTube](https://www.youtube.com/watch?v=rAF8mLI0naQ) #tech/rust/error-handling

[What the Error Handling Project Group is Working On | Inside Rust Blog](https://blog.rust-lang.org/inside-rust/2020/11/23/What-the-error-handling-project-group-is-working-on.html)

[Error Handling in a Correctness-Critical Rust Project | sled-rs.github.io](http://sled.rs/errors.html)

# Ch9 - Newsletter Delivery

> As the **blog author**,
> want to send an email to all my **confirmed** subscribers,
> So that I can notify them when new content is published.

## Implementation Strategy

- Retrieve the newsletter issue details from the body of the incoming API call;
- Fetch the list of all confirmed subscribers from the database;
- Iterate through the whole list:
  - Get the subscriber email;
  - Send an email out via Postmark.

```mermaid
sequenceDiagram
    participant API as Incoming API Call
    participant Server as Server
    participant DB as Database
    participant EmailService as Postmark

    API ->> Server: Retrieve newsletter issue details
    Server ->> DB: Fetch list of confirmed subscribers
    DB -->> Server: List of subscribers

    loop For each subscriber
        Server ->> Server: Get subscriber email
        Server ->> EmailService: Send email
        EmailService -->> Server: Email sent confirmation
    end
```

## Things to do at after this chapter and WHY

1. **Security**

Our POST /newsletters endpoint is unprotected - anyone can fire a request to it and broadcast to our entire audience, unchecked.

2. **You Only Get One Shot**

As soon as you hit POST /newsletters, your content goes out to your entire mailing list. No chance to edit or review it in draft mode before giving the green light for publishing.

3. **Performance**

We are sending emails out one at a time. We wait for the current one to be dispatched successfully before moving on to the next in line. This is not a massive issue if you have 10 or 20 subscribers, but it becomes noticeable shortly afterwards: latency is going to be horrible for newsletters with a sizeable audience.

4. **Fault Tolerance**

If we fail to dispatch one email, we bubble up the error using ? and return a 500 Internal Server Error to the caller. The remaining emails are never sent, nor do we retry to dispatch the failed one.

5. **Retry Safety**

Many things can go wrong when communicating over the network. What should a consumer of our API do if they experience a timeout or a 500 Internal Server Error when calling our service? They cannot retry - they risk sending the newsletter issue twice to the entire mailing list.

# Ch10 - Security

## Authentication

Is the process of verifying the identity of a user or entity. It answers the question, “**Who are you?**” The goal of authentication is to confirm that the user is indeed who they claim to be. This is typically achieved using one or more of the following methods:

### Verifying user identity

1. **Something You Know**: This includes passwords, PINs, or answers to security questions. The user is authenticated if they provide the correct information.

2. **Something You Have**: This involves physical devices such as a smartphone, a security token, or a smart card. The user might need to provide a code generated by the device, or simply have the device connected to authenticate.

3. **Something You Are**: This involves biometric data, such as fingerprints, facial recognition, retina scans, or voice recognition.

4. **Somewhere You Are**: Authentication can sometimes involve location, verified through IP addresses or GPS data.

5. **Something You Do**: This could involve behavioral biometrics, like typing speed, gait, or other activity-based patterns.

**Multi-factor Authentication (MFA)** combines two or more of these methods to enhance security.

## Authorization

is the process of determining whether an authenticated user has permission to access a specific resource or perform a particular action. It answers the question, “**What are you allowed to do?**” Once a user is authenticated, the system checks their privileges or permissions to see what actions they are permitted to carry out.

• **Read**: Permission to view a file or data.

• **Write**: Permission to edit or modify a file or data.

• **Execute**: Permission to run a program or execute a command.

• **Delete**: Permission to remove a file or data.

`Basic` Authentication Scheme, a standard defined by the Internet Engineering Task Force (IETF) in [RFC 2617](https://datatracker.ietf.org/doc/html/rfc2617#section-2) and later updated by [RFC 7617](https://datatracker.ietf.org/doc/html/rfc7617)

**Basic Auth Scheme Flow**

```mermaid
graph TD
    A[Start] --> B[Client sends request to Server]
    B --> C{Server checks if request is authorized}
    C --|Unauthorized|--> D[Server sends challenge <br> WWW-Authenticate: Basic realm='WallyWorld']
    C --|Authorized|--> H[Server grants access to the resource]
    D --> E[Client sends credentials]
    E --> F[Credentials = base64  userID:password ]
    F --> G{Server decodes and checks credentials}
    G --|Valid|--> H
    G --|Invalid|--> I[Server responds with 401 Unauthorized]
    H --> J[End]
    I --> J[End]
```

## Dictionary Attack

## Argon2

## [OWASP](https://owasp.org/) guidance password guidance

- Use Argon2id with a minimum configuration of 15 MiB of memory, an iteration count of 2, and 1 degree of parallelism.
- If Argon2id is not available, use bcrypt with a work factor of 10 or more and with a password
  limit of 72 bytes.
- For legacy systems using scrypt, use a minimum CPU/memory cost parameter of (2^16), a minimum block size of 8 (1024 bytes), and a parallelization parameter of 1.
- If FIPS-140 compliance is required, use PBKDF2 with a work factor of 310,000 or more and set with an internal hash function of HMAC-SHA-256.
- Consider using a pepper to provide additional defense in depth (though alone, it provides no additional secure characteristics).

## [PHC-string-format](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#specification)

> `async/await` in Rust is built around a concept called cooperative scheduling.

[What is blocking](https://ryhl.io/blog/async-what-is-blocking/) - Async article #tech/rust/async

> a man-in-the-middle attack - MITM

## Three categories of API callers:

- Other APIs (machine-to-machine);
- A person, via a browser;
- Another API, on behalf of a person.

> Social logins rely on [identity federation](https://en.wikipedia.org/wiki/Federated_identity)

[Message authentication](https://en.wikipedia.org/wiki/Message_authentication) - it guarantees that the message has not been modified in transit
(integrity) and it allows you to verify the identity of the sender (data origin authentication).

**HMAC** - This will help securing query parameters

```mermaid
graph TD

A[Client send request with a <br> query string] --> B[Attacker tries to modify <br> the query string]

B --> C{Is HMAC tag valid?}

C --|No|--> D[Server detects tampering]

D --> E[Reject request <br> and log the attempt]

C --|Yes|--> F[Server calculates <br> HMAC using secret key]

F --> G{Does calculated HMAC <br> match received HMAC?}

G --|No|--> H[Reject request <br> due to mismatch]

G --|Yes|--> I[Accept request <br> and process safely]

E --> J[End]

H --> J

I --> J

```

> Error Messages Must Be **Ephemeral** (like an flash error indicator - toast message)

## Use Cookie ephemeral error message or Flash messages

```mermaid
graph TD

A[User enters invalid <br> credentials] --> B[User submits login form]

B --> C[Server receives <br> POST /login request]

C --> D[Server sets a <br> cookie with error message]

D --> E[Server redirects <br> to GET /login]

E --> F[Browser calls <br> GET /login with cookies]

F --> G[Server checks for <br> error message in cookies]

G --|Error found|--> H[Server prepares <br> HTML <br> form with error message]

G --|No error|--> I[Server prepares <br> normal HTML form]

H --> J[Server deletes error <br> message from cookie]

I --> K[Server returns HTML <br> form to the user]

J --> K

K --> L[End]
```

## Cookie usages

- Setting Cookie **Max-Age** to zero will immediately removed the cookie after browser reload

- **Http-Only** - can hide created cookies from clientside code, the browser will store them and attach them to outgoing requests. Scripts will not be able to see them

- A **cookie** value with an HMAC tag attached is often referred to as a **signed cookie**

## Session-based Authentication

Session-based authentication is a strategy to avoid asking users to provide their password on every single page

### Flow sample

```mermaid
graph TD

A[User visits login page] --> B[User enters <br> credentials and submits]

B --> C[Server validates credentials]

C -->|Invalid credentials| D[Return error <br> message to user]

C -->|Valid credentials| E[Generate session token]

E --> F[Set session token as cookie]

F --> G[Browser stores cookie]

G --> H[User accesses <br> restricted functionality]

H --> I[Browser sends <br> request with <br> session token in cookie]

I --> J[Server verifies <br> session token]

J -->|Valid token| K[Grant access to <br> restricted functionality]

J -->|Invalid token| L[Return error or <br> request re-authentication]
```

a user is “logged in” if there is a valid user id associated with the
user_id key in the session state. To log out it is enough to delete the session - remove the state from the storage backend and unset the client-side cookie

## Session Fixation Vulnerability in Web-based Applications

A **Session Fixation Vulnerability** occurs when an attacker tricks a user into using a specific session ID, which the attacker knows in advance. Once the user logs in, the attacker uses the known session ID to gain unauthorized access to the user’s account. This vulnerability can be prevented by regenerating a new session ID after a successful login.

[acrossecurity.com/papers/session_fixation.pdf](https://acrossecurity.com/papers/session_fixation.pdf)

# Exercises

- Add a Send a newsletter issue link to the admin dashboard;
- Add an HTML form at GET /admin/newsletters to submit a new issue;
- Adapt POST /newsletters to process the form data:
  - Change the route to POST /admin/newsletters;
  - Migrate from ‘Basic’ to session-based authentication;
  - Use the Form extractor (application/x-www-form-urlencoded) instead of the Json extractor (application/json) to handle the request body;
- Adapt the test suite.

# Ch11 - Fault Tolerant

**Fault-tolerant** refers to a system’s ability to continue operating properly in the event of a failure of some of its components. In other words, a fault-tolerant system is designed to handle errors or malfunctions without affecting its overall functionality or performance.

## 4 Concepts for reliable service

- idempotency
- locking
- queues
- background jobs

## Goals

1. **Ensure Best-Effort Delivery**: Strive to deliver the new newsletter issue to all subscribers as effectively as possible.

2. **Acknowledge Delivery Limitations**: Understand that not all emails will be delivered due to potential issues, such as accounts being deleted or other delivery failures.

3. **Minimize Duplicate Deliveries**: Implement measures to reduce the likelihood of subscribers receiving the same issue multiple times.

4. **Accept the Possibility of Some Duplicates**: Recognize that it is not feasible to eliminate duplicate deliveries entirely, but the implementation should aim to minimize their occurrence.

## Idempotent - Retry Safety

## HTTP Streaming

HTTP/1.1 supports another mechanism to transfer data - Transfer-Encoding: chunked

[HTTP Streaming (or Chunked vs Store & Forward) · GitHub](https://gist.github.com/CMCDragonkai/6bfade6431e9ffb7fe88)

> **Cross-request synchronization** will avoid duplicate delivery

## Backward Recovery

tries to achieve a semantic rollback by executing **compensating actions** i.e Order did not deliver - proceed to refund flow

## Forward Recovery

drive the overall workflow to completion even if one or more
sub-tasks did not succeed.

### Passive recovery

pushes on the API caller the responsibility to drive the workflow to completion

### Active recovery

does not require the caller to do anything apart from kicking off the workflow.

The system must self-heal

> By running all async expressions on the current task, the expressions are able to run concurrently but not in parallel. This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue. If parallelism is required, spawn each async expression using tokio::spawn and pass the join handle to select!.

# Learn more

- **Anything under tests ends up being compiled in its own binary**

- We do not want to venture further with a broken CI

- Your database is a gigantic global variable

- Want to know more about [Serde](https://www.joshmcguigan.com/blog/understanding-serde/)?

- Two techniques to ensure test isolation when interacting with relational database in a test:
  - wrap the whole test in a SQL transaction and rollback at the end of it - good but tricky how to capture the "transaction" feature, maybe depends on db storage
  - Spin up a brand-new logical database for each integration test

# Cheatsheet

- You can mute a warning using the `#[allow(clippy::lint_name)]`
- Disable whole lint in clippy.toml by `#![allow(clippy::lint_name)]`

# Endpoints

## /subscriptions

post

```sh
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
http://127.0.0.1:8000/subscriptions
```

## /health_check

```sh
curl http://127.0.0.1:8000/health_check -i
```

# Commands

## Cargo

### cargo watch

```sh
cargo watch -x check -x test -x run
```

### cargo tarpaulin

```sh
 cargo tarpaulin --ignore-tests
```

### clippy for failing the linter

```sh
cargo clippy -- -D warnings
```

### formatting the code

```sh
 cargo fmt -- --check # will fail if contains unformmated code
```

### cargo test with fn name filter

```sh

export TEST_LOG=true && cargo test [test_function_name]  | bunyan
```

### cargo test fn name filter with options

```sh
TEST_LOG=true cargo test --quiet --release \
[test_fn_name] | grep "VERIFY PASSWORD" | bunyan
```

### cargo nightly installation

`--allow-downgrade` - Some components of the bundle installed by rustup might be broken/missing on the latest nightly release:
`--allow-downgrade` tells rustup to find and install the latest nightly where all the needed components are
available

```sh
rustup toolchain install nightly --allow-downgrade
```

### display generated code from macros

```sh
cargo expand --test health_check

cargo +nightly expand --bin [name of the bin]
```

## Postgres

### running sqlx migrate

```sh
sqlx migrate run
```

```sh
# prod
DATABASE_URL=POSTGRES_URL sqlx migrate run
```

### add table

```sh
sqlx migrate add [table name]
```

## Docker

build project

```sh
# make sure to run -> cargo sqlx prepare

docker build --tag zero2prod --file Dockerfile .
```

run project

```sh
docker run -p 8000:8000 zero2prod
```

list detail about project

```sh
docker images zero2prod
```

check status

```sh
docker ps
```

## Digital Ocean

```sh
doctl apps create --spec spec.yaml
```

```sh
doctl apps list
```

```sh
doctl auth init
```

```sh
# db migration into digital ocean - execute into your command line, ensure your machine or ip is added as a trusted source in digital ocean
DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run
```

### updating the spec

get the doctl id

```sh
doctl app list --format ID
```

update

```sh
doctl apps update $APP_ID --spec spec.yaml
```

## Troubleshooting

ulimit - to resolve error with `code: 24, kind: Uncategorized, message: "Too many open files`

```sh
ulimit -n 10000
```

# Resources

- [source code](https://github.com/LukeMathWalker/zero-to-production)

# Db Config

```sh
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
```

```
host=

port=25060
username=newsletter
password=
database=newsletter
sslmode=require
```

# Creds

digital ocean

```sh
[DIGITAL OCEAN PROJ ID]
```

admin pwd: everythinghastostartsomewhere

# Additional Exercises

- What happens if a user tries to subscribe twice? Make sure that they receive two confirmation emails;
- What happens if a user clicks on a confirmation link twice?
- What happens if the subscription token is well-formatted but non-existent?
- Add validation on the incoming token, we are currently passing the raw user input straight into a
  query (thanks sqlx for protecting us from SQL injections <3);
- Use a proper templating solution for our emails (e.g. tera);
- Anything that comes to your mind!
- **ch11** adding a n_retries and execute_after
- columns to keep track of how many attempts have already taken place and how long we should wait before trying again.
- expiry mechanism for our idempotency keys

# TODO

- go back to study the Authentication and Authorisation, password hashing

# Related

- [[Inner development loop]]
- [[Luca Palmieri]]

<br />
<br />
