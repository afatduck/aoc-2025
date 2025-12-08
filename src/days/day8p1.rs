#[allow(dead_code)]

const N_SHORTEST_PAIRS: usize = 1000;

struct JunctionBox{
    x: f32,
    y: f32,
    z: f32,
    connected_to: Vec<usize>
}

struct Connection {
    a: usize,
    b: usize,
    distance: f32
}

impl JunctionBox {
    fn distance_squared(&self, to: &JunctionBox) -> f32 {
        (self.x - to.x).powi(2) + (self.y - to.y).powi(2) + (self.z - to.z).powi(2)
    }

    fn from(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z, connected_to: Vec::new()
        }
    }

    fn connect(&mut self, index: usize) {
        self.connected_to.push(index);
    }
}

impl Connection {
    fn from(a: usize, b: usize, distance: f32) -> Self {
        Self {
            a, b, distance
        }
    }
}

pub fn solve(input: String) {

    let mut boxes: Vec<JunctionBox> = Vec::new(); 
    let mut connections: Vec<Connection> = Vec::new(); 

    for (i, line) in input.split_ascii_whitespace().enumerate() {
        let mut values = line.split(",");
        let x: f32 = values.next().unwrap().parse().unwrap();
        let y: f32 = values.next().unwrap().parse().unwrap();
        let z: f32 = values.next().unwrap().parse().unwrap();
        let jbox = JunctionBox::from(x, y, z);
        
        for (j, other) in boxes.iter().enumerate() {
            connections.push(
                Connection::from(i, j, jbox.distance_squared(other))
            );
        }

        boxes.push(jbox);
    }

    connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    for i in 0..N_SHORTEST_PAIRS {
        let con = &connections[i];
        boxes[con.a].connect(con.b);
        boxes[con.b].connect(con.a);
    }

    let mut sizes: Vec<u64> = Vec::new();
    let mut visited: Vec<usize> = Vec::new(); 
    let n = boxes.len();

    for i in 0..n {
        if !visited.contains(&i) {
            visited.push(i);
            let mut size = 1u64;
            let mut search_stack = boxes[i].connected_to.clone();
            while !search_stack.is_empty() {
                let j = search_stack.pop().unwrap();
                if !visited.contains(&j) {
                    visited.push(j);
                    size += 1;
                    search_stack.extend(boxes[j].connected_to.iter());
                }
            }
            sizes.push(size);
        } 
    }

    sizes.sort();

    let mut result = 1u64;
    for i in sizes.len()-3..sizes.len() {
        result *= sizes[i];
    }
    println!("{result}")

}