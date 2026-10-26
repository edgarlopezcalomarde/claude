---
name: setup
description: Instala el binario cluse y lo activa como status line de Claude Code.
disable-model-invocation: true
---

# cluse setup

1. Comprueba que existe `cargo` (`cargo --version`). Si no, para y dile al usuario que instale Rust desde https://rustup.rs.
2. Compila e instala el binario: `cargo install --path "${CLAUDE_PLUGIN_ROOT}" --locked --force`.
3. Lee `~/.claude/settings.json`.
   - Si ya tiene una `statusLine` distinta de `cluse`, enséñasela al usuario y pregunta antes de reemplazarla.
   - Si no, añade `"statusLine": { "type": "command", "command": "cluse" }` sin tocar el resto del fichero.
4. Dile al usuario que reinicie Claude Code para ver la barra.
