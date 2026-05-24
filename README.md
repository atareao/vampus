# 🧛‍♂️ Vampus

[![Crates.io](https://img.shields.io/crates/v/vampus?style=flat-square)](https://crates.io/crates/vampus)
[![License](https://img.shields.io/crates/l/vampus?style=flat-square)](#license)
[![Rust](https://img.shields.io/badge/rust-2024-orange?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/vampus/ci.yml?branch=main&style=flat-square)](https://github.com/yourusername/vampus/actions)
[![Coverage](https://img.shields.io/codecov/c/github/yourusername/vampus?style=flat-square)](https://codecov.io/gh/yourusername/vampus)

> **Una herramienta de línea de comandos moderna y asíncrona para gestión de versiones semánticas**

Vampus es una herramienta CLI potente y eficiente escrita en **Rust** que automatiza completamente el proceso de versionado semántico (SemVer) en tus proyectos, proporcionando capacidades avanzadas de actualización, retroceso y previsualización de versiones.

---

## ✨ Características Principales

- 🚀 **Alto Rendimiento**: Construido sobre Tokio para operaciones I/O asíncronas ultrarrápidas
- 📦 **Versionado Semántico**: Soporte completo para incrementos/decrementos SemVer (`patch`, `minor`, `major`)
- ⏪ **Retroceso Inteligente**: Comando `downgrade` para revertir versiones de manera segura
- 👁️ **Previsualización**: Comando `preview` para validar cambios antes de aplicarlos
- 🎯 **Multi-archivo**: Actualiza versiones consistentemente en múltiples archivos de configuración
- ⚙️ **Configuración Flexible**: Archivo `.vampus.yml` para personalizar patrones y archivos objetivo
- 🛡️ **Operaciones Transaccionales**: Verificación previa para prevenir estados inconsistentes
- 🎨 **Interfaz Moderna**: CLI intuitiva con mensajes informativos y coloridos

---

## 🚀 Instalación Rápida

### Desde Crates.io (Recomendado)

```bash
cargo install vampus
```

### Desde el Código Fuente

```bash
# Clonar el repositorio
git clone https://github.com/yourusername/vampus.git
cd vampus

# Compilar e instalar
cargo install --path .
```

### Desde Releases (Binarios Precompilados)

Descarga el binario para tu plataforma desde [GitHub Releases](https://github.com/yourusername/vampus/releases).

---

## 📖 Guía de Uso

### 1. Inicialización del Proyecto

```bash
# Crear archivo de configuración por defecto
vampus init
```

Esto crea un archivo `.vampus.yml` con la configuración inicial:

```yaml
current_version: 0.1.0
replaces:
  - file: Cargo.toml
    pattern: ^version\s*=\s*"{{current_version}}"$
```

### 2. Comandos Principales

#### Actualización de Versiones

```bash
# Incrementar versión patch (0.1.0 → 0.1.1)
vampus upgrade --patch

# Incrementar versión minor (0.1.0 → 0.2.0)
vampus upgrade --minor

# Incrementar versión major (0.1.0 → 1.0.0)
vampus upgrade --major
```

#### Retroceso de Versiones

```bash
# Decrementar versión patch (0.1.1 → 0.1.0)
vampus downgrade --patch

# Decrementar versión minor (0.2.0 → 0.1.0)
vampus downgrade --minor

# Decrementar versión major (1.0.0 → 0.1.0)
vampus downgrade --major
```

#### Previsualización

```bash
# Ver la próxima versión sin aplicar cambios
vampus preview --minor
# Output: Current version: 0.1.0
#         Preview version (Increment): 0.2.0
```

#### Información

```bash
# Mostrar la versión actual del proyecto
vampus show
# Output: 0.1.0
```

---

## ⚙️ Configuración

### Archivo `.vampus.yml`

```yaml
# Versión actual del proyecto
current_version: 1.2.3

# Archivos y patrones a actualizar
replaces:
  # Cargo.toml para proyectos Rust
  - file: Cargo.toml
    pattern: ^version\s*=\s*"{{current_version}}"$

  # package.json para proyectos Node.js
  - file: package.json
    pattern: "version"\s*:\s*"{{current_version}}"

  # Archivos personalizados
  - file: src/version.rs
    pattern: pub const VERSION\s*:\s*&str\s*=\s*"{{current_version}}";
```

### Patrones de Búsqueda

Los patrones utilizan expresiones regulares de Rust. El marcador `{{current_version}}` se reemplaza automáticamente por la versión actual/nueva.

**Ejemplos de patrones comunes:**

| Tipo de Archivo   | Patrón de Ejemplo                           |
| ----------------- | ------------------------------------------- |
| `Cargo.toml`      | `^version\s*=\s*"{{current_version}}"$`     |
| `package.json`    | `"version"\s*:\s*"{{current_version}}"`     |
| Python `setup.py` | `version\s*=\s*["']{{current_version}}["']` |
| C/C++ Header      | `#define\s+VERSION\s+"{{current_version}}"` |

---

## 🏗️ Arquitectura y Tecnologías

### Stack Tecnológico

- **Lenguaje**: Rust 2024 Edition
- **Runtime Asíncrono**: [Tokio](https://tokio.rs/) para operaciones I/O no bloqueantes
- **CLI Framework**: [Clap](https://clap.rs/) con derive macros
- **Regex Engine**: [regex](https://docs.rs/regex/) para patrones de búsqueda
- **Serialización**: [Serde](https://serde.rs/) + [serde_yaml](https://docs.rs/serde_yaml/) para configuración
- **Logging**: [tracing](https://tracing.rs/) para observabilidad avanzada

### Características de Rendimiento

- ⚡ **Operaciones Asíncronas**: Procesamiento paralelo de múltiples archivos
- 🧠 **Gestión Inteligente de Memoria**: Zero-copy string processing donde es posible
- 🔒 **Seguridad de Tipos**: Aprovecha el sistema de tipos de Rust para prevenir errores
- 🎯 **Optimizaciones del Compilador**: Compilación con `--release` para máximo rendimiento

---

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Por favor, lee nuestras [pautas de contribución](CONTRIBUTING.md) antes de comenzar.

### Desarrollo Local

```bash
# Clonar el repositorio
git clone https://github.com/yourusername/vampus.git
cd vampus

# Instalar dependencias de desarrollo
cargo install cargo-watch

# Ejecutar tests
cargo test

# Desarrollo con auto-reload
cargo watch -x run

# Verificar formato y linting
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

### Roadmap

- [ ] Soporte para más formatos de archivo (TOML, XML, etc.)
- [ ] Integración con sistemas de control de versiones (Git tags automáticos)
- [ ] Plugin system para extensibilidad
- [ ] Interfaz web opcional para equipos
- [ ] Soporte para monorepos
- [ ] Validación de dependencias semánticas

---

## 📄 Licencia

Este proyecto está licenciado bajo la [MIT License](LICENSE) - consulta el archivo LICENSE para más detalles.

---

## 🙏 Agradecimientos

- Equipo de [Rust](https://www.rust-lang.org/) por crear un lenguaje increíble
- [Tokio](https://tokio.rs/) por el runtime asíncrono
- [Clap](https://clap.rs/) por el framework CLI
- Toda la comunidad de Rust por las bibliotecas de calidad

---

<div align="center">

**[Documentación](https://docs.rs/vampus)** •
**[Ejemplos](examples/)** •
**[Reportar Bug](https://github.com/yourusername/vampus/issues)** •
**[Solicitar Característica](https://github.com/yourusername/vampus/issues/new?template=feature_request.md)**

Hecho con ❤️ y ☕ por la comunidad

</div>
