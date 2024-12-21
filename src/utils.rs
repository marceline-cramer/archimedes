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

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
};

use differential_dataflow::{
    input::Input,
    trace::{Cursor, TraceReader},
    Collection, Data, Hashable,
};
use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use timely::{
    communication::{Allocate, Allocator},
    container::flatcontainer::IntoOwned,
    dataflow::Scope,
    progress::frontier::AntichainRef,
    worker::Worker,
};

pub type Time = i32;
pub type Diff = isize;
pub type InputSession<T> = differential_dataflow::input::InputSession<Time, T, Diff>;

#[derive(Clone)]
pub struct Aggregate<G: Scope, T>(Collection<G, T>);

impl<G: Scope, T> AsRef<Collection<G, T>> for Aggregate<G, T> {
    fn as_ref(&self) -> &Collection<G, T> {
        &self.0
    }
}

impl<G: Input, T: Data> Aggregate<G, T> {
    pub fn new(scope: &mut G) -> Self {
        Self(scope.new_collection().1)
    }
}

impl<G: Scope, T: Data> Aggregate<G, T> {
    pub fn from_collection(inner: Collection<G, T>) -> Self {
        Self(inner)
    }

    pub fn push(&mut self, other: &Collection<G, T>) {
        self.0 = self.0.concat(other);
    }

    pub fn with_map_errs<O, E>(
        &mut self,
        other: &Collection<G, Result<O, E>>,
        mut map: impl FnMut(E) -> T + 'static,
    ) -> Collection<G, O>
    where
        O: Data,
        E: Data,
    {
        self.push(&other.flat_map(move |result| result.err().map(&mut map)));
        other.flat_map(Result::ok)
    }

    pub fn with_errs<O, E>(&mut self, other: &Collection<G, Result<O, E>>) -> Collection<G, O>
    where
        O: Data,
        E: Data + Into<T>,
    {
        self.with_map_errs(other, |err| err.into())
    }
}

pub fn key<K, V>((k, _v): (K, V)) -> K {
    k
}

pub fn value<K, V>((_k, v): (K, V)) -> V {
    v
}

pub fn swap<K, V>((k, v): (K, V)) -> (V, K) {
    (v, k)
}

pub fn map_value<K, I, O>(
    mut cb: impl FnMut(I) -> Option<O>,
) -> impl FnMut((K, I)) -> Option<(K, O)> {
    move |(k, v)| cb(v).map(|v| (k, v))
}

pub fn run_dataflow<I: WorkerInput, O: WorkerOutput>(
    update_rx: Receiver<Vec<I::Update>>,
    cb: impl Fn(&mut Worker<Allocator>) -> (I, O) + Send + Sync + 'static,
) -> Receiver<Vec<O::Result>> {
    let (output_tx, output_rx) = flume::unbounded();
    let (crew_tx, crew_rx) = flume::unbounded();

    let helm = Helm {
        update_rx,
        output_tx,
        crew_tx,
        crew_rx,
    };

    let config = timely::Config::thread();
    let guards = timely::execute(config, move |worker| {
        let (input, output) = cb(worker);
        let helm = helm.clone();

        if worker.index() == 0 {
            let captain = Captain::new(worker, helm, input, output);
            captain.run();
        } else {
            drop(input);
            let mut crew = Crew::new(worker, &helm, output);
            while crew.update() {}
        }
    })
    .unwrap();

    std::thread::spawn(move || {
        drop(guards);
        eprintln!("dropped dataflow worker threads");
    });

    output_rx
}

pub trait WorkerInput: 'static {
    type Update: Send + Sync + 'static;
    fn advance_to(&mut self, time: Time);
    fn on_update(&mut self, update: Self::Update);
    fn flush(&mut self);
}

pub trait WorkerOutput: 'static {
    type Result: Debug + Send + Sync + 'static;
    fn advance_to(&mut self, time: Time);
    fn pending(&self, time: &Time) -> bool;
    fn results(&mut self) -> Vec<Self::Result>;
}

pub struct CrewReport<T> {
    pub idx: usize,
    pub advance_tx: Sender<Time>,
    pub output_rx: Receiver<Vec<T>>,
}

pub struct Helm<I: WorkerInput, O: WorkerOutput> {
    pub update_rx: Receiver<Vec<I::Update>>,
    pub output_tx: Sender<Vec<O::Result>>,
    pub crew_tx: Sender<CrewReport<O::Result>>,
    pub crew_rx: Receiver<CrewReport<O::Result>>,
}

impl<I: WorkerInput, O: WorkerOutput> Clone for Helm<I, O> {
    fn clone(&self) -> Self {
        Self {
            update_rx: self.update_rx.clone(),
            output_tx: self.output_tx.clone(),
            crew_tx: self.crew_tx.clone(),
            crew_rx: self.crew_rx.clone(),
        }
    }
}

pub struct Captain<'a, A: Allocate, I: WorkerInput, O: WorkerOutput> {
    pub crew: Crew<'a, A, O>,
    pub inputs: I,
    pub update_rx: Receiver<Vec<I::Update>>,
    pub advance_tx: Vec<Sender<Time>>,
    pub output_tx: Sender<Vec<O::Result>>,
    pub output_rx: Vec<Receiver<Vec<O::Result>>>,
}

