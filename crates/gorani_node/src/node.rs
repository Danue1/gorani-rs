use gorani_syntax::SyntaxNode;

pub trait Node: Sized {
    fn cast(node: SyntaxNode) -> Option<Self>;

    fn node(&self) -> SyntaxNode;
}

pub trait Children<Parent: Node>: Node {
    fn parent(&self) -> Option<Parent> {
        self.node().parent().and_then(Parent::cast)
    }
}

pub trait Parent<Children: Node>: Node {
    fn children(&self) -> impl Iterator<Item = Children> {
        self.node().children().filter_map(Children::cast)
    }

    fn child(&self) -> Option<Children> {
        self.node().children().find_map(Children::cast)
    }
}

impl<Children, Parent> crate::Children<Parent> for Children
where
    Children: Node + crate::Parent<Children>,
    Parent: Node,
{
    fn parent(&self) -> Option<Parent> {
        self.node().parent().and_then(Parent::cast)
    }
}
