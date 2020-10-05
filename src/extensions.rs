use gdnative::prelude::*;

pub trait NodeExt {
    /// Gets a node at `path`, assumes that it's safe to use, and casts it to `T`.
    ///
    /// # Safety
    ///
    /// See `Ptr::assume_safe`.
    unsafe fn get_typed_node<T, P>(&self, path: P) -> TRef<'_, T, Shared>
    where
        T: GodotObject + SubClass<Node>,
        P: Into<NodePath>;
}

impl NodeExt for Node {
    unsafe fn get_typed_node<T, P>(&self, path: P) -> TRef<'_, T, Shared>
    where
        T: GodotObject + SubClass<Node>,
        P: Into<NodePath>,
    {
        self.get_node(path.into())
            .expect("node should exist")
            .assume_safe()
            .cast()
            .expect("node should be of the correct type")
    }
}

pub trait Vector2Ext {
    fn up() -> Self;
    fn down() -> Self;
    fn left() -> Self;
    fn right() -> Self;
}

impl Vector2Ext for Vector2 {
    fn up() -> Self {
        Vector2::new(0.0, -1.0)
    }
    fn down() -> Self {
        Vector2::new(0.0, 1.0)
    }
    fn left() -> Self {
        Vector2::new(-1.0, 0.0)
    }
    fn right() -> Self {
        Vector2::new(1.0, 0.0)
    }
}
