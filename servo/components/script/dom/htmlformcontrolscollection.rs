/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLCollectionBinding::HTMLCollectionMethods;
use dom::bindings::codegen::Bindings::HTMLFormControlsCollectionBinding::HTMLFormControlsCollectionMethods;
use dom::bindings::inheritance::{HTMLCollectionTypeId};
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflectable};
use dom::element::Element;
use dom::htmlcollection::{CollectionFilter, HTMLCollection};
use dom::node::Node;


pub struct HTMLFormControlsCollection {
    collection: HTMLCollection,
}

impl HTMLFormControlsCollection {
    fn new_inherited(root: &Node, filter: Box<CollectionFilter + 'static>) -> HTMLFormControlsCollection {
        HTMLFormControlsCollection {
            collection: HTMLCollection::new_inherited(HTMLCollectionTypeId::HTMLFormControlsCollection, root, filter)
        }
    }

    pub fn new(root: &Node, filter: Box<CollectionFilter + 'static>)
        -> Root<HTMLFormControlsCollection>
    {
        Root::new_box(box HTMLFormControlsCollection::new_inherited(root, filter))
    }

    // FIXME: This shouldn't need to be implemented here since HTMLCollection (the parent of
    // HTMLFormControlsCollection) implements Length
    pub fn Length(&self) -> u32 {
        self.collection.Length()
    }


    // FIXME: This shouldn't need to be implemented here since HTMLCollection (the parent of
    // HTMLFormControlsCollection) implements IndexedGetter.
    // https://github.com/servo/servo/issues/5875
    //
    // https://dom.spec.whatwg.org/#dom-htmlcollection-item
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<Element>> {
        self.collection.IndexedGetter(index, found)
    }
}
