#  Dokumentacja Backendu (Rust) 🚀

## Architektura

Backend aplikacji został zbudowany w języku **Rust** z wykorzystaniem nowoczesnych i wydajnych bibliotek, co gwarantuje wysoką niezawodność i szybkość działania.

-   **Framework webowy:** [Rocket](https://rocket.rs/)
-   **Klient bazy danych Neo4j:** [neo4rs](https://docs.rs/neo4rs/latest/neo4rs/)
-   **Runtime asynchroniczny:** [Tokio](https://tokio.rs/)
-   **Serializacja/deserializacja JSON:** [Serde](https://serde.rs/)

System opiera się na klasycznym podziale odpowiedzialności, inspirowanym architekturą warstwową:

-   **Model:** Struktury danych (`struct`) reprezentujące encje w bazie (np. `Event`, `User`).
-   **Controller:** Funkcje obsługujące endpointy API, odpowiedzialne za przyjmowanie żądań i zwracanie odpowiedzi.
-   **Service:** Warstwa logiki biznesowej, gdzie realizowane są operacje na danych.
-   **Repo:** Moduł odpowiedzialny za bezpośrednią interakcję z bazą danych Neo4j.

---

## Struktury Danych 💾

### Event

```rust
/// Reprezentuje pojedyncze wydarzenie w systemie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unikalny identyfikator wydarzenia.
    pub id: i32,
    /// Nazwa wydarzenia.
    pub name: String,
    /// Data i czas rozpoczęcia wydarzenia w formacie ISO 8601.
    pub start_datetime: String,
    /// Lista słów kluczowych powiązanych z wydarzeniem.
    pub keywords: Vec<String>,
}

/// Struktura używana przy tworzeniu nowego wydarzenia.
#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub name: String,
    pub start_datetime: String,
    pub keywords: Vec<String>,
}

/// Struktura używana przy aktualizacji istniejącego wydarzenia.
/// Wszystkie pola są opcjonalne.
#[derive(Debug, Deserialize)]
pub struct UpdateEventRequest {
    pub name: Option<String>,
    pub start_datetime: Option<String>,
    pub keywords: Vec<String>,
}
```

### User

```rust
/// Reprezentuje użytkownika systemu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unikalna nazwa użytkownika (identyfikator).
    pub name: String,
    /// Opcjonalny adres email użytkownika.
    pub email: Option<String>,
}

/// Reprezentuje relację zapisu użytkownika na wydarzenie.
#[derive(Debug, Serialize)]
pub struct UserEventRegistration {
    pub user_name: String,
    pub event_id: i32,
    pub is_attending: bool,
}
```

### Zarządzanie połączeniem z bazą Neo4j

Połączenie z bazą jest zarządzane przez dedykowaną strukturę `Neo4jConnection`, która wykorzystuje `Arc<Graph>` do bezpiecznego współdzielenia puli połączeń w środowisku asynchronicznym.

```rust
use neo4rs::{ConfigBuilder, Error, Graph};
use std::sync::Arc;

pub struct Neo4jConnection {
    pub graph: Arc<Graph>,
}

impl Neo4jConnection {
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self, Error> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .build()?;

        let graph = Arc::new(Graph::connect(config).await?);

        Ok(Neo4jConnection { graph })
    }
}
```

---

## Zapytania do Bazy Danych (Cypher) 🔍

Poniżej znajdują się kluczowe zapytania Cypher używane w aplikacji do zarządzania danymi w grafie Neo4j.

### Wydarzenia

#### 1. Pobranie wydarzenia po identyfikatorze
**Opis:** Znajduje konkretne wydarzenie na podstawie jego `id` i zwraca jego dane wraz z listą powiązanych słów kluczowych.
```cypher
MATCH (e:Event {id: $id})
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    collect(k.name) AS keywords
```
-   `OPTIONAL MATCH` zapewnia, że zapytanie zadziała nawet, jeśli wydarzenie nie ma żadnych słów kluczowych.
-   `collect(k.name)` agreguje nazwy wszystkich powiązanych słów kluczowych do jednej listy.

#### 2. Pobranie wszystkich wydarzeń
**Opis:** Zwraca listę wszystkich wydarzeń w bazie wraz z ich słowami kluczowymi.
```cypher
MATCH (e:Event)
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    collect(k.name) AS keywords
```

#### 3. Dodawanie nowego wydarzenia
**Opis:** Tworzy nowy węzeł `Event`, dynamicznie przydziela mu `id` (o 1 większe od maksymalnego istniejącego), a następnie tworzy lub łączy podane słowa kluczowe.
```cypher
// 1. Znajdź maksymalne istniejące ID i dodaj 1
MATCH (e:Event)
WITH COALESCE(MAX(e.id), 0) + 1 AS newId

// 2. Utwórz węzeł Event z nowym ID i danymi z parametrów
CREATE (e:Event {
    id: newId,
    name: $eventName,
    startDatetime: datetime($startDatetime)
})

// 3. Dla każdego słowa kluczowego z listy, utwórz węzeł (jeśli nie istnieje) i połącz z wydarzeniem
WITH e
UNWIND $keywords AS kw
MERGE (k:EventKeyword { name: kw })
MERGE (e)-[:HAS]->(k)

// 4. Zwróć dane nowo utworzonego wydarzenia
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    collect(k.name) AS keywords
```
-   `COALESCE(MAX(e.id), 0) + 1` bezpiecznie inkrementuje ID, startując od 1, jeśli baza jest pusta.
-   `MERGE` zapobiega tworzeniu duplikatów słów kluczowych.

#### 4. Usunięcie wydarzenia
**Opis:** Usuwa węzeł wydarzenia o podanym `id` oraz wszystkie jego relacje.
```cypher
MATCH (e:Event {id: $eventId})
DETACH DELETE e
```
-   `DETACH DELETE` kasuje węzeł wraz ze wszystkimi jego połączeniami, zapobiegając pozostawieniu "osieroconych" relacji.

#### 5. Edycja wydarzenia
**Opis:** Aktualizuje dane wydarzenia. Nadpisuje tylko te pola, które zostały przekazane w parametrach. Usuwa stare powiązania ze słowami kluczowymi i tworzy nowe na podstawie dostarczonej listy.
```cypher
// 1. Znajdź wydarzenie i zaktualizuj pola, jeśli nowe wartości nie są puste
MATCH (e:Event {id: $eventId})
SET
    e.name = coalesce($eventName, e.name),
    e.startDatetime = coalesce(datetime($startDatetime), e.startDatetime)
WITH e

// 2. Usuń stare relacje do słów kluczowych
OPTIONAL MATCH (e)-[oldRel:HAS]->(:EventKeyword)
DELETE oldRel
WITH e

// 3. Utwórz nowe relacje do słów kluczowych
UNWIND $keywords AS kw
MERGE (k:EventKeyword { name: kw })
MERGE (e)-[:HAS]->(k)

// 4. Zwróć zaktualizowane wydarzenie
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    collect(k.name) AS keywords
```
-   `coalesce()` pozwala na warunkową aktualizację – jeśli parametr jest `null`, zachowuje starą wartość.

#### 6. Pobranie wyróżnionych wydarzeń
**Opis:** Zwraca ograniczoną liczbę wydarzeń (np. 3), które mogą być użyte jako "polecane" lub "najnowsze".
```cypher
MATCH (e:Event)
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    collect(k.name) AS keywords
LIMIT 3
```

#### 7. Wyszukiwanie wydarzeń po słowach kluczowych
**Opis:** Filtruje wydarzenia, zwracając tylko te, które są powiązane ze *wszystkimi* słowami kluczowymi z podanej listy.
```cypher
MATCH (e:Event)-[:HAS]->(k:EventKeyword)
WITH e, COLLECT(k.name) AS keywords
WHERE all(kw IN $kws WHERE kw IN keywords)
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    keywords
```
-   `all(...)` działa jak predykat, który jest prawdziwy tylko wtedy, gdy wszystkie elementy z listy `$kws` znajdują się w zebranej liście `keywords` wydarzenia.

#### 8. Pobranie wszystkich słów kluczowych
**Opis:** Zwraca listę wszystkich unikalnych nazw słów kluczowych istniejących w bazie.
```cypher
MATCH (k:EventKeyword)
RETURN k.name
```

---

### Użytkownicy i Rejestracje

#### 1. Przypisanie użytkownika do wydarzenia
**Opis:** Tworzy relację `REGISTERED_TO` pomiędzy użytkownikiem a wydarzeniem, o ile taka relacja jeszcze nie istnieje.
```cypher
MATCH (u:User {name: $userName})
MATCH (e:Event {id: $eventId})
MERGE (u)-[:REGISTERED_TO]->(e)
```

#### 2. Usunięcie przypisania użytkownika do wydarzenia
**Opis:** Usuwa relację `REGISTERED_TO` między użytkownikiem a wydarzeniem.
```cypher
MATCH (u:User {name: $userName})-[r:REGISTERED_TO]->(e:Event {id: $eventId})
DELETE r
```

#### 3. Pobranie wszystkich wydarzeń danego użytkownika
**Opis:** Zwraca listę wydarzeń, na które zapisał się dany użytkownik.
```cypher
MATCH (:User {name: $userName})-[:REGISTERED_TO]->(e:Event)
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
    e.id            AS eventId,
    e.name          AS eventName,
    e.startDatetime AS start,
    collect(k.name) AS keywords
```

#### 4. Sprawdzenie, czy użytkownik jest zarejestrowany na wydarzenie
**Opis:** Zwraca `true` lub `false` w zależności od tego, czy istnieje relacja `REGISTERED_TO` między użytkownikiem a wydarzeniem.
```cypher
MATCH (u:User {name: $userName}), (e:Event {id: $eventId})
RETURN EXISTS((u)-[:REGISTERED_TO]->(e)) AS isAttending
```

---

## System Rekomendacji ✨

### 1. Rekomendacja na podstawie podobieństwa słów kluczowych (Jaccard)
**Opis:** System wyszukuje przyszłe wydarzenia, w których użytkownik jeszcze nie bierze udziału. Rekomendacje opierają się na **współczynniku podobieństwa Jaccarda** między zbiorami słów kluczowych wydarzeń, na które użytkownik jest już zapisany, a innymi wydarzeniami. Zwracane są tylko te wydarzenia, których współczynnik podobieństwa przekracza próg `0.5`.

```cypher
// 1. Znajdź wydarzenia, na które zapisany jest użytkownik (u)
MATCH (u:User {name: $userName})-[:REGISTERED_TO]->(e:Event)

// 2. Znajdź inne, przyszłe wydarzenia (other), w których użytkownik nie uczestniczy
MATCH (e)-[:HAS]->(k:EventKeyword)<-[:HAS]-(other:Event)
WHERE other.startDatetime > datetime() AND NOT (u)-[:REGISTERED_TO]->(other)

// 3. Oblicz współczynnik Jaccarda
WITH other,
     // Zlicz wspólne słowa kluczowe (przecięcie zbiorów)
     count(k) AS intersection,
     // Zbierz słowa kluczowe z obu wydarzeń
     [(e)-[:HAS]->(ek) | ek.name] AS set1,
     [(other)-[:HAS]->(ok) | ok.name] AS set2
WITH other,
     // Oblicz Jaccard = |A ∩ B| / |A ∪ B|
     (1.0 * intersection) / size(set1 + [x IN set2 WHERE NOT x IN set1]) AS jaccard,
     set2 AS keywords

// 4. Odfiltruj wyniki poniżej progu i posortuj
WHERE jaccard > 0.5
RETURN
    other.id            AS eventId,
    other.name          AS eventName,
    other.startDatetime AS start,
    keywords
ORDER BY jaccard DESC
```

### 2. Rekomendacja z użyciem osadzeń grafowych (Graph Embeddings & KNN)
**Opis:** Bardziej zaawansowane podejście, które wykorzystuje uczenie maszynowe do znalezienia "podobnych" użytkowników. Proces polega na wygenerowaniu wektorowych reprezentacji (osadzeń) dla użytkowników i wydarzeń, a następnie znalezieniu użytkowników o najbardziej zbliżonych wektorach.

#### Krok 1: Projekcja grafu do pamięci GDS
Tworzymy wirtualny graf w pamięci, zawierający tylko użytkowników, wydarzenia i relacje `REGISTERED_TO`.

```cypher
CALL gds.graph.project(
    'registrations',
    ['User', 'Event'],
    {
        REGISTERED_TO: {
            orientation: 'UNDIRECTED'
        }
    }
)
```

#### Krok 2: Generowanie osadzeń (embeddings) metodą FastRP
Uruchamiamy algorytm FastRP, aby dla każdego węzła w grafie wygenerować wektor cech (embedding).

```cypher
CALL gds.fastRP.mutate(
    'registrations',
    {
        embeddingDimension: 256,
        iterationWeights: [0.8, 1, 1, 1],
        mutateProperty: 'embedding'
    }
)
```

#### Krok 3: Obliczenie podobieństw i zapisanie relacji `SIMILAR`
Algorytm KNN (k-najbliższych sąsiadów) oblicza podobieństwo między użytkownikami na podstawie ich osadzeń i tworzy nowe relacje `SIMILAR` między najbardziej podobnymi.

```cypher
CALL gds.knn.write(
    'registrations',
    {
        nodeLabels: ['User'],
        nodeProperties: ['embedding'],
        topK: 10,
        writeRelationshipType: 'SIMILAR',
        writeProperty: 'score'
    }
)
```

#### Krok 4: Rekomendacja wydarzeń na podstawie podobnych użytkowników
Finalne zapytanie wyszukuje wydarzenia, w których uczestniczą użytkownicy `podobni` do naszego, a w których nasz użytkownik jeszcze nie bierze udziału.

```cypher
// 1. Znajdź wydarzenia, w których uczestniczy docelowy użytkownik (u)
MATCH (u:User {name: $userName})-[:REGISTERED_TO]->(e:Event)
WITH u, COLLECT(DISTINCT e) AS userEvents

// 2. Znajdź podobnych użytkowników (similarUser) i ich wydarzenia (recEvent)
MATCH (u)-[:SIMILAR]->(similarUser)-[:REGISTERED_TO]->(recEvent:Event)

// 3. Odfiltruj wydarzenia, które już są na liście użytkownika i są przyszłe
WHERE recEvent.startDatetime > datetime() AND NOT recEvent IN userEvents

// 4. Zbierz unikalne rekomendacje i ich słowa kluczowe
WITH DISTINCT recEvent
OPTIONAL MATCH (recEvent)-[:HAS]->(k:EventKeyword)
RETURN
    recEvent.id AS eventId,
    recEvent.name AS eventName,
    recEvent.startDatetime AS start,
    collect(k.name) AS keywords
```
