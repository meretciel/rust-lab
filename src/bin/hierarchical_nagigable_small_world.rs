use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use nalgebra::point;
use rand::Rng;
use rust_lab::numerical_utils::{dot, sub};
use rust_lab::logging;
use log::LevelFilter;

fn dist_sq(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let z = sub(&x, &y);
    dot(&z, &z)
}

#[derive(Debug)]
struct Point {
    index: usize,
    value: Vec<f64>,
    neighbors_by_layer: HashMap<usize, HashSet<usize>>,
}

impl Point {
    fn add_neighbor_bi(&mut self, other: &Rc<RefCell<Point>>, layer: usize) {
        log::info!("[connect] layer={}, from={}, to={}", layer, self.index, other.borrow().index);
        let neighbors_at_layer = self.neighbors_by_layer.get_mut(&layer);
        if let Some(v) = neighbors_at_layer {
            v.insert(other.borrow().index);
        } else {
            let mut s = HashSet::new();
            s.insert(other.borrow().index);
            self.neighbors_by_layer.insert(layer, s);
        }


        let mut binding = other.borrow_mut();
        let other_neighbors_at_layer = binding.neighbors_by_layer.get_mut(&layer);

        if let Some(v) = other_neighbors_at_layer {
            v.insert(self.index);
        } else {
            let mut s = HashSet::new();
            s.insert(self.index);
            binding.neighbors_by_layer.insert(layer, s);
        }
    }

    // /// Sets new neighbors at the given layer.
    // /// @param new_neighbors: List of new neighbors.
    // /// @param layer: The provided layer.
    // fn set_neighbors(&mut self, new_neighbors: &Vec<Rc<RefCell<Point>>>, layer: usize) {
    //     let _index = self.index;
    //     println!("Set neighbors for data point [{_index}]");
    //     // Remove connections between self and current neighbors
    //     // for item in new_neighbors.iter() {
    //     //     let _ = item.borrow_mut().neighbors_by_layer.get_mut(&layer).map(
    //     //         |neighbors| {neighbors.remove(&self.index);}
    //     //     );
    //     // }
    //
    //     if let Some(existing_neighbors) = self.neighbors_by_layer.get_mut(&layer) {
    //         existing_neighbors.iter().for_each(
    //             |point_index| {
    //                 if let Some(connections) = point.borrow_mut().neighbors_by_layer.get_mut(&layer) {
    //                     connections.remove(&target_index);
    //                 }
    //             }
    //         )
    //     }
    //
    //     // Add new neighbors
    //     for item in new_neighbors.iter() {
    //         self.add_neighbor_bi(item, layer);
    //     }
    // }

    fn set_new_neighbors(target: &Rc<RefCell<Point>>,
                         existing_neighbors: &Vec<Rc<RefCell<Point>>>,
                         new_neighbors: &Vec<Rc<RefCell<Point>>>, layer: usize) {

        let target_index = target.borrow().index;
        existing_neighbors.iter().for_each(
            |point| {
                let point_index = point.borrow().index;
                // Delete edge on the neighbor side (i.e. from neighbor to target)
                if let Some(connections) = point.borrow_mut().neighbors_by_layer.get_mut(&layer) {
                    connections.remove(&target_index);
                    log::info!("[disconnect] layer={}, from={}, to={}", layer, target_index, point_index);
                }
                // Delete edge on the target side (i.e. from target to neighbor)
                if let Some(connections) = target.borrow_mut().neighbors_by_layer.get_mut(&layer) {
                    connections.remove(&point_index);
                }
            }
        );

        for neighbor in new_neighbors.iter() {
            target.borrow_mut().add_neighbor_bi(neighbor, layer);
            // if target_index != neighbor.borrow().index {
            //     target.borrow_mut().add_neighbor_bi(neighbor, layer);
            // }
        }
    }
}


#[derive(Debug)]
struct Elem(Rc<RefCell<Point>>, f64);

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Elem {}


impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct DataStructure {
    layer_coeff: f64,
    points: Vec<Rc<RefCell<Point>>>,
    top_layer: usize,
    top_layer_enter_point: Option<Rc<RefCell<Point>>>,
}


impl DataStructure {
    const MAX_LAYERS: usize = 10;

