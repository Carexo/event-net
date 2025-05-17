MATCH (n) DETACH DELETE n;

CREATE CONSTRAINT user_name IF NOT EXISTS
FOR (u:User)
REQUIRE u.name IS UNIQUE;

CREATE CONSTRAINT event_id IF NOT EXISTS
FOR (e:Event)
REQUIRE e.id IS UNIQUE;

CREATE CONSTRAINT eventKeyword_name IF NOT EXISTS
FOR (k:EventKeyword)
REQUIRE k.name IS UNIQUE;

LOAD CSV WITH HEADERS FROM 'file:///user_events.csv' AS line
CALL (line) {
MERGE (u:User {name: line.user_name})
MERGE (e:Event {id: toInteger(line.event_id)})
MERGE (u)-[:REGISTERED_TO]->(e)
SET e.name = line.event_name, e.startDatetime = datetime(line.start_datetime)
} IN TRANSACTIONS OF 1000 ROWS;

LOAD CSV WITH HEADERS FROM 'file:///event_keyword.csv' AS line
CALL (line) {
MERGE (e:Event {id: toInteger(line.event_id)})
MERGE (k:EventKeyword {name: line.keyword})
MERGE (e)-[:HAS]->(k)
} IN TRANSACTIONS OF 1000 ROWS;