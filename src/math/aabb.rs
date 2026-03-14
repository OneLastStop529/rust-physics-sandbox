use crate::math::Vec2;

#[derive(Clone, Copy)]
// An axis-aligned bounding box (Aabb) is a rectangle defined by its center point and half extents along the x and y axes. The half extents represent the distance from the center to the min and max corners of the Aabb along each axis.
pub struct Aabb {
    pub center: Vec2,
    pub half_extents: Vec2,
}

impl Aabb {
    // Constructs an Aabb from a center point and half extents. The half extents represent the distance from the center to the min and max corners of the Aabb along each axis.
    pub fn from_center_half_extents(center: Vec2, half_extents: Vec2) -> Self {
        Self {
            center,
            half_extents,
        }
    }

    pub fn min(&self) -> Vec2 {
        self.center - self.half_extents
    }

    pub fn max(&self) -> Vec2 {
        self.center + self.half_extents
    }
}