    fn new(layer_coeff: f64) -> DataStructure {
        DataStructure {
            layer_coeff,
            points: Vec::new(),
            top_layer: 0,
            top_layer_enter_point: None,
        }
    }

    fn get_point_by_index(&self, index: usize) -> Rc<RefCell<Point>> {
        Rc::clone(self.points.get(index).unwrap())
    }

    fn get_sample_layer(&self) -> usize {
        // return 0;
        let mut rng = rand::thread_rng();
        let mut value = rng.gen::<f64>();
        while value == 0. {
            value = rng.gen::<f64>();
        }

        let tt = -value.ln() * self.layer_coeff;
        println!("DEBUG: value={tt}");

        Self::MAX_LAYERS.min((-value.ln() * self.layer_coeff).floor() as usize)
    }


    fn insert_point(&mut self, value: Vec<f64>, max_connections: usize) {
        // First, assign the internal index to the data point
        let index = self.points.len();
        let point = Point {
            index,
            value,
            neighbors_by_layer: HashMap::new(),
        };
        let new_point = Rc::new(RefCell::new(point));
        self.points.push(Rc::clone(&new_point));

        let layer = self.get_sample_layer();
        let ctl = self.top_layer;
        println!("Sampled layer: {layer}. Current top layer: {ctl}");

        if layer > self.top_layer || self.top_layer_enter_point.is_none() {
            self.top_layer_enter_point = Some(Rc::clone(&new_point));
        }

        if let Some(top_layer_enter_point) = &self.top_layer_enter_point {
            let mut enter_points = vec![Rc::clone(&top_layer_enter_point)];


            for k in (layer + 1..=self.top_layer).rev() {
                let query = &new_point.borrow().value;
                let w = self.search_layer(query, &enter_points, 1, k);
                enter_points = vec![Rc::clone(&w.first().unwrap())];
            }


            for k in (0..=layer.min(self.top_layer)).rev() {
                let enter_point_indexes: Vec<usize> = enter_points.iter().map(|x| {x.borrow().index}).collect();
                log::info!("[enter points] layer={}, data={:?}", k, enter_point_indexes);
                let candidates = self.search_layer(
                    &new_point.borrow().value,
                    &enter_points,
                    max_connections,
                    k);
                let neighbors = DataStructure::select_neighbors(
                    &new_point.borrow().value,
                    &candidates,
                    max_connections,
                );

                // Add bi-directional connections
                for nbr in neighbors.iter().filter(
                    |nbr_point| {!Rc::ptr_eq(&new_point, nbr_point)}
                ) {
                    new_point.borrow_mut().add_neighbor_bi(nbr, k);;
                }

                // Go through the neighbors again and remove connections if the number exceeds
                // the limit.
                for nbr in neighbors.iter() {
                    let existing_neighbors: Vec<Rc<RefCell<Point>>> = nbr.borrow().neighbors_by_layer.get(&k).map(
                        |neighbors| {
                            neighbors.iter().map(|&index| {self.get_point_by_index(index)}).collect()
                        }
                    ).unwrap_or_default();

                    let num_of_neighbors = existing_neighbors.len();
                    if num_of_neighbors > max_connections {
                        let new_neighbors = DataStructure::select_neighbors(
                            &nbr.borrow().value, &existing_neighbors, max_connections,
                        );
                        println!(">>> Number of new neighbors: {}", new_neighbors.len());
                        Point::set_new_neighbors(nbr, &existing_neighbors, &new_neighbors, k);
                    }
                }

                enter_points = candidates;
            }

            if layer > self.top_layer {
                self.top_layer = layer;
            }
        } else {
            panic!("Top layer enter point is not available.")
        }
    }


