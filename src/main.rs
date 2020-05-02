use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize)]
struct Tree {
  id: String,
  children: Vec<Tree>,
}

impl Tree {
  fn dot(&self) -> String {
    format!("digraph {} {{\n", self.id)
      + "  rankdir=TD;\n"
      + "  node [shape=record,width=0.1,height=0.1];\n\n"
      + &self.dot_inner("n".to_string())
      + "}\n"
  }

  fn dot_inner(&self, path: String) -> String {
    if self.children.is_empty() {
      return "".to_string();
    }

    let children = || self.children.iter().zip(0..self.children.len());
    format!(
      "  {} [label=\"{}\"];\n",
      path,
      children()
        .map(|(c, i)| format!("<p{}> {}", i, c.id))
        .collect::<Vec<_>>()
        .join(" | "),
    ) + &self
      .edges()
      .iter()
      .map(|i| format!("  {}:p{} -> {}_{} [headport=n]", path, i, path, i))
      .collect::<Vec<_>>()
      .join("\n")
      + "\n"
      + &children()
        .map(|(c, i)| c.dot_inner(format!("{}_{}", path, i)))
        .collect::<Vec<_>>()
        .join("")
  }

  fn edges(&self) -> Vec<usize> {
    self
      .children
      .iter()
      .zip(0..self.children.len())
      .filter(|(c, _)| c.children.len() > 0)
      .map(|(_, i)| i)
      .collect()
  }
}

fn main() {
  let bs = std::fs::read("demo/demo.json").unwrap();
  let text = String::from_utf8(bs).unwrap();
  let tree: Tree = serde_json::from_str(&text).unwrap();

  // println!("{:#?}", tree);
  println!("{}", tree.dot());
}
