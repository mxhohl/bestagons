use std::{array, ops::Index};

use crate::coordinates::Coordinates;

#[cfg(not(features = "use_flat_top"))]
const HEX_ANGLE_OFFSET: f64 = std::f64::consts::PI / 6.;

#[cfg(features = "use_flat_top")]
const HEX_ANGLE_OFFSET:f64 = 0f64;

struct HexagonGridGeometry {
    center: [f64; 2],
    size: [f64; 2],
}

impl HexagonGridGeometry {
    pub fn new(center: [f64; 2], size: [f64; 2]) -> Self {
        Self { center, size }
    }

    pub fn center(&self, coord: Coordinates) -> [f64; 2] {
        #[cfg(not(features = "use_flat_top"))]
        return [
            self.size[0] * (3f64.sqrt() * coord.q() as f64 + 3f64.sqrt() / 2. * coord.r() as f64),
            self.size[1] * (3. / 2. * coord.q() as f64),
        ];

        #[cfg(features = "use_flat_top")]
        return [
            self.size[0] * (3. / 2. * coord.q() as f64),
            self.size[1] * (3f64.sqrt() / 2. * coord.q() as f64 + 3f64.sqrt() * coord.r() as f64)
        ];
    }

    pub fn corners(&self, coord: Coordinates) -> [[f64; 2]; 6] {
        let center = self.center(coord);

        array::from_fn(|i| {
            let angle = std::f64::consts::PI / 3. * i as f64 + HEX_ANGLE_OFFSET;
            [
                center[0] + self.size[0] * angle.cos(),
                center[1] + self.size[1] * angle.sin(),
            ]
        })
    }

    pub fn corner(&self, coord:Coordinates, i: i32) -> [f64; 2] {
        let center = self.center(coord);
        let angle = std::f64::consts::PI / 3. * i as f64 + HEX_ANGLE_OFFSET;
        [
            center[0] + self.size[0] * angle.cos(),
            center[1] + self.size[1] * angle.sin(),
        ]
    }

    pub fn cell_at<T>(&self, point: T) -> Coordinates
    where T: Index<usize, Output = f64> {
        #[cfg(not(features = "use_flat_top"))]
        let q = (3f64.sqrt() / 3. * point[0] - 1. / 3. * point[2]) / self.size[0];
        #[cfg(not(features = "use_flat_top"))]
        let r = (2. / 3. * point[1]) / self.size[1];

        #[cfg(features = "use_flat_top")]
        let q = (2. / 3. * point[1]) / self.size[0];
        #[cfg(features = "use_flat_top")]
        let r = (-1. / 3. * point[0] + 3f64.sqrt() / 3. * point[1]) / self.size[1];

        Coordinates::from_floating(q, r, -q - r)
    }

}

struct HexagonGridGeometryBuilder {
    geometry: HexagonGridGeometry,
}

impl HexagonGridGeometryBuilder {
    pub fn new() -> HexagonGridGeometryBuilder {
        Self {
            geometry: HexagonGridGeometry {
                center: [0., 0.],
                size: [0., 0.],
            }
        }
    }

    pub fn center(mut self, x: f64, y: f64) -> Self {
        self.geometry.center = [x, y];
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.geometry.size = [size, size];
        self
    }

    pub fn size_irregular(mut self, x: f64, y: f64) -> Self {
        self.geometry.size = [x, y];
        self
    }

    pub fn build(self) -> HexagonGridGeometry {
        self.geometry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center() {
        let geometry = HexagonGridGeometryBuilder::new()
            .center(0., 0.)
            .build();

        assert_eq!(geometry.center(Coordinates::zero()), [0., 0.]);
    }
}
