// project graph in memory
MATCH (source:User)-[r:REGISTERED_TO]->(target:Event)
RETURN gds.graph.project(
  'registrations',
  source,
  target,
  {
    sourceNodeLabels: labels(source),
    targetNodeLabels: labels(target),
    relationshipType: 'REGISTERED_TO'
  },
  { undirectedRelationshipTypes: ['REGISTERED_TO'] }
);

// add embeddings to the graph
CALL gds.fastRP.mutate(
  'registrations',
  {
    embeddingDimension: 256,
    iterationWeights: [0.8, 1, 1, 1],
    randomSeed: 42,
    mutateProperty: 'embedding'
  }
)
YIELD nodePropertiesWritten;

// compute and write similarities
CALL gds.knn.write(
  'registrations',
  {
    nodeProperties: ['embedding'],
    nodeLabels: ['User'],
    topK: 40,
    sampleRate: 1.0,
    deltaThreshold: 0.0,
    writeProperty: 'score',
    writeRelationshipType: 'SIMILAR'
  }
)
YIELD similarityDistribution
RETURN similarityDistribution.mean AS meanSimilarity;

// show similar users
//MATCH (source:User)-[r:SIMILAR]->(target:User)
//RETURN *;

// recommend
//MATCH (u:User {name: "qhill"})-[r:REGISTERED_TO]->(e:Event)
//WITH COLLECT(e) AS events, u
//MATCH (u)-[s:SIMILAR]->(:User)-[:REGISTERED_TO]->(e:Event WHERE e.startDatetime > datetime() AND (NOT e  IN events))
//RETURN e;