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

    fn is_link(&self) -> bool {
        match self.value().attr("href") {
            Some(_) => true,
            None => false,
        }
    }

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(&self)
    }

    fn has_id(&self, id: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        match self.value().id {
            Some(ref val) => case_sensitivity.eq(id.as_bytes(), val.as_bytes()),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use html::Html;
    use selector::Selector;
    use selectors::Element;
    use selectors::attr::CaseSensitivity;

    #[test]
    fn test_has_id() {
        use html5ever::LocalName;

        let html = "<p id='link_id_456'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();

        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            true,
            element.has_id(
                &LocalName::from("link_id_456"),
                CaseSensitivity::CaseSensitive
            )
        );

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            false,
            element.has_id(
                &LocalName::from("any_link_id"),
                CaseSensitivity::CaseSensitive
            )
        );
    }

    #[test]
    fn test_is_link() {
        let html = "<a href='https://www.example.com'>Example website</a>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("a").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(true, element.is_link());

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(false, element.is_link());
    }

    #[test]
    fn test_has_class() {
        use html5ever::LocalName;
        let html = "<p class='my_class'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            true,
            element.has_class(&LocalName::from("my_class"), CaseSensitivity::CaseSensitive)
        );

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            false,
            element.has_class(&LocalName::from("my_class"), CaseSensitivity::CaseSensitive)
        );
    }

}
