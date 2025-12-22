# TODO HIGHLIGHT for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://img.shields.io/codecov/c/github/placintaalexandru/zed-todo-highlighter?logo=codecov&label=Coverage)](https://app.codecov.io/gh/placintaalexandru/zed-todo-highlighter)
![CodeQL](https://github.com/placintaalexandru/todo-highlighter/workflows/CodeQL/badge.svg)
![Dependabot enabled](https://img.shields.io/badge/Dependabot-Enabled-brightgreen?logo=dependabot)
![GitHub release (with filter)](https://img.shields.io/github/v/release/placintaalexandru/todo-highlighter?logo=github&label=Release&color=brightgreen)

<img align="right" width="200" height="200" src="./assets/logo.jpg">

Zed extension to highlight `TODO`s and other keywords in your code.

<br>
<br>
<br>
<br>

### Preview

- one keyword per line
  ![Logo](assets/one-label-per-line.png)

- multiple keywords per line (in this case every keyword's patch will be highlighted using keyword's color)
  ![Logo](assets/two-labels-on-the-same-line.png)

### Config

In order to highlight the keywords, you need to allow Zed to draw background colors:

```json
{
    "lsp_document_colors": "background"
}
```

`TODO` is a built-in keyword. You can override the look by customizing extension's settings.

To customize which keywords to highlight and their appearance, you can use local project's `.zed/settings.json` or global Zed's `settings.json` as long as the LSP server is enabled and configured:

```json
"lsp": {
  "todo-highlight-lsp": {
    "initialization_options": {
      "highlights": {
        "TODO": {
          "background": "#81ff81"
        },
        "FIXME": {
          "background": "#ffff81"
        },
        "BUG": {
          "background": "#ff8181"
        }
      },
    },
  },
}
```

The color can be a RGB or a RGBA value, so the following is also accepted:

```json
"lsp": {
  "todo-highlight-lsp": {
    "initialization_options": {
      "highlights": {
        "TODO": {
          "background": "#81ff811b"
        }
      }
    }
  }
}
```

### Limitations

1. Only background color is supported since it's the only feature supported by the LSP server. To add foreground color requires interaction with the editor and there is no Zed API at the moment.
2. Not possible to show the line number of the highlighted keyword since extensions cannot modify the editor's UI.
