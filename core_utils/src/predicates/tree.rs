use super::case::Case;
use crate::displayable::Displayable;
use std::fmt;
use termtree;

/// 定义一个树结点类型
type CaseTreeInner = termtree::Tree<Displayable>;

/// A `Case` rendered as a tree for display.
pub struct CaseTree(CaseTreeInner);

impl fmt::Display for CaseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// 根据 case 构造一颗termtree树
fn convert(case: &Case<'_>) -> CaseTreeInner {
    // 定义多个叶子结点，存放断言的多个参数结点 和 多个意外结果结点
    let mut leaves: Vec<CaseTreeInner> = vec![];

    leaves.extend(case.predicate().iter().flat_map(|pred| {
        // 获得断言的参数迭代器
        pred.parameters().map(|item| {
            // 创建断言参数结点
            let root = Displayable::new(&item);
            termtree::Tree::new(root).with_multiline(true)
        })
    }));

    leaves.extend(case.products().map(|item| {
        // 创建断言意外结果结点
        let root = Displayable::new(item);
        termtree::Tree::new(root).with_multiline(true)
    }));

    // 创建断言子结点
    leaves.extend(case.children().map(convert));

    // 创建根结点
    let root =
        case.predicate().map(|p| Displayable::new(&p)).unwrap_or_default();

    // 关联根结点和叶子结点
    CaseTreeInner::new(root).with_leaves(leaves)
}

/// Render `Self` as a displayable tree.
pub trait CaseTreeExt {
    /// Render `Self` as a displayable tree.
    fn tree(&self) -> CaseTree;
}

impl<'a> CaseTreeExt for Case<'a> {
    fn tree(&self) -> CaseTree {
        // 根据 case 构造一颗termtree树
        CaseTree(convert(self))
    }
}