    /// Select neighbors from the candidates.
    ///
    /// @param query: The target or the query data point.
    /// @param candidates: A list of candidates from which we select the neighbors.
    /// @param neighbor_size: The max number of selected neighbors.
    /// @return: A list of points selected as neighbors of the query data point. The number of
    ///     selected neighbors is less than or equal to the neighbor_size.
    fn select_neighbors(query: &Vec<f64>,
                        candidates: &Vec<Rc<RefCell<Point>>>,
                        neighbor_size: usize) -> Vec<Rc<RefCell<Point>>> {
        let mut results = BinaryHeap::new();

        for point in candidates.iter() {
            let distance_score = dist_sq(query, &point.borrow().value);
            results.push(Reverse(Elem(Rc::clone(point), distance_score)));

            if results.len() > neighbor_size {
                let _ = results.pop();
            }
        }

        results.into_sorted_vec().into_iter().map(|elem| {
            Rc::clone(&elem.0.0)
        }).collect()

        // // Add elements to the working set
        // for point in candidates.iter() {
        //     let distance_score = dist_sq(query, &point.borrow().value);
        //     working_set.push(Elem(Rc::clone(point), distance_score))
        // }
        // println!("DEBUG: size of candidates: {}", candidates.len());
        // while !working_set.is_empty() && results.len() < neighbor_size {
        //     println!("DEBUG: checkpoint 100");
        //     let elem = working_set.pop().unwrap();
        //     let elem_distance_sq = elem.1;
        //     let best_in_result = results.peek();
        //
        //     match best_in_result {
        //         None => results.push(Reverse(Elem(Rc::clone(&elem.0), elem_distance_sq))),
        //         Some(v) if elem_distance_sq < v.0.1 => results.push(Reverse(Elem(Rc::clone(&elem.0), elem_distance_sq))),
        //         Some(v) => println!("discard element {elem:?} in the working set.")
        //     }
        // }
        //
        // results.into_sorted_vec().into_iter().map(|elem| {
        //     Rc::clone(&elem.0.0)
        // }).collect()
    }

    /// Search the layer.
    /// @param query:
    /// @param enter_points:
    /// @param max_num_results
    /// @param layer
    /// @return: A sorted array of points. The first element is the closest to the query.
    fn search_layer(&mut self, query: &Vec<f64>, enter_points: &Vec<Rc<RefCell<Point>>>, max_num_results: usize, layer: usize)
                    -> Vec<Rc<RefCell<Point>>> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut candidates = BinaryHeap::new();
        let mut results = BinaryHeap::new();

        for item in enter_points.iter() {
            visited.insert(item.borrow().index);
            candidates.push(Elem(Rc::clone(item), dist_sq(query, &item.borrow().value)));
            results.push(Reverse(Elem(Rc::clone(item), dist_sq(query, &item.borrow().value))));
        }

        while !candidates.is_empty() {

            let nearest_candidate_elem = candidates.pop().unwrap();
            if !results.is_empty() && nearest_candidate_elem.1 > results.peek().unwrap().0.1 {
                break;
            }

            let l1 = candidates.len();
            let l2 = results.len();
            let l3 = nearest_candidate_elem.1;
            let l4 = results.peek().map(|x| {x.0.1}).unwrap_or(-1.);
            println!("DEBUG: checkpoint 101: candidate size: {l1}, result size: {l2}, best_candidate_dist: {l3}, worst_result_dist: {l4}.");
            let vv: Vec<usize> = candidates.iter().map(|x|{x.0.borrow().index}).collect();
            let target_index = nearest_candidate_elem.0.borrow().index;
            println!("DEBUG: target point: {target_index}, candidates: {vv:?}");

            let binding = nearest_candidate_elem.0.borrow();
            let empty_set = HashSet::new();
            let neighbors = binding.neighbors_by_layer.get(&layer).unwrap_or(&empty_set);
            // for neighbor in neighbors.iter()
            for &neighbor_index in neighbors.into_iter() {
                if !visited.contains(&neighbor_index) {
                    println!("data point {neighbor_index} is added to visited set.");
                    visited.insert(neighbor_index);

                    let neighbor_point = self.get_point_by_index(neighbor_index);
                    let neighbor_to_query_dist_sq = dist_sq(query, &neighbor_point.borrow().value);

                    if results.len() < max_num_results || neighbor_to_query_dist_sq < results.peek().unwrap().0.1 {
                        candidates.push(Elem(Rc::clone(&neighbor_point), neighbor_to_query_dist_sq));
                        results.push(Reverse(Elem(Rc::clone(&neighbor_point), neighbor_to_query_dist_sq)));
                    }

                    if results.len() > max_num_results {
                        results.pop();
                    }
                }
            }
        }

        results.into_sorted_vec().into_iter().map(|reversed_elem| {
            self.get_point_by_index(reversed_elem.0.0.borrow().index)
        }).collect()
    }
}


