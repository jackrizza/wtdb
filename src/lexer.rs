
const COMPARE_QUERIES : [&str; 4] = ["SELECT", "INSERT", "FROM", "WHERE"];
const COMPARE_OPERATORS : [&str; 3] = ["*", "+", "-"];

#[derive(Debug, Clone)]
pub enum Description {
    Query(String),
    Operator(String),
    String(String),
    Int(i32),
    Float(f32),
    None,
}

#[derive(Debug)]
pub enum WarpReturn {
    Next(i32),
    Eol
}

#[derive(Debug)]
pub struct Lexer {
    pub query : String,
    pub node_tree : Vec<Node>,
    pub executer : Vec<LeftRight>
}


#[derive(Debug, Clone)]
pub struct Node {
    pub id : i32,
    pub payload : Description,
}

#[derive(Debug)]
pub struct LeftRight {
    pub id : i32,
    pub left : i32,
    pub right : Vec<i32>,
}


pub trait Warp {
    fn foward(id : i32, len : i32) -> WarpReturn;
    fn backward(id : i32) -> WarpReturn;
}

impl Warp for Node {
    fn foward(id : i32, len : i32) -> WarpReturn {
        let next = id + 1;
        if next > len {
            WarpReturn::Eol
        } else {
            WarpReturn::Next(next)
        }
    }

    fn backward(id : i32) -> WarpReturn {
        let next = id - 1;
        if next < 0 {
            WarpReturn::Eol
        } else {
            WarpReturn::Next(next)
        }
    }
}

impl Warp for LeftRight {
    fn foward(id : i32, len : i32) -> WarpReturn {
        let next = id + 1;
        if next > len {
            WarpReturn::Eol
        } else {
            WarpReturn::Next(next)
        }
    }

    fn backward(id : i32) -> WarpReturn {
        let next = id - 1;
        if next < 0 {
            WarpReturn::Eol
        } else {
            WarpReturn::Next(next)
        }
    }
}

impl Lexer {
    pub fn new(query : String) -> Self {
        Lexer {
            query : query,
            node_tree : Vec::new(),
            executer : Vec::new(),
        }
    } 

    pub fn node_tree(&mut self) {
        for s in self.query.split(" ") {
            // if Query
            if COMPARE_QUERIES.iter().filter(|x| **x == s).collect::<Vec<&&str>>().len() > 0 {
                self.node_tree.push(Node {
                    id : (self.node_tree.len() + 1) as i32,
                    payload : Description::Query(s.to_string()),
                });
            }
            // if Operator
            else if COMPARE_OPERATORS.iter().filter(|x| **x == s).collect::<Vec<&&str>>().len() > 0 {
                self.node_tree.push( Node {
                    id : (self.node_tree.len() + 1) as i32,
                    payload : Description::Operator(s.to_string()),
                });
            }
            // else String
            else {
                self.node_tree.push( Node {
                    id : (self.node_tree.len() + 1) as i32,
                    payload : Description::String(s.to_string()),
                })
            }
        }
    }

    pub fn executer(&mut self) {
        
        let mut cloned_node_tree = self.node_tree.clone();
        let mut right : Vec<i32> = Vec::new();
        let mut left : i32 = -1;
        let mut last : usize = 0 as usize;
        loop {
            if cloned_node_tree.len() == 0 {
                break;
            }

            if cloned_node_tree.len() - 1 < 0 {
                last = 0 as usize;
            } else {
                last = cloned_node_tree.len() - 1;
            }
            println!("{:?}", cloned_node_tree[last]); 
            match cloned_node_tree[last].payload {
                Description::Query(_) => {
                   left = cloned_node_tree[last].id; 
                }, 
                Description::Operator(_) => {
                    right.push(cloned_node_tree[last].id);
                },
                Description::String(_) => {
                    right.push(cloned_node_tree[last].id);
                },
                _ => println!("Could not read Description for left right builder"),
            };

            if right.len() > 0 && left != -1 {
                self.executer.push(LeftRight {
                    id : (self.executer.len() + 1) as i32,
                    left : left,
                    right : right
                });

                left = -1;
                right = Vec::new();
            }
            cloned_node_tree.pop();
        }
    }
}
