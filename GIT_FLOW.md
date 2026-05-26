# Git Flow

Este proyecto sigue **Git Flow** con versionado semántico automático.

## Ramas

| Rama | Propósito | Base |
|---|---|---|
| `main` | Producción. Cada merge aquí dispara una release automática. | — |
| `development` | Integración de features en curso. | `main` |
| `feature/*` | Nuevas funcionalidades. | `development` |
| `hotfix/*` | Correcciones urgentes a producción. | `main` |

## Flujo diario

### Features

```bash
# 1. Parte de development
git checkout development
git pull origin development

# 2. Crea rama feature
git checkout -b feature/mi-feature

# 3. Trabaja con conventional commits
git commit -m "feat: add dark mode toggle"
git commit -m "fix: header overflow on mobile"
git commit -m "refactor: extract theme parser"
#  ¡NO version bump! Eso lo hace CI automáticamente.

# 4. PR: feature/mi-feature → development
git push origin feature/mi-feature
# Crear Pull Request en GitHub
```

### Hotfixes

```bash
# 1. Parte de main
git checkout main
git pull origin main

# 2. Crea rama hotfix
git checkout -b hotfix/arreglo-critico

# 3. Commits con conventional commits
git commit -m "fix: crash on empty file"

# 4. PR: hotfix/arreglo-critico → main
git push origin hotfix/arreglo-critico
# Crear Pull Request en GitHub

# 5. Después del merge, sincronizar development
git checkout development
git merge main
git push origin development
```

### Releases

```bash
# 1. Cuando development está listo para producción
#    Crear PR: development → main

# 2. Al mergear el PR en main, CI automáticamente:
#    a) Detecta si el bump es patch/minor/major según conventional commits
#    b) Ejecuta vampus para actualizar Cargo.toml
#    c) Genera CHANGELOG.md con git-cliff
#    d) Crea tag vX.Y.Z
#    e) Hace push del tag → dispara release.yml
#    f) GitHub Actions compila 5 targets
#    g) Crea GitHub Release con los binarios
#    h) Publica en crates.io
```

## Conventional Commits

El formato de los mensajes determina el bump automático:

| Mensaje | Bump | Ejemplo |
|---|---|---|
| `feat: ...` | **minor** (0.Y.0) | `feat: add image viewer support` |
| `feat(scope): ...` | **minor** (0.Y.0) | `feat(render): add table alignment` |
| `fix: ...` | **patch** (0.0.Z) | `fix: crash on empty input` |
| `refactor: ...` | **patch** (0.0.Z) | `refactor: extract parser` |
| `chore: ...` | **patch** (0.0.Z) | `chore: update dependencies` |
| `docs: ...` | **patch** (0.0.Z) | `docs: fix typos` |
| `feat!: ...` | **major** (X.0.0) | `feat!: redesign API` |
| `BREAKING CHANGE` | **major** (X.0.0) | any commit with `BREAKING CHANGE:` in body |

### Prefijos válidos

`feat:` · `fix:` · `refactor:` · `chore:` · `docs:` · `style:` · `test:` · `perf:` · `ci:` · `build:` · `revert:`

Añadir `!` después del prefijo para breaking changes: `feat!:` o `fix!:`.

## Protección de ramas

Configurado vía GitHub API:

| Rama | Push directo | Status checks | Notas |
|---|---|---|---|
| `main` | Solo admins y GitHub Actions | ✅ `test` (strict) | Merge solo por PR, CI obligatorio |
| `development` | Permitido | ✅ `test` | — |

### vampus

Gestiona el versionado en `Cargo.toml` y otros archivos.

```bash
vampus --patch      # 0.1.3 → 0.1.4
vampus --minor      # 0.1.3 → 0.2.0
vampus --major      # 0.1.3 → 1.0.0
vampus --preview --patch   # muestra el resultado sin aplicar
```

Config: `.vampus.yml`

### git-cliff

Genera `CHANGELOG.md` a partir de conventional commits.

```bash
git-cliff -o CHANGELOG.md                  # regenera changelog completo
git-cliff --tag v0.2.0 -o CHANGELOG.md     # para una versión específica
git-cliff --bumped-version                 # muestra la próxima versión
```

Config: `cliff.toml`

## CI Workflows

### `release-prepare.yml` (push a main)

Detecta bump type, ejecuta vampus, genera changelog, crea tag, sincroniza `development` con `main`.

### `release.yml` (push tag v*)

Compila para 5 targets, publica en crates.io, crea GitHub Release.

### `ci.yml` (PR a main/development, push a development)

Verifica formato, lint, build y tests.

## Resumen visual

```
main        ──hotfix──●────────────●────────────────●
                       \            /                /
development ──────────●──●──●────●──●──●──●────────●
                        \ /        \   /
feature     ──────────●  feature ──●
                      feature/foo   feature/bar

● = merge a development (PR normal)
● = merge a main (release auto)
```

## Preguntas frecuentes

**¿Debo hacer version bump manualmente?**  
No. El CI lo hace automáticamente al mergear a main. Solo preocúpate de escribir conventional commits correctos.

**¿Qué pasa si necesito un release sin cambios de código?**  
Por ejemplo, para re-ejecutar CI. No debería ocurrir, pero si es necesario, puedes crear un commit vacío en main con cualquier mensaje que no empiece por "chore: release".

**¿Cómo sincronizo development con main después de un hotfix?**  
```bash
git checkout development
git merge main
git push origin development
```

**¿Dónde se configura la versión actual?**  
En `.vampus.yml` → `current_version`. vampus lo actualiza automáticamente al hacer bump.

**Secretos de GitHub necesarios**  

| Secreto | Propósito |
|---|---|
| `GH_PAT` | Personal Access Token con scope `contents: write` y `workflows: write` |
| `CARGO_REGISTRY_TOKEN` | Token de API de crates.io para `cargo publish` |