impl<'a, A: Allocate, I: WorkerInput, O: WorkerOutput> Captain<'a, A, I, O> {
    pub fn new(worker: &'a mut Worker<A>, helm: Helm<I, O>, input: I, output: O) -> Self {
        let crew = Crew::new(worker, &helm, output);
        let Helm {
            crew_rx,
            update_rx,
            output_tx,
            ..
        } = helm;

        let mut reported = 0;
        let mut advance_tx = Vec::new();
        let mut output_rx = Vec::new();
        while reported < crew.worker.peers() {
            let report = crew_rx.recv().unwrap();
            advance_tx.push(report.advance_tx);
            output_rx.push(report.output_rx);
            reported += 1;
        }

        Self {
            crew,
            inputs: input,
            update_rx,
            advance_tx,
            output_rx,
            output_tx,
        }
    }

    pub fn run(mut self) {
        // begin the event loop
        let mut time = 0;
        self.inputs.advance_to(time);

        // pump the event loop
        while let Ok(mut updates) = self.update_rx.recv() {
            // drain any other pending updates
            for more_updates in self.update_rx.drain() {
                updates.extend(more_updates);
            }

            // feed the inputs
            for update in updates {
                self.inputs.on_update(update);
            }

            // move to next time and flush inputs
            time += 1;
            self.inputs.advance_to(time);
            self.inputs.flush();

            // advance the crew
            for tx in self.advance_tx.iter() {
                tx.send(time).unwrap();
            }

            // update ourself
            self.crew.update();

            // collect results from crew
            let mut results = Vec::new();
            for rx in self.output_rx.iter() {
                let new = rx.recv().unwrap();
                results.extend(new);
            }

            // send results
            let Ok(()) = self.output_tx.send(results) else {
                break;
            };
        }
    }
}

pub struct Crew<'a, A: Allocate, O: WorkerOutput> {
    pub worker: &'a mut Worker<A>,
    pub output: O,
    pub advance_rx: Receiver<Time>,
    pub output_tx: Sender<Vec<O::Result>>,
}

impl<'a, A: Allocate, O: WorkerOutput> Crew<'a, A, O> {
    pub fn new<I: WorkerInput>(worker: &'a mut Worker<A>, helm: &Helm<I, O>, output: O) -> Self {
        let (advance_tx, advance_rx) = flume::unbounded();
        let (output_tx, output_rx) = flume::unbounded();

        let report = CrewReport {
            idx: worker.index(),
            advance_tx,
            output_rx,
        };

        helm.crew_tx.send(report).unwrap();

        Self {
            worker,
            output,
            advance_rx,
            output_tx,
        }
    }

    pub fn update(&mut self) -> bool {
        let Ok(time) = self.advance_rx.recv() else {
            return false;
        };

        while self.output.pending(&time) {
            self.worker.step();
        }

        let results = self.output.results();
        self.output_tx.send(results).unwrap();

        self.output.advance_to(time);

        true
    }
}

pub type TraceSet<T, K> = TraceMap<T, K, ()>;

pub struct TraceMap<T, K, V> {
    inner: HashSet<(K, V)>,
    trace: T,
}

pub trait DynTraceMap<K, V> {
    fn advance_to(&mut self, time: Time);
    fn update(&mut self);
    fn distinct(&self) -> HashMap<K, V>;
    fn distinct_keys(&self) -> HashSet<K>;
}

impl<T, K, V> DynTraceMap<K, V> for TraceMap<T, K, V>
where
    T: TraceReader<Time = Time, Diff = Diff>,
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq + Hash,
    for<'a> T::Key<'a>: IntoOwned<'a, Owned = K>,
    for<'a> T::Val<'a>: IntoOwned<'a, Owned = V>,
{
    fn advance_to(&mut self, time: Time) {
        let frontier = [time];
        let frontier = AntichainRef::new(&frontier);
        self.trace.set_physical_compaction(frontier);
        self.trace.set_logical_compaction(frontier);
    }

    fn update(&mut self) {
        let (mut cursor, storage) = self.trace.cursor();
        for (pair, sums) in cursor.to_vec(&storage) {
            let count: isize = sums.iter().map(|(_time, sum)| *sum).sum();

            if count > 0 {
                self.inner.insert(pair);
            } else {
                self.inner.remove(&pair);
            }
        }
    }

    fn distinct(&self) -> HashMap<K, V> {
        self.inner.iter().cloned().collect()
    }

    fn distinct_keys(&self) -> HashSet<K> {
        self.inner.iter().map(|(k, _)| k).cloned().collect()
    }
}

impl<T, K, V> TraceMap<T, K, V> {
    pub fn new(trace: T) -> Self {
        Self {
            inner: HashSet::new(),
            trace,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Key<T>(u64, PhantomData<T>);

impl<T> Copy for Key<T> {}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Hashable<Output = u64>> Key<T> {
    pub fn new(data: &T) -> Self {
        Key(data.hashed(), PhantomData)
    }

    pub fn pair(data: T) -> (Self, T) {
        (Self::new(&data), data)
    }
}
