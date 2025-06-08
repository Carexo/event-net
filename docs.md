# Backend dokumentacja (Rust)

## Architektura

Backend zbudowany jest w języku Rust z wykorzystaniem następujących bibliotek:
- **rocket** - framework webowy
- **neo4rs** - klient Neo4j dla Rust
- **tokio** - asynchroniczny runtime
- **serde** - serializacja/deserializacja JSON

Architektura:
- **Model** - struktury danych reprezentujące encje w bazie
- **Controller** - funkcje obsługujące endpointy API
- **Service** - logika biznesowa 
- **Repo** - interakcja z bazą danych

## Struktury danych

### Event

```rust
/// Reprezentuje pojedyncze wydarzenie w systemie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unikalny identyfikator wydarzenia
    pub id: i32,
    /// Nazwa wydarzenia
    pub name: String,
    /// Data i czas rozpoczęcia wydarzenia w formacie ISO 8601
    pub start_datetime: String,
    /// Lista słów kluczowych powiązanych z wydarzeniem
    pub keywords: Vec<String>,
}

/// Używane przy tworzeniu nowego wydarzenia
#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub name: String,
    pub start_datetime: String,
    pub keywords: Vec<String>,
}

/// Używane przy aktualizacji istniejącego wydarzenia
#[derive(Debug, Deserialize)]
pub struct UpdateEventRequest {
    pub name: Option<String>,
    pub start_datetime: Option<String>,
    pub keywords: Vec<String>,
}
```

### User

```rust
/// Reprezentuje użytkownika systemu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unikalna nazwa użytkownika
    pub name: String,
    /// Opcjonalny adres email
    pub email: Option<String>,
}

/// Reprezentuje relację użytkownika z wydarzeniem
#[derive(Debug, Serialize)]
pub struct UserEventRegistration {
    pub user_name: String,
    pub event_id: i32,
    pub is_attending: bool,
}
```

## Zarządzanie połaczeniem z baża Neo4j
```rust
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

# Zapytania do bazy danych

## Wydarzenia

1. Pobranie wydarzenia po identyfikatorze

To zapytanie znajduje dokładnie jedno wydarzenie o podanym id, zwraca jego podstawowe dane (id, nazwa, data rozpoczęcia) oraz wszystkie słowa kluczowe z nim powiązane. Idealne, gdy chcemy wyświetlić szczegóły konkretnego wydarzenia.

```cypher
MATCH (e:Event {id: $id})
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
e.id            AS eventId,
e.name          AS eventName,
e.startDatetime AS start,
collect(k.name) AS keywords;
```

- MATCH (e:Event {id: $id}) – wybór wydarzenia o danym kluczu
- OPTIONAL MATCH … HAS → k – pobranie słów kluczowych, ale bez błędu, jeśli nie ma ich w ogóle
- collect(k.name) – umieszcza wszystkie nazwy słów kluczowych w jednej liście

---

2. Pobranie wszystkich wydarzeń

Zwraca listę wszystkich wydarzeń w bazie wraz z ich słowami kluczowymi. Przydaje się np. do budowania widoku „Wszystkie wydarzenia” na froncie.

```angular2html

MATCH (e:Event)
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
e.id            AS eventId,
e.name          AS eventName,
e.startDatetime AS start,
collect(k.name) AS keywords;
```

- Przechodzi przez każdy węzeł Event
- Dla każdego zbiera powiązane słowa kluczowe

---

3. Dodawanie nowego wydarzenia

Tworzy węzeł Event z automatycznie przydzielonym id +1 względem naj­wyższego istniejącego, ustawia nazwę i datę, a następnie tworzy lub łączy słowa kluczowe (unikalne dzięki constraintowi) i łączy je relacją HAS. Na koniec zwraca pełne dane nowo utworzonego wydarzenia.

```cypher
// 1. Wyliczenie nowego ID
MATCH (e:Event)
WITH COALESCE(MAX(e.id), 0) + 1 AS newId

// 2. Utworzenie węzła Event
CREATE (e:Event {
id: newId,
name: $eventName,
startDatetime: datetime($startDatetime)
})

// 3. Powiązanie słów kluczowych
WITH e
UNWIND $keywords AS kw
MERGE (k:EventKeyword { name: kw })
MERGE (e)-[:HAS]->(k)

