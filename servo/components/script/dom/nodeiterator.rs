/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::js::{JS, MutHeap, Root};
use dom::node::Node;
use std::cell::Cell;
use std::rc::Rc;

pub mod NodeFilterConstants {
    pub const FILTER_ACCEPT: u16 = 1;
    pub const FILTER_REJECT: u16 = 2;
    pub const FILTER_SKIP: u16 = 3;
    pub const SHOW_ALL: u32 = 4294967295;
    pub const SHOW_ELEMENT: u32 = 1;
    pub const SHOW_ATTRIBUTE: u32 = 2;
    pub const SHOW_TEXT: u32 = 4;
    pub const SHOW_CDATA_SECTION: u32 = 8;
    pub const SHOW_ENTITY_REFERENCE: u32 = 16;
    pub const SHOW_ENTITY: u32 = 32;
    pub const SHOW_PROCESSING_INSTRUCTION: u32 = 64;
    pub const SHOW_COMMENT: u32 = 128;
    pub const SHOW_DOCUMENT: u32 = 256;
    pub const SHOW_DOCUMENT_TYPE: u32 = 512;
    pub const SHOW_DOCUMENT_FRAGMENT: u32 = 1024;
    pub const SHOW_NOTATION: u32 = 2048;
} // mod NodeFilterConstants

pub struct NodeIterator {
    root_node: JS<Node>,
    #[ignore_heap_size_of = "Defined in rust-mozjs"]
    reference_node: MutHeap<JS<Node>>,
    pointer_before_reference_node: Cell<bool>,
    what_to_show: u32,
    #[ignore_heap_size_of = "Can't measure due to #6870"]
    filter: Filter,
}

impl NodeIterator {
    fn new_inherited(root_node: &Node,
                     what_to_show: u32,
                     filter: Filter) -> NodeIterator {
        NodeIterator {
            root_node: JS::from_ref(root_node),
            reference_node: MutHeap::new(root_node),
            pointer_before_reference_node: Cell::new(true),
            what_to_show: what_to_show,
            filter: filter
        }
    }

    pub fn new_with_filter(root_node: &Node,
                           what_to_show: u32,
                           filter: Filter) -> Root<NodeIterator> {
        Root::new_box(box NodeIterator::new_inherited(root_node, what_to_show, filter))
    }

    pub fn new(root_node: &Node,
               what_to_show: u32) -> Root<NodeIterator> {
        let filter = Filter::None;
        NodeIterator::new_with_filter(root_node, what_to_show, filter)
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-root
    fn Root(&self) -> Root<Node> {
        Root::from_ref(&*self.root_node)
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-whattoshow
    fn WhatToShow(&self) -> u32 {
        self.what_to_show
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-referencenode
    fn ReferenceNode(&self) -> Root<Node> {
        self.reference_node.get()
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-pointerbeforereferencenode
    fn PointerBeforeReferenceNode(&self) -> bool {
        self.pointer_before_reference_node.get()
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-nextnode
    fn NextNode(&self) -> Fallible<Option<Root<Node>>> {
        // https://dom.spec.whatwg.org/#concept-NodeIterator-traverse
        // Step 1.
        let node = self.reference_node.get();

        // Step 2.
        let mut before_node = self.pointer_before_reference_node.get();

        // Step 3-1.
        if before_node {
            before_node = false;

            // Step 3-2.
            let result = try!(self.accept_node(node.r()));

            // Step 3-3.
            if result == NodeFilterConstants::FILTER_ACCEPT {
                // Step 4.
                self.reference_node.set(node.r());
                self.pointer_before_reference_node.set(before_node);

                return Ok(Some(node));
            }
        }

        // Step 3-1.
        for following_node in node.following_nodes(&self.root_node) {
            // Step 3-2.
            let result = try!(self.accept_node(following_node.r()));

            // Step 3-3.
            if result == NodeFilterConstants::FILTER_ACCEPT {
                // Step 4.
                self.reference_node.set(following_node.r());
                self.pointer_before_reference_node.set(before_node);

                return Ok(Some(following_node));
            }
        }

        Ok(None)
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-previousnode
    fn PreviousNode(&self) -> Fallible<Option<Root<Node>>> {
        // https://dom.spec.whatwg.org/#concept-NodeIterator-traverse
        // Step 1.
        let node = self.reference_node.get();

        // Step 2.
        let mut before_node = self.pointer_before_reference_node.get();

        // Step 3-1.
        if !before_node {
            before_node = true;

            // Step 3-2.
            let result = try!(self.accept_node(node.r()));

            // Step 3-3.
            if result == NodeFilterConstants::FILTER_ACCEPT {
                // Step 4.
                self.reference_node.set(node.r());
                self.pointer_before_reference_node.set(before_node);

                return Ok(Some(node));
            }
        }

        // Step 3-1.
        for preceding_node in node.preceding_nodes(&self.root_node) {

            // Step 3-2.
            let result = try!(self.accept_node(preceding_node.r()));

            // Step 3-3.
            if result == NodeFilterConstants::FILTER_ACCEPT {
                // Step 4.
                self.reference_node.set(preceding_node.r());
                self.pointer_before_reference_node.set(before_node);

                return Ok(Some(preceding_node));
            }
        }

        Ok(None)
    }

    // https://dom.spec.whatwg.org/#dom-nodeiterator-detach
    fn Detach(&self) {
        // This method intentionally left blank.
    }
}


impl NodeIterator {
    // https://dom.spec.whatwg.org/#concept-node-filter
    fn accept_node(&self, node: &Node) -> Fallible<u16> {
        // Step 1.
        let n = node.NodeType() - 1;
        // Step 2.
        if (self.what_to_show & (1 << n)) == 0 {
            return Ok(NodeFilterConstants::FILTER_SKIP)
        }
        // Step 3-5.
        match self.filter {
            Filter::None => Ok(NodeFilterConstants::FILTER_ACCEPT),
            Filter::Native(f) => Ok((f)(node))
        }
    }
}



pub enum Filter {
    None,
    Native(fn (node: &Node) -> u16)
}
