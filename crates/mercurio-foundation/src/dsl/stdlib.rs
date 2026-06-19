use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

use rhai::{Dynamic, Engine, Map};

use super::types::{DslElement, ElementSet, ModelContext};

pub fn register_stdlib(engine: &mut Engine) {
    engine.register_fn("count_by_kind", |set: &mut ElementSet| -> Map {
        let mut counts = BTreeMap::<String, i64>::new();
        for id in &set.ids {
            if let Some(element) = set.graph.element(*id) {
                *counts.entry(element.kind.as_ref().to_string()).or_default() += 1;
            }
        }
        counts
            .into_iter()
            .map(|(kind, count)| (kind.into(), Dynamic::from(count)))
            .collect()
    });

    engine.register_fn("all_parts", |context: &mut ModelContext| -> ElementSet {
        context.parts()
    });

    engine.register_fn(
        "reachable",
        |element: &mut DslElement, relation: String| -> ElementSet {
            let graph = Arc::clone(&element.graph);
            let mut visited = BTreeSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(element.id);

            while let Some(id) = queue.pop_front() {
                if !visited.insert(id) {
                    continue;
                }
                for edge in graph.outgoing(id, &relation) {
                    queue.push_back(edge.target);
                }
            }

            visited.remove(&element.id);
            ElementSet {
                ids: visited.into_iter().collect(),
                graph,
            }
        },
    );

    engine.register_fn("specialization_depth", |element: &mut DslElement| -> i64 {
        let graph = Arc::clone(&element.graph);
        let mut depth = 0i64;
        let mut current = element.id;
        let mut visited = BTreeSet::new();

        loop {
            if !visited.insert(current) {
                break;
            }
            match graph
                .outgoing(current, "specializes")
                .next()
                .map(|edge| edge.target)
            {
                Some(parent) => {
                    depth += 1;
                    current = parent;
                }
                None => break,
            }
        }

        depth
    });
}