// 4. Zwrócenie utworzonego wydarzenia
RETURN
e.id            AS eventId,
e.name          AS eventName,
e.startDatetime AS start,
collect(k.name) AS keywords;
```

- COALESCE(MAX(e.id), 0)+1 – nawet jeśli nie ma żadnego wydarzenia, id zaczyna się od 1
- MERGE gwarantuje, że dla każdego słowa kluczowego powstanie tylko jeden węzeł

---

4. Usunięcie wydarzenia

Błyskawicznie usuwa węzeł wydarzenia o danym id wraz ze wszystkimi relacjami do innych węzłów (np. HAS czy REGISTERED_TO). Stosujemy DETACH DELETE, żeby nie zostawić „wiszących” relacji.

```
MATCH (e:Event {id: $eventId})
DETACH DELETE e;
```

- DETACH DELETE – kasuje i węzeł, i wszystkie połączenia z nim

---

5. Edycja wydarzenia

Wyszukuje wydarzenie, nadpisuje jego nazwę i/lub datę tylko jeśli przekazano nowe wartości, a jeśli nie — pozostawia stare. Następnie czyści wszystkie stare relacje do słów kluczowych i tworzy nowe na podstawie przekazanej listy.

```
MATCH (e:Event {id: $eventId})
SET
e.name          = coalesce($eventName, e.name),
e.startDatetime = coalesce(datetime($startDatetime), e.startDatetime)
WITH e
OPTIONAL MATCH (e)-[oldRel:HAS]->(oldK:EventKeyword)
DELETE oldRel
WITH e
UNWIND $keywords AS kw
MERGE (k:EventKeyword { name: kw })
MERGE (e)-[:HAS]->(k)
RETURN
e.id AS eventId,
e.name AS eventName,
e.startDatetime AS start,
collect(k.name) AS keywords;
```

- coalesce pozwala pominąć zmianę, gdy parametr jest null
- Usunięcie starych relacji gwarantuje, że po edycji nie zostanie „martwy” link do nieużywanego słowa

---

6. Pobranie wyróżnionych wydarzeń

Po prostu zwraca pierwsze trzy wydarzenia (kolejność zgodna z wewnętrznym porządkiem bazy), wraz ze słowami kluczowymi. Można to wykorzystać jako „polecane” lub „na górze listy”.

```
MATCH (e:Event)
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
e.id AS eventId,
e.name AS eventName,
e.startDatetime AS start,
collect(k.name) AS keywords
LIMIT 3;

    •	LIMIT 3 – ogranicza wynik do trzech rekordów

⸻

7. Wyszukiwanie wydarzeń po słowach kluczowych

Co robi?
Filtruje listę wszystkich wydarzeń i pozostawia tylko te, które mają w sobie wszystkie słowa kluczowe podsłane w parametrze $kws. Przydatne do wyszukiwania precyzyjnego: np. „tylko te z ['konferencja','tech']”.

