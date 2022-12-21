# star-ls
star-ls is a language server for the Starlark programming language, used by the Bazel and Buck build systems.

## Planned Features
- type checking
- general autocomplete/goto-definition
- Bazel-specific features
    - target autocompletion/goto-definition
    - support Bazel-specific types

## Credits
Much of the code here was inspired by rust-analyzer. Special thanks to matklad and his [Explaining rust-analyzer](https://www.youtube.com/watch?v=I3RXottNwk0&list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) series, which really helped me understand how language servers work!
