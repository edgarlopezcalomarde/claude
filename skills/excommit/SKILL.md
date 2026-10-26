---
name: excommit
description: Create a commit, bump the project version, write a good message and push. Use when the user says "excommit", "commit y push", "commitea y sube".
---

# excommit

1. Ejecuta `git status` (y `git diff` para entender los cambios). Si no hay cambios, para y avísalo.
2. Añade los cambios al stage: `git add -A`. No incluyas secretos (.env, claves); si los ves, para y pregunta.
3. Actualiza la versión del proyecto según los cambios (semver):
   - fix/ajustes → patch · nueva funcionalidad → minor · cambio incompatible → major.
4. Si es un proyecto web (hay `package.json`): `npm version <patch|minor|major> --no-git-tag-version`
   (actualiza `package.json` y el lockfile). Si no, sube la versión en el fichero que la tenga
   (`pyproject.toml`, `Cargo.toml`, etc.); si no hay ninguno, sáltalo. Vuelve a hacer `git add -A`.
5. Crea el commit con un mensaje corto y descriptivo (estilo conventional commits si el repo ya lo usa;
   mira `git log --oneline -5`).
   - NO añadas `Co-Authored-By: Claude` ni ninguna línea de atribución.
   - NUNCA uses `git commit --amend`.
6. Push a la rama actual: `git push`; si no tiene upstream, `git push -u origin HEAD`.
7. Informa: versión nueva, mensaje del commit y rama.
