# Configuration (.vampus.yml)

Vampus uses a `.vampus.yml` file in the current working directory.

## Structure

```yaml
current_version: 0.4.8
replaces:
  - file: Cargo.toml
    pattern: ^version\s*=\s*"{{current_version}}"$
  - file: package.json
    pattern: ^  "version": "{{current_version}}",
```

- `current_version`: The current semver version
- `replaces`: List of search/replace rules for files containing the version string

## Pattern format

The placeholder `{{current_version}}` is used in the search pattern. It is automatically wrapped with capture groups so the replacement preserves the surrounding text.

Characters `( ) [ ] { }` in the pattern are escaped for regex safety.

## Multiple patterns per file

You can define multiple `replaces` entries pointing to the same file. Each replacement is applied sequentially, chaining the changes.

## Configuration files

Vampus does not search for `.vampus.yml` in parent directories — it always reads it from the current working directory.

Use `vampus init` to generate a default configuration.
