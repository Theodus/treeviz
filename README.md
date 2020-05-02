# treeviz

Generate Graphviz Dot for a tree-based data structure from JSON

## Notes
- The JSON layout is chosen to be easily generated, not easily written directly
- More human-friendly input formats may be added as desired

## Examples

![demo](demo/demo.svg)

JSON input:
```json
{ "id": "Tree", "children": [
  { "id": "a", "children": [
    { "id": "c", "children": [
      { "id": "i", "children": [] },
      { "id": "j", "children": [] }]
    },
    { "id": "d", "children": [
      { "id": "k", "children": [] },
      { "id": "l", "children": [] } ]
    }]
  },
  { "id": "b", "children": [
    { "id": "g", "children": [
      { "id": "m", "children": [] },
      { "id": "n", "children": [] },
      { "id": "o", "children": [] }]
    },
    { "id": "h", "children": [] }]
  }]
}
```

Graphviz Dot output:
```dot
digraph Tree {
  rankdir=TD;
  node [shape=record,width=0.1,height=0.1];

  n [label="<p0> a | <p1> b"];
  n:p0 -> n_0 [headport=n];
  n:p1 -> n_1 [headport=n];
  n_0 [label="<p0> c | <p1> d"];
  n_0:p0 -> n_0_0 [headport=n];
  n_0:p1 -> n_0_1 [headport=n];
  n_0_0 [label="<p0> i | <p1> j"];
  n_0_1 [label="<p0> k | <p1> l"];
  n_1 [label="<p0> g | <p1> h"];
  n_1:p0 -> n_1_0 [headport=n];
  n_1_0 [label="<p0> m | <p1> n | <p2> o"];
}
```
