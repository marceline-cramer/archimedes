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

use serde::{Deserialize, Serialize};

pub use crate::frontend::types::{ResourceId, Value};
use crate::utils::Key;

pub type Tuple = Arc<[Value]>;

pub type IndexList = Arc<[usize]>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Node {
    /// Joins two nodes together.
    Join {
        /// The left-hand node to join. Leftover terms come first.
        lhs: Key<Node>,

        /// The right-hand node to join. Leftover terms come after left-hand terms.
        rhs: Key<Node>,

        /// The number of terms starting from the front of each node to join.
        num: usize,
    },

    /// Projects (rearranges) a node's terms.
    ///
    /// Can also unload nodes.
    Project {
        /// The node to project.
        src: Key<Node>,

        /// A list of indices for which terms of the original node this projection loads.
        map: Arc<[usize]>,
    },

    /// Loads node contents from a relation.
    LoadRelation {
        /// The key of the relation (given by [ResourceId]) to load.
        resource: ResourceId,
    },

    /// Stores a node's contents into a relation.
    StoreRelation {
        /// The node to load terms from.
        src: Key<Node>,

        /// The relation to store into.
        dst: ResourceId,

        /// The map from destination terms to source terms.
        map: IndexList,
    },
}

impl Node {
    pub fn join_lhs(self) -> Option<(Key<Node>, usize)> {
        match self {
            Node::Join { lhs, num, .. } => Some((lhs, num)),
            _ => None,
        }
    }

    pub fn join_rhs(self) -> Option<(Key<Node>, usize)> {
        match self {
            Node::Join { rhs, num, .. } => Some((rhs, num)),
            _ => None,
        }
    }

    pub fn project_src(self) -> Option<Key<Node>> {
        match self {
            Node::Project { src, .. } => Some(src),
            _ => None,
        }
    }

    pub fn project_map(self) -> Option<IndexList> {
        match self {
            Node::Project { map, .. } => Some(map),
            _ => None,
        }
    }

    pub fn load_relation(self) -> Option<ResourceId> {
        match self {
            Node::LoadRelation { resource } => Some(resource),
            _ => None,
        }
    }

    pub fn store_relation(self) -> Option<(Key<Node>, (ResourceId, IndexList))> {
        match self {
            Node::StoreRelation { src, dst, map } => Some((src, (dst, map))),
            _ => None,
        }
    }
}
