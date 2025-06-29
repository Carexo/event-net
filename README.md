# event-net

**Created by**:
 - Bartosz Czyż
 - Marceli Mietła

## How to run

1. Copy `.env.example` as `.env` and change values if needed.

2. Run command in the app's root folder (where compose.yaml file is):

> [!WARNING]
> You have to had installed docker

```
docker compose up --build
```

Or to auto reload after file change:

```
docker compose up --build --watch
```
## Data model

> [!NOTE]
> Properties with (*) are required and uniqe

![data model](./graph.svg)

## Importing example data

1. Run command in app's root folder (where compose.yaml file is):
```bash
docker compose exec neo4j sh
cypher-shell -u $DB_USER -p $DB_PASSWORD --file /import/import.cypher
```

## Creating node embedding and knn

1. Run command in app's root folder (where compose.yaml file is):
```bash
docker compose exec neo4j sh
cypher-shell -u $DB_USER -p $DB_PASSWORD --file /import/frp.cypher
```