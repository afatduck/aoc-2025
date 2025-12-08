#[allow(dead_code)]

struct JunctionBox{
    x: f32,
    y: f32,
    z: f32
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
            x, y, z
        }
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

    let mut visited: Vec<usize> = Vec::new(); 

    for i in 0..connections.len() {
        let con = &connections[i];
        if !visited.contains(&con.a) {
            visited.push(con.a);
        }
        if !visited.contains(&con.b) {
            visited.push(con.b);
        }
        if visited.len() == boxes.len() {
            let result = boxes[con.a].x * boxes[con.b].x;
            println!("{result}");
            break;
        }
    }


}