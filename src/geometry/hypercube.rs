use std::fmt;

pub struct Hypercube {
    dims: u32,
    side_length: f64,
    pub vertices: Vec<Vec<f64>>,
}

// todo: Improve a better Debug output
// todo:    -> (look for some crate that does this)
// todo:    -> (or implement the Debug trait for the Hypercube struct) :c
impl fmt::Debug for Hypercube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hypercube {{ dims: {}, side_length: {}, vertices: np.array([\n",
            self.dims, self.side_length
        )?;
        for vertex in &self.vertices {
            write!(f, "    [")?;
            for (index, &val) in vertex.iter().enumerate() {
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.1}", val)?;
            }
            write!(f, "],\n")?;
        }
        write!(f, "])\n}}")
    }
}

impl Hypercube {
    pub fn new(dims: u32, side_length: f64) -> Self {
        let mut hypercube = Hypercube {
            dims,
            side_length,
            vertices: Vec::new(),
        };
        hypercube.generate_hypercube_vertices();
        hypercube
    }

    fn generate_hypercube_vertices(&mut self) {
        let num_vertices = 2usize.pow(self.dims);
        let mut vertices = Vec::with_capacity(num_vertices);

        // do the same as above using a=n anonymous function
        (0..num_vertices).for_each(|i| {
            vertices.push(
                (0..self.dims)
                    .map(|j| {
                        if (i >> j) & 1 == 1 {
                            self.side_length
                        } else {
                            0.0
                        }
                    })
                    .collect(),
            )
        });

        self.vertices = vertices;
    }
}
