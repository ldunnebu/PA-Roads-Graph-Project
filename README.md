# PA-Roads-Graph-Project

The graph used for this project is an unweighted, undirected graph called “Pennsylvania road network” from Stanford Large Network Dataset Collection (SNAP).
https://snap.stanford.edu/data/roadNet-PA.html 

My goal for this project was to get the average length of the shortest path from one node to all the other nodes using a Breadth-First Search. I also wanted to find the nodes (interesections) with the shortest average path length highlighting their centrality. Unfortunately, the dataset was very large (over 1,000,000 vertices and 1,500,000 edges, diameter (longest shortest path) of 786) so the code was going to take over eight hours to run, therefore, I took a sample of 50,000 vertices. This limits the effectiveness of the centrality measure because it doesn't include a large portion of the data. I did, however, find the longest shortest path in the sample.
