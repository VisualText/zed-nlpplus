# NLP++ for Zed

[![PyPI: NLPPlus](https://img.shields.io/pypi/v/NLPPlus?label=PyPI%20NLPPlus&logo=pypi&logoColor=white)](https://pypi.org/project/NLPPlus/) [![npm: nlpplus](https://img.shields.io/npm/v/nlpplus?label=npm%20nlpplus&logo=npm)](https://www.npmjs.com/package/nlpplus)

[![CI](https://github.com/VisualText/zed-nlpplus/actions/workflows/ci.yml/badge.svg)](https://github.com/VisualText/zed-nlpplus/actions/workflows/ci.yml)

[NLP++](https://visualtext.org) support for the [Zed](https://zed.dev) editor.

| Language | Files |
|---|---|
| NLP++ | Pass files: `*.nlp`, `*.pat` |
| NLP++ Dictionary | `*.dict` |
| NLP++ Knowledge Base | `*.kbb` |
| NLP++ Analyzer Sequence | `analyzer.seq` |
| NLP++ Parse Tree | `*.tree` |

All five get syntax highlighting from
[tree-sitter-nlpplus](https://github.com/VisualText/tree-sitter-nlpplus). Pass files also get
bracket matching, an outline of their regions, rules and functions, and the
[NLP++ language server](https://www.npmjs.com/package/nlpplus-language-server). The language server
provides hover, go-to-definition, references, rename, completion, diagnostics and formatting.

The extension uses `nlpplus-language-server` from your `PATH` if you have installed it
(`npm i -g nlpplus-language-server`). Otherwise it installs the server from npm.

## Developing

Install Rust with [rustup](https://rustup.rs). Then, in Zed, run **zed: install dev extension**
and choose this directory.

## License

MIT
