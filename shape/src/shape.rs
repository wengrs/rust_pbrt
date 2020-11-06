use base::bounding::AABB;
pub trait Shape
{
    fn bound(&self) -> AABB;
}