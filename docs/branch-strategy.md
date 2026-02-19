# Branching Strategy

## Structure

Branches follow the pattern: `<module>/<type>/<description>`

```
<module>/<type>/<description>
```

### Modules

| Module | Description |
|---|---|
| `schema` | Rust validation layer (`bastion-schema`) |
| `core` | Rust HTTP gateway and pipeline (`bastion-core`) |
| `dashboard` | Django admin and REST API |
| `infra` | CI/CD, Docker, deployment config |

### Types

| Type | When to use |
|---|---|
| `feature` | New functionality |
| `fix` | Bug fixes |
| `experimental` | Proofs of concept, not guaranteed to merge |
| `chore` | Refactors, dependency updates, cleanup |

### Hotfixes

Hotfixes bypass the module prefix and branch directly from `main`:

```
hotfix/<description>
```

---

## Integration Branches

Each module has a staging branch for ongoing work before merging to `main`:

```
schema/dev
core/dev
dashboard/dev
```

Feature branches merge into the module's `dev` branch first. Only stable, tested code is promoted to `main`.

---

## Naming Conventions

- Use **kebab-case** for descriptions
- Keep descriptions **short and specific** (2–4 words)
- Use **imperative verbs** when describing actions

---

## Examples

```
schema/feature/add-regex-validation
schema/fix/numeric-range-overflow
schema/chore/refactor-error-types

core/feature/axum-http-server
core/feature/micro-batching-backpressure
core/experimental/actor-model-poc
core/fix/arrow-ipc-serialization

dashboard/feature/schema-crud-api
dashboard/fix/auth-token-expiry

infra/chore/add-github-actions-ci

hotfix/kafka-producer-deadlock
```

---

## Workflow

```
main
 └── schema/dev
 │    └── schema/feature/add-regex-validation
 │    └── schema/fix/numeric-range-overflow
 └── core/dev
 │    └── core/feature/axum-http-server
 │    └── core/experimental/actor-model-poc
 └── dashboard/dev
      └── dashboard/feature/schema-crud-api
```

1. Branch off the module's `dev` branch
2. Open a PR into the module's `dev` branch when ready
3. Promote `<module>/dev` → `main` once stable
4. `hotfix/*` branches from `main` and merges back into both `main` and affected `dev` branches

