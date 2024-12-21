// Copyright (c) 2024 Marceline Cramer.
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// This file is part of Archimedes.
//
// Archimedes is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// Archimedes is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Archimedes. If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use differential_dataflow::operators::{iterate::Variable, Join, Threshold};
use timely::{communication::Allocator, dataflow::Scope, order::Product, worker::Worker};

use crate::{
    backend::types::*,
    utils::{map_value, swap, value, InputSession, Key},
};

pub fn backend(worker: &mut Worker<Allocator>) {
    let mut facts = InputSession::new();
    let mut nodes = InputSession::new();

    worker.dataflow(|scope| {
        let nodes = nodes.to_collection(scope).map(Key::pair);
        let facts = facts.to_collection(scope);

        let relations = scope.iterative::<u32, _, _>(|scope| {
            // import context
            let facts = facts.enter(scope);
            let nodes = nodes.enter(scope);

            // initialize iterative variables and state
            let step = Product::new(Default::default(), 1);
            let relations = Variable::new_from(facts, step);
            let tuples = Variable::new(scope, step);

            // extract left and right sides of join operation
            let join_lhs = nodes
                .flat_map(map_value(Node::join_lhs))
                .map(|(dst, (src, num))| (src, (dst, num)))
                .join(&tuples)
                .map(value)
                .map(join_slice);

            let join_rhs = nodes
                .flat_map(map_value(Node::join_rhs))
                .map(|(dst, (src, num))| (src, (dst, num)))
                .join(&tuples)
                .map(value)
                .map(join_slice);

            // select and merge sides of join source nodes
            let join = join_lhs.join(&join_rhs).map(join);

            // project operation
            let project_src = nodes.flat_map(map_value(Node::project_src)).map(swap);
            let project_map = nodes.flat_map(map_value(Node::project_map));

            // join together project targets and rearrange values
            let project = project_src
                .join(&tuples)
                .map(value)
                .join(&project_map)
                .map(project);

            // load relation operation
            let load_relation = nodes
                .flat_map(map_value(Node::load_relation))
                .map(swap)
                .join(&relations)
                .map(value);

            // combine all operations into new tuples
            let new_tuples = join.concat(&project).concat(&load_relation).distinct();
            tuples.set_concat(&new_tuples);

            // store new tuples to corresponding relations
            let stored = nodes
                .map(value)
                .flat_map(Node::store_relation)
                .join(&new_tuples)
                .map(value)
                .map(store)
                .distinct();

            relations.set_concat(&stored).leave()
        });
    });
}

pub fn join_slice(((dst, num), vals): ((Key<Node>, usize), Tuple)) -> ((Key<Node>, Tuple), Tuple) {
    let prefix = vals[..num].into();
    let trail = vals[num..].into();
    ((dst, prefix), trail)
}

pub fn join(
    ((dst, prefix), (lhs, rhs)): ((Key<Node>, Tuple), (Tuple, Tuple)),
) -> (Key<Node>, Tuple) {
    let result = prefix
        .iter()
        .chain(lhs.iter())
        .chain(rhs.iter())
        .cloned()
        .collect();

    (dst, result)
}

pub fn project((dst, (src, map)): (Key<Node>, (Tuple, Arc<[usize]>))) -> (Key<Node>, Tuple) {
    (dst, map.iter().map(|idx| src[*idx].clone()).collect())
}

pub fn store(((dst, map), vals): ((ResourceId, IndexList), Tuple)) -> (ResourceId, Tuple) {
    let vals = map.iter().map(|idx| vals[*idx].clone()).collect();
    (dst, vals)
}
