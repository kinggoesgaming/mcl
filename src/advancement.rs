pub struct Advancement<'p> {
    children: Vec<Self>,
    parent: Option<&'p Self>,
}

impl<'p> Advancement<'p> {
    /// Gets all the children [Advancement]s.
    pub fn children(&self) -> &'_ [Self] {
        &self.children
    }

    /// Gets the parent [Advancement].
    pub const fn parent(&self) -> Option<&'p Advancement> {
        self.parent
    }
}
