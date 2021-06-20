use crate::game::map::{WorldMap, Coordinate, HexCell};
use std::sync::{Arc};
use std::cmp;
use dashmap::DashMap;

impl WorldMap {
    pub async fn grow(map: Arc<DashMap<Coordinate, HexCell>>, radius: i16) {
        log::debug!("Start: Generated word map");
        let mut threads = Vec::new();
        let mut thread_range = vec![];
        thread_range.push((-radius, radius + 1));
        let cpu_count = num_cpus::get() as i16;

        if cpu_count > radius {
            thread_range.push((-radius, radius + 1));
        } else {
            let step_size = radius / cpu_count;
            let mut current_step = -radius;
            for i in 0..cpu_count {
                let mut next_step = current_step + step_size * 2;
                if i == cpu_count - 1 {
                    next_step = radius + 1;
                }
                thread_range.push((current_step, next_step));
                current_step = next_step;
            }
        }

        for i in thread_range {
            let m = map.clone();
            threads.push(tokio::task::spawn_blocking(move || {
                for x in i.0..i.1 + 1 {
                    let r1 = cmp::max(-radius, -x - radius);
                    let r2 = cmp::min(radius, -x + radius);
                    for r in r1..r2 + 1 {
                        m.insert(Coordinate::new(x, -x - r), HexCell::new());
                    }
                }
            }));
        }
        for thread in threads {
            thread.await.unwrap();
        }
        log::debug!("DONE: Generated word map with {} cells", map.iter().count());
    }
}