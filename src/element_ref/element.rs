use selectors::{Element, OpaqueElement};
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::context::VisitedHandlingMode;
use html5ever::{LocalName, Namespace};
use selectors::matching;

use super::ElementRef;
use selector::{NonTSPseudoClass, PseudoElement, Simple};

/// Note: will never match against non-tree-structure pseudo-classes.
impl<'a> Element for ElementRef<'a> {
    type Impl = Simple;

    fn parent_element(&self) -> Option<Self> {
        self.parent().and_then(ElementRef::wrap)
    }

    fn first_child_element(&self) -> Option<Self> {
        self.children()
            .find(|child| child.value().is_element())
            .map(ElementRef::new)
    }

    fn last_child_element(&self) -> Option<Self> {
        self.children()
            .rev()
            .find(|child| child.value().is_element())
            .map(ElementRef::new)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.value().name.ns == ns!(html)
    }

    fn get_local_name(&self) -> &LocalName {
        &self.value().name.local
    }

    fn get_namespace(&self) -> &Namespace {
        &self.value().name.ns
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        _pc: &NonTSPseudoClass,
        _context: &mut matching::MatchingContext<Self::Impl>,
        _visited_handling: VisitedHandlingMode,
        _flags_setter: &mut F,
    ) -> bool {
        false
    }

    fn has_class(&self, name: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        //self.value().classes.contains(name)
        self.value().has_class(name, case_sensitivity)
    }

    fn is_empty(&self) -> bool {
        !self.children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.value().is_document())
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &LocalName,
        operation: &AttrSelectorOperation<&String>,
    ) -> bool {
        self.value().attrs.iter().any(|(key, value)| {
            !matches!(*ns, NamespaceConstraint::Specific(url) if *url != key.ns)
                && *local_name == key.local && operation.eval_str(value)
        })
    }

    fn match_pseudo_element(
        &self,
        _pe: &PseudoElement,
        _context: &mut matching::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    // TODO
    fn is_link(&self) -> bool {
        false
    }

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(&self)
    }

    fn has_id(&self, id: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        false
    }
}
