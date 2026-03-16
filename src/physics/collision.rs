use crate::{
    math::{Aabb, Vec2},
    physics::{
        body::{BodyHandle, RigidBody},
        collider::Collider,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BodyPair {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CollisionStats {
    pub candidate_pairs: usize,
    pub collisions: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionHit {
    pub normal: Vec2,
    pub penetration: f32,
}

pub fn collect_collision_pairs(bodies: &[RigidBody]) -> Vec<BodyPair> {
    let mut pairs = Vec::new();

    for body_a in 0..bodies.len() {
        for body_b in (body_a + 1)..bodies.len() {
            if bodies[body_a].is_static() && bodies[body_b].is_static() {
                continue;
            }

            if bodies[body_a].collider.is_none() || bodies[body_b].collider.is_none() {
                continue;
            }

            pairs.push(BodyPair { body_a, body_b });
        }
    }

    pairs
}

pub fn detect_collision(body_a: &RigidBody, body_b: &RigidBody) -> Option<CollisionHit> {
    let collider_a = body_a.collider?;
    let collider_b = body_b.collider?;

    match (collider_a, collider_b) {
        (Collider::Circle { radius: radius_a }, Collider::Circle { radius: radius_b }) => {
            collide_circle_circle(body_a.position, radius_a, body_b.position, radius_b)
        }
        (
            Collider::Aabb {
                half_extents: half_extents_a,
            },
            Collider::Aabb {
                half_extents: half_extents_b,
            },
        ) => collide_aabb_aabb(
            Aabb::from_center_half_extents(
                body_a.position,
                Vec2::new(half_extents_a.0, half_extents_a.1),
            ),
            Aabb::from_center_half_extents(
                body_b.position,
                Vec2::new(half_extents_b.0, half_extents_b.1),
            ),
        ),
        (Collider::Circle { radius }, Collider::Aabb { half_extents }) => collide_circle_aabb(
            body_a.position,
            radius,
            Aabb::from_center_half_extents(
                body_b.position,
                Vec2::new(half_extents.0, half_extents.1),
            ),
        ),
        (Collider::Aabb { half_extents }, Collider::Circle { radius }) => collide_circle_aabb(
            body_b.position,
            radius,
            Aabb::from_center_half_extents(
                body_a.position,
                Vec2::new(half_extents.0, half_extents.1),
            ),
        )
        .map(|hit| CollisionHit {
            normal: hit.normal * -1.0,
            penetration: hit.penetration,
        }),
    }
}

pub fn collide_circle_circle(
    center_a: Vec2,
    radius_a: f32,
    center_b: Vec2,
    radius_b: f32,
) -> Option<CollisionHit> {
    let delta = center_b - center_a;
    let radius_sum = radius_a + radius_b;
    let distance_squared = delta.length_squared();

    if distance_squared > radius_sum * radius_sum {
        return None;
    }

    let distance = distance_squared.sqrt();
    let normal = if distance > 0.0 {
        delta * (1.0 / distance)
    } else {
        Vec2::new(1.0, 0.0)
    };

    Some(CollisionHit {
        normal,
        penetration: radius_sum - distance,
    })
}

pub fn collide_aabb_aabb(aabb_a: Aabb, aabb_b: Aabb) -> Option<CollisionHit> {
    let delta = aabb_b.center - aabb_a.center;
    let overlap_x = aabb_a.half_extents.x + aabb_b.half_extents.x - delta.x.abs();
    let overlap_y = aabb_a.half_extents.y + aabb_b.half_extents.y - delta.y.abs();

    if overlap_x < 0.0 || overlap_y < 0.0 {
        return None;
    }

    if overlap_x <= overlap_y {
        Some(CollisionHit {
            normal: Vec2::new(delta.x.signum_or_positive(), 0.0),
            penetration: overlap_x,
        })
    } else {
        Some(CollisionHit {
            normal: Vec2::new(0.0, delta.y.signum_or_positive()),
            penetration: overlap_y,
        })
    }
}

pub fn collide_circle_aabb(center: Vec2, radius: f32, aabb: Aabb) -> Option<CollisionHit> {
    let min = aabb.min();
    let max = aabb.max();
    let closest = center.clamp(min, max);
    let delta = closest - center;
    let distance_squared = delta.length_squared();

    if distance_squared > radius * radius {
        return None;
    }

    if distance_squared > 0.0 {
        let distance = distance_squared.sqrt();
        return Some(CollisionHit {
            normal: delta * (1.0 / distance),
            penetration: radius - distance,
        });
    }

    let offset = center - aabb.center;
    let distance_to_x_face = aabb.half_extents.x - offset.x.abs();
    let distance_to_y_face = aabb.half_extents.y - offset.y.abs();

    if distance_to_x_face <= distance_to_y_face {
        Some(CollisionHit {
            normal: Vec2::new(offset.x.signum_or_positive(), 0.0),
            penetration: radius + distance_to_x_face,
        })
    } else {
        Some(CollisionHit {
            normal: Vec2::new(0.0, offset.y.signum_or_positive()),
            penetration: radius + distance_to_y_face,
        })
    }
}

trait SignumOrPositive {
    fn signum_or_positive(self) -> f32;
}

impl SignumOrPositive for f32 {
    fn signum_or_positive(self) -> f32 {
        if self < 0.0 { -1.0 } else { 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BodyPair, collect_collision_pairs, collide_aabb_aabb, collide_circle_aabb,
        collide_circle_circle, detect_collision,
    };
    use crate::{
        math::{Aabb, Vec2},
        physics::{
            body::{BodyType, RigidBody},
            collider::Collider,
        },
    };

    #[test]
    fn skips_static_static_and_missing_collider_pairs() {
        let bodies = vec![
            RigidBody::new(
                0,
                BodyType::Static,
                Vec2::new(0.0, 0.0),
                Some(Collider::Circle { radius: 1.0 }),
            ),
            RigidBody::new(
                1,
                BodyType::Static,
                Vec2::new(2.0, 0.0),
                Some(Collider::Circle { radius: 1.0 }),
            ),
            RigidBody::new(
                2,
                BodyType::Dynamic,
                Vec2::new(4.0, 0.0),
                Some(Collider::Circle { radius: 1.0 }),
            ),
            RigidBody::new(3, BodyType::Dynamic, Vec2::new(6.0, 0.0), None),
        ];

        let pairs = collect_collision_pairs(&bodies);

        assert_eq!(
            pairs,
            vec![
                BodyPair {
                    body_a: 0,
                    body_b: 2
                },
                BodyPair {
                    body_a: 1,
                    body_b: 2
                }
            ]
        );
    }

    #[test]
    fn detects_circle_circle_overlap() {
        let hit =
            collide_circle_circle(Vec2::new(0.0, 0.0), 1.5, Vec2::new(2.0, 0.0), 1.0).unwrap();

        assert_eq!(hit.normal, Vec2::new(1.0, 0.0));
        assert!((hit.penetration - 0.5).abs() < 1e-6);
    }

    #[test]
    fn rejects_separated_circle_circle_pair() {
        let hit = collide_circle_circle(Vec2::new(0.0, 0.0), 1.0, Vec2::new(3.0, 0.0), 1.0);

        assert_eq!(hit, None);
    }

    #[test]
    fn detects_aabb_aabb_overlap() {
        let hit = collide_aabb_aabb(
            Aabb::from_center_half_extents(Vec2::new(0.0, 0.0), Vec2::new(2.0, 1.0)),
            Aabb::from_center_half_extents(Vec2::new(3.0, 0.25), Vec2::new(2.0, 1.0)),
        )
        .unwrap();

        assert_eq!(hit.normal, Vec2::new(1.0, 0.0));
        assert!((hit.penetration - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rejects_separated_aabb_pair() {
        let hit = collide_aabb_aabb(
            Aabb::from_center_half_extents(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
            Aabb::from_center_half_extents(Vec2::new(3.5, 0.0), Vec2::new(1.0, 1.0)),
        );

        assert_eq!(hit, None);
    }

    #[test]
    fn detects_circle_aabb_overlap() {
        let hit = collide_circle_aabb(
            Vec2::new(0.25, 1.5),
            1.0,
            Aabb::from_center_half_extents(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
        )
        .unwrap();

        assert_eq!(hit.normal, Vec2::new(0.0, -1.0));
        assert!((hit.penetration - 0.5).abs() < 1e-6);
    }

    #[test]
    fn rejects_separated_circle_aabb_pair() {
        let hit = collide_circle_aabb(
            Vec2::new(3.0, 3.0),
            0.5,
            Aabb::from_center_half_extents(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
        );

        assert_eq!(hit, None);
    }

    #[test]
    fn dispatches_shape_pairs_consistently() {
        let circle = RigidBody::new(
            0,
            BodyType::Dynamic,
            Vec2::new(0.25, 1.5),
            Some(Collider::Circle { radius: 1.0 }),
        );
        let aabb = RigidBody::new(
            1,
            BodyType::Static,
            Vec2::new(0.0, 0.0),
            Some(Collider::Aabb {
                half_extents: (1.0, 1.0),
            }),
        );

        let hit = detect_collision(&circle, &aabb).unwrap();

        assert_eq!(hit.normal, Vec2::new(0.0, -1.0));
        assert!((hit.penetration - 0.5).abs() < 1e-6);
    }
}
