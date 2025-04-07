/*
	graph
	This problem requires you to implement a basic graph functio
*/

// 导入标准库中的集合类型
use std::collections::{HashMap, HashSet};
use std::fmt;

// 定义一个表示"图中不存在该节点"错误的结构体
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

// 为NodeNotInGraph实现Display trait，以便可以打印错误信息
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

// 定义无向图结构体，使用邻接表存储图结构
pub struct UndirectedGraph {
    // 邻接表：键是节点名称(String)，值是该节点的邻居列表(Vec<(邻居名称, 边权重)>)
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

// 为UndirectedGraph实现Graph trait
impl Graph for UndirectedGraph {
    // 创建一个新的空图
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    
    // 获取可变的邻接表引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    
    // 获取不可变的邻接表引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    
    // 添加边到无向图中（会同时添加两个方向的边）
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 解构边为：起始节点、目标节点和权重
        let (from_node, to_node, weight) = edge;
        
        // 如果起始节点不存在于图中，则添加它
        if !self.contains(from_node) {
            self.add_node(from_node);
        }
        // 如果目标节点不存在于图中，则添加它
        if !self.contains(to_node) {
            self.add_node(to_node);
        }
        
        // 添加从起始节点到目标节点的边
        self.adjacency_table_mutable()
            .entry(from_node.to_string())  // 获取起始节点的入口
            .or_insert_with(Vec::new)     // 如果不存在则插入空Vec
            .push((to_node.to_string(), weight));  // 添加边到邻居列表
            
        // 因为是无向图，所以还要添加反向边（从目标节点到起始节点）
        self.adjacency_table_mutable()
            .entry(to_node.to_string())   // 获取目标节点的入口
            .or_insert_with(Vec::new)     // 如果不存在则插入空Vec
            .push((from_node.to_string(), weight));  // 添加反向边
    }
}

// 定义图的trait，提供图的基本操作接口
pub trait Graph {
    // 创建一个新图
    fn new() -> Self;
    
    // 获取可变的邻接表
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    
    // 获取不可变的邻接表
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    // 添加节点到图中
    fn add_node(&mut self, node: &str) -> bool {
        let node = node.to_string();
        // 如果节点已存在，返回false
        if self.contains(&node) {
            return false;
        }
        // 在邻接表中插入新节点（初始邻居列表为空）
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }
    
    // 添加边到图中（这是trait的默认实现，会被具体实现覆盖）
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from_node, to_node, weight) = edge;
        if !self.contains(from_node) {
            self.add_node(from_node);
        }
        if !self.contains(to_node) {
            self.add_node(to_node);
        }
        self.adjacency_table_mutable()
            .entry(from_node.to_string())
            .or_insert_with(Vec::new)
            .push((to_node.to_string(), weight));
    }
    
    // 检查图中是否包含指定节点
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    
    // 获取图中所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    
    // 获取图中所有边的列表
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        // 遍历邻接表中的每个节点及其邻居
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                // 将每条边添加到结果列表中
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}