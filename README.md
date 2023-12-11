# ServiceNow Bug Tracker

### Background

This started out as a small exploration about ServiceNow's REST API.
Admittedly, this is not really how ServiceNow was meant to be used. This bug
tracker is simply using ServiceNow's REST API as a back-end database, whereas
ServiceNow is meant to serve more as a full-stack low-code/no-code web app
development service. I chose this development route not out of practicality but
instead curiosity. I wanted to see if it was possible to have a purely
front-end application using WASM and encryption to handle API key's being
stored on the client side. I would NEVER do this in production, its a terrible
idea. However, for a simple demo where the API key's will probably expire
before it matters, this solution is much cheaper than paying for a full web
host like AWS. If you are curious about the client-side keys, see the
"Encrypted API Keys with WASM". As for the rest of the application, please
continue on to the next section...

### Cool AI Features
- **Bug Submission Maker:** Since this is a demo, I thought it would be a nice
  feature to quickly populate the submission form with a semi-realistic bug
  report for quick testing.

- **Automated Bug Title and Summary Generation:** This automatically makes a
  title and short description based on the bug submission. The title and
  description is then used within the bug tracker table.

- **Automated Department Routing:** I thought it would be nice to automatically
  assign different types of bug submissions to different specialty departments
  (UI, Backend, etc).

- **Automated Severity Classification:** This is still a work in progress, but
this attempts to classify each bug report as LOW, MEDIUM, OR HIGH. This tends
to be biased towards HIGH...

- **Automatic Recommendations to Users:** When a user submits a bug, the bug report
is inspected. After the inspection, a list of simple things the user can try in
the mean time to resolve the bug are generated and displayed. This way, the
user may arrive at a temporary solution until the issue is fully fixed.

### How ServiceNow Is Used as a Back-End
ServiceNow has a very verbose REST API, maybe too verbose given the
documentation is over 7000 pages long. In comparison, the scope of this program
is very small and we will only be using the Table API. The Table API allows for
requesting full tables, table rows, modifying tables, etc. Below is an
example curl request to get a table as a json format:

```bash
curl "https://<DEV_INSTANCE>.service-now.com/api/now/table/<TABLE_NAME>?sysparm_limit=<REQUEST_AMOUNT>" \
--request GET \
--header "Accept:application/json" \
--user '<USER>':'<PASS>'
```

### Encrypted API Key with WASM
**THIS IS A BAD IDEA, DO NOT USE IN PRODUCTION. YOU HAVE BEEN WARNED**

Disclaimer and practicality aside, this is a interesting idea. Most of the
time, API keys are handled as ENV variables on the server side. So if the client
needs to make a REST request, the server must act like a "proxy" for the
client; taking the clients request, and forwarding it to the appropriate route
so long as they have proper credentials. This raises the question, can we have
client-side auth? Well, technically yes, we can serve our API keys encrypted with
a passcode and authenticate with a hash of the passcode. You then have to make
the assumption that your passcode and API-keys are perfectly random. The world
is not perfect however... Since I am using ServiceNow, the allowed usernames
and passwords are not very complex, and someone with a half-way decent GPU
could probably crack the hash in no time. But once again, for a simple demo, it
does not matter much. So what does the actual encryption architecture look like?
The passcode hash follows the SHA512 standard and the actual encryption of the
keys is a rolling XOR function. This is not the best setup, but it's what I thought
up in the meantime. If you want to see how this works in more depth, see `build.rs`
and `./wasm_env_crypt/src/lib.rs`. 

**Update:** I also realized another point of vulnerability is that someone could
record the packets being sent and find out the tokens that way. Once again,
cool idea, don't use in production.

# Building
This was built on NixOS. My Nix packages are not the most pure, so it will be a
minute before I have full nix flake solution running. But you should mostly
need `rust`, `wasm-pack`, and `python3 or some other http server for testing`.
The important thing after those dependencies is to ensure your environment
variables are properly set (see next section).

### Setting passcode on build
Set the env variable `WASM_ENV_CRYPT` to the passcode you want before building.
Make sure that your openai api key and servicenow keys are also exported...

```bash
export WASM_ENV_CRYPT="<PASSCODE HERE>"
export OPENAI_API_KEY="<YOUR API KEY>"
export SERVICENOW_USER="<YOUR API KEY>"
export SERVICENOW_PASS="<YOUR API KEY>"
...
```

### Notes For Me
See: [https://support.servicenow.com/kb?id=kb_article_view&sysparm_article=KB0831585]
