use colored::Colorize;
use petgraph::algo::{has_path_connecting, kosaraju_scc};
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;

use crate::circuit_view::CircuitView;
use crate::gate::Gate;
use crate::id::{CircuitID, OrderedWireID, WireID};

// サブ回路インスタンスの境界ポート番号
pub type TimingPortIndex = usize;
// サブ回路の境界ポート間に必要な前後関係
pub type TimingPortEdges = Vec<(TimingPort, TimingPort)>;
// チェック済みサブ回路のタイミング情報を CircuitID で引くための表
pub type TimingLibrary = HashMap<CircuitID, TimingPortEdges>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimingPortSide {
    Inputs,
    Outputs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimingPort {
    pub side: TimingPortSide,
    pub index: TimingPortIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TimingPoint {
    Produce,
    Consume,
}

// 回路内部のタイミンググラフの頂点.
// delay の有無を表現するため、各 WireID を produce/consume の2頂点に分ける.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TimingNode {
    wire: WireID,
    point: TimingPoint,
}

type TimingGraph = DiGraphMap<TimingNode, ()>;

fn produce(wire: WireID) -> TimingNode {
    TimingNode {
        wire,
        point: TimingPoint::Produce,
    }
}

fn consume(wire: WireID) -> TimingNode {
    TimingNode {
        wire,
        point: TimingPoint::Consume,
    }
}

fn add_order_edges(graph: &mut TimingGraph, wires: &[OrderedWireID]) {
    // order が同じ場合は制約なし. 小さい order から大きい order へだけ辺を張る.
    for from in wires {
        for to in wires {
            if from.order < to.order {
                graph.add_edge(consume(from.id), consume(to.id), ());
            }
        }
    }
}

fn boundary_node(port: TimingPort, inputs: &[WireID], outputs: &[WireID]) -> TimingNode {
    // サブ回路境界の inputs は消費点、outputs は生成点として親回路のノードへ写す.
    match port.side {
        TimingPortSide::Inputs => {
            assert!(
                port.index < inputs.len(),
                "{}",
                format!(
                    "Subcircuit timing edge refers to inputs[{}], but it has only {} ports",
                    port.index,
                    inputs.len()
                )
                .red()
            );
            consume(inputs[port.index])
        }
        TimingPortSide::Outputs => {
            assert!(
                port.index < outputs.len(),
                "{}",
                format!(
                    "Subcircuit timing edge refers to outputs[{}], but it has only {} ports",
                    port.index,
                    outputs.len()
                )
                .red()
            );
            produce(outputs[port.index])
        }
    }
}

pub(crate) fn check_timing(
    circuit: &dyn CircuitView,
    timing_library: &TimingLibrary,
) -> TimingPortEdges {
    let graph = build_graph(circuit, timing_library);
    assert_acyclic(circuit, &graph);

    // この回路をサブ回路として使うときに親へ渡す、境界ポート間の到達可能性を集める.
    let inputs = circuit.subcircuit_input_wire_ids();
    let outputs = circuit.subcircuit_output_wire_ids();
    let boundary_nodes: Vec<(TimingPort, TimingNode)> = inputs
        .iter()
        .enumerate()
        .map(|(index, wire)| {
            (
                TimingPort {
                    side: TimingPortSide::Inputs,
                    index,
                },
                consume(*wire),
            )
        })
        .chain(outputs.iter().enumerate().map(|(index, wire)| {
            (
                TimingPort {
                    side: TimingPortSide::Outputs,
                    index,
                },
                produce(*wire),
            )
        }))
        .collect();
    let mut timing_edges = Vec::new();

    for (from_port, from_node) in &boundary_nodes {
        for (to_port, to_node) in &boundary_nodes {
            if from_port == to_port {
                continue;
            }
            if has_path_connecting(&graph, *from_node, *to_node, None) {
                timing_edges.push((*from_port, *to_port));
            }
        }
    }

    timing_edges
}

fn build_graph(circuit: &dyn CircuitView, timing_library: &TimingLibrary) -> TimingGraph {
    let mut graph = TimingGraph::new();

    // delay が 0 の wire だけ同一サイクル内の produce -> consume 制約を張る.
    // delay がある wire はここを切ることでサイクル境界として扱う.
    for (id, info) in circuit.wires() {
        graph.add_node(produce(*id));
        graph.add_node(consume(*id));
        if info.value.delay == 0 {
            graph.add_edge(produce(*id), consume(*id), ());
        }
    }

    for gate in circuit.gates() {
        match &gate.value {
            Gate::Jtl { a, q, .. } | Gate::Buff { a, q, .. } => {
                graph.add_edge(consume(*a), produce(*q), ());
            }
            Gate::Split { a, q1, q2, .. } => {
                graph.add_edge(consume(*a), produce(*q1), ());
                graph.add_edge(consume(*a), produce(*q2), ());
            }
            Gate::Merge { a, b, q, .. } => {
                graph.add_edge(consume(*a), produce(*q), ());
                graph.add_edge(consume(*b), produce(*q), ());
            }
            Gate::And { a, b, clk, q, .. }
            | Gate::Or { a, b, clk, q, .. }
            | Gate::Xor { a, b, clk, q, .. }
            | Gate::Xnor { a, b, clk, q, .. }
            | Gate::Ndro { a, b, clk, q, .. } => {
                // クロック付きゲートでは、指定 order と clk -> q の制約を追加する.
                add_order_edges(&mut graph, &[*a, *b, *clk]);
                graph.add_edge(consume(clk.id), produce(*q), ());
            }
            Gate::Not { a, clk, q, .. } | Gate::Dff { a, clk, q, .. } => {
                // 1入力のクロック付きゲートも同じ扱い.
                add_order_edges(&mut graph, &[*a, *clk]);
                graph.add_edge(consume(clk.id), produce(*q), ());
            }
            Gate::ZeroAsync { .. } | Gate::Terminate { .. } => {}
            Gate::Subcircuit {
                inputs,
                outputs,
                circuit: subcircuit_name,
                circuit_id,
                ..
            } => {
                // サブ回路は事前にチェック済みの境界ポート間制約を親回路上に展開する.
                let timing_edges = timing_library
                    .get(circuit_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "{}",
                            format!(
                                "Timing information for subcircuit `{}` is missing (gate defined at {})",
                                subcircuit_name, gate.location
                            )
                            .red()
                        )
                    });

                for (from, to) in timing_edges {
                    graph.add_edge(
                        boundary_node(*from, inputs, outputs),
                        boundary_node(*to, inputs, outputs),
                        (),
                    );
                }
            }
            Gate::_Reserved => unreachable!(),
        }
    }

    graph
}

fn assert_acyclic(circuit: &dyn CircuitView, graph: &TimingGraph) {
    let cyclic_component = kosaraju_scc(graph).into_iter().find(|component| {
        component.len() > 1
            || component
                .iter()
                .any(|node| graph.contains_edge(*node, *node))
    });

    // 閉路がなければ正常終了.
    let Some(component) = cyclic_component else {
        return;
    };

    // エラー表示では閉路成分に含まれる wire 名と定義位置を出す.
    let mut wires: Vec<WireID> = component.iter().map(|node| node.wire).collect();
    wires.sort();
    wires.dedup();

    let mut lines = vec!["Timing constraints are unsatisfiable.".to_string()];
    lines.push("Cycle includes:".to_string());
    for wire in wires {
        let info = circuit.wires().get(&wire).unwrap();
        lines.push(format!(
            "  wire `{}` defined at {}",
            info.value.name, info.location
        ));
    }

    panic!("{}", lines.join("\n").red());
}