fn main() {
    let data = vec![
        vec![0.5766059790780005, 0.9711961293853643],
        vec![0.77655302936061, 0.5806781117297726],
        vec![0.8480399683239941, 0.9995324064208375],
        vec![0.6907604750768213, 0.5843680568704989],
        vec![0.26793293043268723, 0.8925478670580446],
        vec![0.6304274899026008, 0.5986934514013911],
        vec![0.9122148030868045, 0.43433052252404525],
        vec![0.8314645600883519, 0.22451968609171075],
        vec![0.5040571728251309, 0.19978768307779832],
        vec![0.21740409667198302, 0.2708114287433756],
        vec![0.3713995210440875, 0.42030753038882634],
        vec![0.20444677234448294, 0.5264353699632782],
        vec![0.9911313247091867, 0.5352338864251166],
        vec![0.639947759425933, 0.8054641370409904],
        vec![0.20397096994544672, 0.19908474055129144],
        vec![0.12588512114685743, 0.5023778814840922],
        vec![0.6409698102728839, 0.7655894665652356],
        vec![0.2533643477256286, 0.5877343799693737],
        vec![0.3107710799213426, 0.4398525274461979],
        vec![0.25896778808242443, 0.3139579853256216],
        vec![0.45610057272890625, 0.40135870196395673],
        vec![0.8484867559200224, 0.24360757331685307],
        vec![0.4475468609613579, 0.9577932121239567],
        vec![0.2763339775523377, 0.5135256523026364],
        vec![0.8438832335290395, 0.6392865688124052],
        vec![0.8160936732443741, 0.6317694243569391],
        vec![0.0758525609459981, 0.4365703296751756],
        vec![0.4944769813126946, 0.03564137946093293],
        vec![0.5144627469853016, 0.2818478617532021],
        vec![0.9820094572835955, 0.37153255057356427],
        vec![0.0719945618276487, 0.4511216226974618],
        vec![0.007319961262311325, 0.10100469029436943],
        vec![0.44953819773205983, 0.36073666785776204],
        vec![0.7495205325431399, 0.5607587595912158],
        vec![0.2575702419864566, 0.28211198800383597],
        vec![0.7695568330188054, 0.32212413948336227],
        vec![0.8940707871715257, 0.8866002103213477],
        vec![0.1884710784915636, 0.901957247624211],
        vec![0.11216716704899259, 0.3319812710974426],
        vec![0.34209641185261874, 0.03510261936131809],
        vec![0.9448309896044007, 0.5390708226818269],
        vec![0.23911572739829054, 0.26263939052214674],
        vec![0.20965508974033628, 0.6638416940764855],
        vec![0.13730249446598977, 0.6073977683733214],
        vec![0.011841793109952148, 0.404646976259144],
        vec![0.8158830919633735, 0.4156249836402941],
        vec![0.20527616212959934, 0.36000031429033],
        vec![0.16271712675431668, 0.00045843924934785285],
        vec![0.3983271080269005, 0.4140523786325794],
        vec![0.6921293538194214, 0.28581223471918227],
        vec![0.6395375190233525, 0.9840862726902118],
        vec![0.28461509607768304, 0.5846829676083324],
        vec![0.23320627370328578, 0.16636376910079587],
        vec![0.31107398766399463, 0.4820127021400047],
        vec![0.27578211874530323, 0.9064440849142474],
        vec![0.9131112518138766, 0.9013978401328716],
        vec![0.13842156760764376, 0.9401924924424347],
        vec![0.22910065188503637, 0.727173948843218],
        vec![0.6821949808424383, 0.8169671820516788],
        vec![0.15291780315421047, 0.5966510583185544],
        vec![0.8464595774277084, 0.861415131772832],
        vec![0.8422333867220629, 0.27206333406248406],
        vec![0.9644484192116131, 0.4610377712937661],
        vec![0.8778242841680249, 0.43776441950668066],
        vec![0.8553122599908717, 0.15159680211255322],
        vec![0.4889606317642766, 0.4204393749953309],
        vec![0.076071083945463, 0.33150302304419343],
        vec![0.7283903056116425, 0.320285998392953],
        vec![0.09090399368785133, 0.4517582385683716],
        vec![0.41352535823916425, 0.7728014134343422],
        vec![0.12403006252526709, 0.9774598730625159],
        vec![0.9507759522680268, 0.7648499206538416],
        vec![0.8255266389427105, 0.033590957833872115],
        vec![0.33719853283445533, 0.3986204519708101],
        vec![0.02651673909729016, 0.2824208534177163],
        vec![0.4072576979437046, 0.6393829726637481],
        vec![0.5130390776018232, 0.13245948289069803],
        vec![0.21162629355259233, 0.49569048318024544],
        vec![0.21207171688271567, 0.3455420261875536],
        vec![0.5077525694363003, 0.036513943308372154],
        vec![0.32205448405670145, 0.8277209984524078],
        vec![0.5657911558800469, 0.6191085455809284],
        vec![0.45087509985012486, 0.3521839534819857],
        vec![0.8408632774230439, 0.4219389537244291],
        vec![0.2132850434916354, 0.5911806719503162],
        vec![0.39989822245310513, 0.7804215307760525],
        vec![0.3964395059100331, 0.8478045984956097],
        vec![0.563931855675232, 0.29093936891302846],
        vec![0.02539951584397973, 0.595737297449052],
        vec![0.3831458356903965, 0.4757143942037731],
        vec![0.18467440968497895, 0.9251017862348352],
        vec![0.6586950460112418, 0.9308158245868613],
        vec![0.6628822717179358, 0.5626610232974152],
        vec![0.9410626349211688, 0.5126106459116356],
        vec![0.478109607456365, 0.3207724056736661],
        vec![0.7103036575108707, 0.3214631366373614],
        vec![0.013465283055164534, 0.5397061000373409],
        vec![0.990498893468343, 0.6216827014342146],
        vec![0.4273794248349956, 0.5834775014703776],
        vec![0.4700494505114975, 0.1659884618814622],
        vec![0.9876259297466461, 0.8762402299087949],
        vec![0.3323502161109875, 0.7184225652716443],
        vec![0.9627992105148855, 0.016063459064336538],
    ];

    logging::init_logging(LevelFilter::Info);
    let layer_coeff = 1.8;
    let max_connection = 3;
    let mut hnsw = DataStructure::new(layer_coeff);
    let mut c = 0;
    for v in data.into_iter() {
        log::info!("[start] iteration: {} ------------------------------------------------", c);
        hnsw.insert_point(v, max_connection);
        log::info!("[end] iteration: {}", c);

        c += 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_heap() {
        let v1 = Point {index: 0, value: vec![0., 3.], neighbors_by_layer: HashMap::new()};
        let v2 = Point {index: 1, value: vec![5., 0.], neighbors_by_layer: HashMap::new()};
        let query = vec![0., 0.];
        let mut heap = BinaryHeap::new();

        let d1 = dist_sq(&query, &v1.value);
        let d2 = dist_sq(&query, &v2.value);

        heap.push(Elem(Rc::new(RefCell::new(v1)), d1));
        heap.push(Elem(Rc::new(RefCell::new(v2)), d2));

        let top = heap.pop().unwrap();
        assert_eq!(0, top.0.borrow().index);
    }

    #[test]
    fn test_max_heap() {
        let v1 = Point {index: 0, value: vec![0., 3.], neighbors_by_layer: HashMap::new()};
        let v2 = Point {index: 1, value: vec![5., 0.], neighbors_by_layer: HashMap::new()};
        let query = vec![0., 0.];
        let mut heap = BinaryHeap::new();

        let d1 = dist_sq(&query, &v1.value);
        let d2 = dist_sq(&query, &v2.value);

        heap.push(Reverse(Elem(Rc::new(RefCell::new(v1)), d1)));
        heap.push(Reverse(Elem(Rc::new(RefCell::new(v2)), d2)));

        let top = heap.pop().unwrap();
        assert_eq!(1, top.0.0.borrow().index);
    }

    #[test]
    fn test_comparison() {
        let v1 = Point {index: 0, value: vec![0., 3.], neighbors_by_layer: HashMap::new()};
        let v2 = Point {index: 1, value: vec![5., 0.], neighbors_by_layer: HashMap::new()};
        let query = vec![0., 0.];
        let d1 = dist_sq(&query, &v1.value);
        let d2 = dist_sq(&query, &v2.value);

        let elem1 = Elem(Rc::new(RefCell::new(v1)), d1);
        let elem2 =  Elem(Rc::new(RefCell::new(v2)), d2);

        assert_eq!(Ordering::Greater, elem1.partial_cmp(&elem2).unwrap());
    }

}