/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::uniontypes::NodeOrString;
use dom::bindings::error::{ErrorResult, Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::{Castable, NodeTypeId};
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::Element;
use dom::htmlcollection::HTMLCollection;
use dom::node::{Node};
use dom::nodelist::NodeList;
use string_cache::Atom;
use util::str::DOMString;

// https://dom.spec.whatwg.org/#documentfragment

pub struct DocumentFragment {
    node: Node,
}

impl DocumentFragment {
    /// Creates a new DocumentFragment.
    fn new_inherited(id: u64, document: &Document) -> DocumentFragment {
        DocumentFragment {
            node: Node::new_inherited(NodeTypeId::DocumentFragment, id, document),
        }
    }

    pub fn new(id: u64, document: &Document) -> Root<DocumentFragment> {
        Root::new_box(box DocumentFragment::new_inherited(id, document))
    }
    
    // https://dom.spec.whatwg.org/#dom-parentnode-children
    fn Children(&self) -> Root<HTMLCollection> {
        HTMLCollection::children(self.upcast())
    }

    // https://dom.spec.whatwg.org/#dom-nonelementparentnode-getelementbyid
    fn GetElementById(&self, id: DOMString) -> Option<Root<Element>> {
        let node = self.upcast::<Node>();
        let id = Atom::from(id);
        node.traverse_preorder().filter_map(Root::downcast::<Element>).find(|descendant| {
            match descendant.get_attribute(&ns!(), &atom!("id")) {
                None => false,
                Some(attr) => *attr.value().as_atom() == id,
            }
        })
    }

    // https://dom.spec.whatwg.org/#dom-parentnode-firstelementchild
    fn GetFirstElementChild(&self) -> Option<Root<Element>> {
        self.upcast::<Node>().child_elements().next()
    }

    // https://dom.spec.whatwg.org/#dom-parentnode-lastelementchild
    fn GetLastElementChild(&self) -> Option<Root<Element>> {
        self.upcast::<Node>().rev_children().filter_map(Root::downcast::<Element>).next()
    }

    // https://dom.spec.whatwg.org/#dom-parentnode-childelementcount
    fn ChildElementCount(&self) -> u32 {
        self.upcast::<Node>().child_elements().count() as u32
    }

    // https://dom.spec.whatwg.org/#dom-parentnode-queryselector
    fn QuerySelector(&self, selectors: DOMString) -> Fallible<Option<Root<Element>>> {
        self.upcast::<Node>().query_selector(selectors)
    }

    // https://dom.spec.whatwg.org/#dom-parentnode-queryselectorall
    fn QuerySelectorAll(&self, selectors: DOMString) -> Fallible<Root<NodeList>> {
        self.upcast::<Node>().query_selector_all(selectors)
    }
}
