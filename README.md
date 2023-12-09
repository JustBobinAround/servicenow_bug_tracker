# ServiceNow Bug Tracker

### Setting passcode on build
Set the env variable `WASM_ENV_CRYPT` to the passcode you want before building.
Make sure that your openai api key and servicenow keys are also exported...

```bash
export WASM_ENV_CRYPT="<PASSCODE HERE>"
export OPENAI_API_KEY="<YOUR API KEY>"
...
```

### Notes
See: [https://support.servicenow.com/kb?id=kb_article_view&sysparm_article=KB0831585]
