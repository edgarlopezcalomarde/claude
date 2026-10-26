# cluse

Barra de estado para Claude Code que muestra el consumo de tu suscripción sin tener que mirar la web:

```
5h ███████ 68% ↻ 0h41m │ 7d ████ 41%
```

Cada límite es una barra de 10 celdas que cambia de color al cruzar el 50 % (ámbar) y el 80 % (rojo). `↻` es el tiempo que falta para que se reinicie la ventana de 5 horas.

## Instalación

Necesitas [Rust](https://rustup.rs) (el binario se compila en tu máquina).

Dentro de Claude Code:

```
/plugin marketplace add edgarlopezcalomarde/claude
/plugin install cluse@edgar-plugins
/cluse:setup
```

`/cluse:setup` compila e instala `cluse` con `cargo install` y añade esto a `~/.claude/settings.json` (si ya tienes otra `statusLine`, te pregunta antes de reemplazarla):

```json
"statusLine": { "type": "command", "command": "cluse" }
```

Reinicia Claude Code para ver la barra.

## Instalación manual

```
cargo install --git https://github.com/edgarlopezcalomarde/claude cluse
```

y añade la `statusLine` de arriba a tu `settings.json`.

## Notas

- Lee `rate_limits.five_hour` y `rate_limits.seven_day` del JSON que Claude Code envía por stdin. Esos campos solo existen para suscripciones Pro/Max y en versiones recientes; si faltan, la barra muestra `uso n/d`.
- Un plugin no puede activar `statusLine` por sí solo (solo admite `agent` y `subagentStatusLine`), por eso hace falta `/cluse:setup`.
