## Rust Projects

### Building the WASM projects
- From the root directory, run `./scripts/build-wasm.sh <scope>` where `<scope>` is the scope name of the package,

### Using WASM packages
- Run `./scripts/create-global-links.sh`, this will create global npm packages under the scope specified when running build script.
- Run `npm link @<scope>/<package-name>` in project directory. Then `import {...} from @<scope>/<package-name>` in project files.

### Gotchas
- Projects that import the packages must be using a bundler that supports WASM modules. E.g. Webpack 5 with experimental features.