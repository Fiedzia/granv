GRANV - GRaph ANalysis and Visualisation tool


Gran is inspired by graphviz, but tries to be more modern,
give more control over visual aspects and make better use of modern hardware.
Gran is written in Rust, provides single binary supporting wide range of commands
helping you with graph-related tasks.

It can load graphviz files

COMMANDS:

* tree - operate on trees
* graph - operate on graphs
* json - operate on json


TREE OPERATIONS::


* load directory <path> - scan directory content, will provide `type` (file|directory) ,`size` (in bytes), `size_total` (size including children) and `size_human` properties

* load file --format <format> <path> - load data from file, where format can be:

 - tree: file with output of linux `tree` command
 - graphviz: graphviz file. It must be a tree.


* load sql --database postgres|mysql|sqlite|duckdb --query <query> - load data from sql database
* filter-nodes <condition> - sql-like filtering of nodes you want to include or exclude. Useful if you do not want or can not modify original file, but want to load only part of the data.
* layout <layout> - generate visualisation using specific layout. Available layouts:

 - directories: render tree like file managers show directories, top-to-bottom, left-aligned
 - radial: render radial-style circular tree with root node starting in the middle of the image 
 - treemap
 - flamegraph



GRAPH OPERATIONS:


* load file <filename> - load data from file
* load sql --database postgres|mysql|sqlite|duckdb --query <query> - load data from sql database
* load cypher --database neo4j --query <query> - load data from source supporting cypher query
* filter-nodes <condition>, filter-edges <condition> - sql-like filtering of nodes or edges you want to include or exclude. Useful if you do not want or can not modify original file, but want to load only part of the data.
  for example `node.type = "book" OR node.type = "author"`
* clusters --format text|graphviz - show list of graph clusters
* layout <layout> - generate visualisation using specific layout.

Note that commands can be chained using circle • character (Unicode U+2022), for example:
`gran load file example.dot \
 • filter-nodes 'node.type = "book" \
 • filter-edges 'edge.color = "red" \
 • layout 2d.dot 
'


TODO:

* support other graph formats
* load data from graph databases (like neo4j)
* support more layouts (2d and 3d)
* use GPU


