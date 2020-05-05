use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Tree {
  id: String,
  children: Vec<Tree>,
}

impl Tree {
  fn dot(&self) -> String {
    format!("digraph \"{}\" {{\n", self.id)
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
      .map(|i| format!("  {}:p{} -> {}_{} [headport=n];\n", path, i, path, i))
      .collect::<Vec<_>>()
      .join("")
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
  let text = match std::env::args().nth(1) {
    Some(file_name) => String::from_utf8(std::fs::read(file_name).unwrap()).unwrap(),
    None => {
      let mut buf = String::new();
      match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => buf,
        Err(_) => return,
      }
    }
  };
  match serde_json::from_str::<Tree>(&text) {
    Ok(tree) => println!("{}", tree.dot()),
    Err(err) => {
      eprintln!("{}", err);
      std::process::exit(1)
    }
  };
}
