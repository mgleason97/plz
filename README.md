# plz
OpenAI API wrapper to get help from the terminal

Inspired by [sgpt](https://github.com/trstruth/sgpt) :)

## Installation
Prereqs:
* cargo

```bash
git clone https://github.com/mgleason97/plz.git
cargo install --path ./plz
```

## Usage
Make sure your OpenAI API key is exported.
```bash
export OPENAI_API_KEY=<yourapikey>
```

Get help with `plz -- your query`

E.g.
`plz -- k8s how to view pod logs`