MATCH (e:Event)-[:HAS]->(k:EventKeyword)
WITH e, COLLECT(k.name) AS keywords
WHERE all(kw IN $kws WHERE kw IN keywords)
RETURN
e.id AS eventId,
e.name AS eventName,
e.startDatetime AS start,
keywords;
```

- all(kw IN $kws WHERE kw IN keywords) – wymaga, by każdy element z listy $kws był obecny w keywords

---

8. Pobranie wszystkich słów kluczowych

Wyciąga wszystkie unikalne nazwy węzłów EventKeyword z grafu. Można nimi wypełnić dropdown z filtrami po słowach.

```
MATCH (k:EventKeyword)
RETURN k.name;
```

---

## Użytkownicy

1. Przypisanie użytkownika do wydarzenia

Tworzy relację REGISTERED_TO między istniejącym użytkownikiem a wybranym wydarzeniem, jeśli jeszcze jej nie ma. Umożliwia późniejsze sprawdzanie, kto się zapisał.

```
MATCH (u:User {name: $n})
MATCH (e:Event {id: $id})
MERGE (u)-[:REGISTERED_TO]->(e);
```

- MERGE – nie dodaje duplikatu relacji

---

2. Usunięcie przypisania użytkownika

Jeśli użytkownik był przypisany do wydarzenia, usuwa tę konkretną relację. Użyteczne przy wyrejestrowywaniu.

```
MATCH (u:User {name: $n})
MATCH (e:Event {id: $id})
MATCH (u)-[r:REGISTERED_TO]->(e)
DELETE r;
```

---

3. Pobranie wszystkich wydarzeń danego użytkownika

Zwraca pełną listę wydarzeń, na które użytkownik się zapisał, razem ze słowami kluczowymi każdego. Pozwala np. na wyświetlenie „Moje wydarzenia” w profilu.

```
MATCH (u:User {name: $n})
MATCH (u)-[:REGISTERED_TO]->(e:Event)
OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
RETURN
e.id AS eventId,
e.name AS eventName,
e.startDatetime AS start,
collect(k.name) AS keywords;
```

---

4. Sprawdzenie, czy użytkownik jest zarejestrowany

Zwraca jednobitową odpowiedź (true/false), czy istnieje relacja REGISTERED_TO między danym użytkownikiem a danym wydarzeniem. Użyteczne np. do sterowania przyciskiem „Zapisz/Wypisz się”.

```
MATCH (u:User {name: $n}), (e:Event {id: $id})
RETURN EXISTS((u)-[:REGISTERED_TO]->(e)) AS isAttending;
```

---

## Rekomendacje

1. Rekomendacja wydarzeń na podstawie podobieństwa

Dla każdego wydarzenia, na które użytkownik się zapisał, sprawdza inne (przyszłe) wydarzenia mające wspólne słowa kluczowe, oblicza miarę Jaccarda dla zbiorów słów i wybiera te, w których współczynnik podobieństwa przekracza 0.5. Wyniki sortuje malejąco po tym współczynniku — czyli najpierw te najbardziej „podobne”.

```
MATCH (u:User {name: $n})-[:REGISTERED_TO]->(e:Event)-[:HAS]->(k:EventKeyword)
<-[:HAS]-(other:Event WHERE other.startDatetime > datetime())
WHERE NOT EXISTS((u)-[:REGISTERED_TO]->(other))
WITH e, other, count(k) AS intersection,
[(e)-[:HAS]->(ek) | ek.name] AS set1,
[(other)-[:HAS]->(ok) | ok.name] AS set2
WITH other,
(1.0 \* intersection) / size(set1 + [x IN set2 WHERE NOT x IN set1]) AS jaccard,
set2 AS keywords
WHERE jaccard > 0.5
RETURN
other.id AS eventId,
other.name AS eventName,
other.startDatetime AS start,
keywords
ORDER BY jaccard DESC;
```

- intersection – ile słów wspólnych
- size(set1 ∪ set2) – rozmiar sumy zbiorów (potrzebny do Jaccard)
- jaccard > 0.5 – próg, który uznajemy za wystarczająco podobne

---

2. Rekomendacja wydarzeń na podstawie podobnych użytkowników

To zapytanie znajduje innych użytkowników, którzy są powiązani relacją SIMILAR z danym użytkownikiem, następnie zbiera wszystkie przyszłe wydarzenia, na które ci podobni użytkownicy się zapisali, ale w których oryginalny użytkownik jeszcze nie uczestniczy. Wynikiem jest lista unikalnych rekomendowanych wydarzeń wraz z ich słowami kluczowymi.

```
MATCH (u:User {name: $n})-[r:REGISTERED_TO]->(e:Event)
WITH COLLECT(e) AS events, u
// Znajdź użytkowników podobnych do u i ich wydarzenia
MATCH (u)-[s:SIMILAR]->(:User)-[:REGISTERED_TO]->(ee:Event
WHERE ee.startDatetime > datetime()
AND NOT ee IN events)
WITH DISTINCT ee
// Pobierz słowa kluczowe rekomendowanych wydarzeń
OPTIONAL MATCH (ee)-[:HAS]->(k:EventKeyword)
RETURN
ee.id AS eventId,
ee.name AS eventName,
ee.startDatetime AS start,
collect(k.name) AS keywords;
```

- Pierwszy MATCH + COLLECT – zbiera jako listę wszystkie wydarzenia, do których użytkownik u jest zapisany.
- Drugi MATCH – z węzła u przez relację SIMILAR przechodzi do innych użytkowników, a następnie ich relacjami REGISTERED_TO do wydarzeń ee.
- Warunki w WHERE:
- ee.startDatetime > datetime() — tylko wydarzenia jeszcze nieodbyte (przyszłe).
- NOT ee IN events — wyklucza wydarzenia, w których użytkownik już bierze udział.
- WITH DISTINCT ee — gwarantuje, że nie pojawią się duplikaty, jeśli wielu podobnych użytkowników jest zapisanych na to samo wydarzenie.
- OPTIONAL MATCH … HAS → k i collect(k.name) – dopełnia wynik listą słów kluczowych dla każdego rekomendowanego wydarzenia.
