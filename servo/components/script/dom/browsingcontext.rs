/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::conversions::{ToJSValConvertible};
use dom::bindings::js::{JS, Root, RootedReference};
use dom::bindings::reflector::{Reflectable, Reflector};
use dom::document::Document;
use dom::element::Element;
use dom::window::Window;
use js::jsapi::{JSObject};

#[dom_struct]
pub struct BrowsingContext {
    reflector: Reflector,
    history: DOMRefCell<Vec<SessionHistoryEntry>>,
    active_index: usize,
    frame_element: Option<JS<Element>>,
}

impl BrowsingContext {
    pub fn new_inherited(frame_element: Option<&Element>) -> BrowsingContext {
        BrowsingContext {
            reflector: Reflector::new(),
            history: DOMRefCell::new(vec![]),
            active_index: 0,
            frame_element: frame_element.map(JS::from_ref),
        }
    }

    #[allow(unsafe_code)]
    pub fn new(_window: &Window, frame_element: Option<&Element>) -> Root<BrowsingContext> {
        unsafe {
            let object = box BrowsingContext::new_inherited(frame_element);
            let raw = Box::into_raw(object);
            Root::from_ref(&*raw)
        }
    }

    pub fn init(&self, document: &Document) {
        assert!(self.history.borrow().is_empty());
        assert_eq!(self.active_index, 0);
        self.history.borrow_mut().push(SessionHistoryEntry::new(document));
    }

    pub fn active_document(&self) -> Root<Document> {
        Root::from_ref(&*self.history.borrow()[self.active_index].document)
    }

    pub fn active_window(&self) -> Root<Window> {
        Root::from_ref(self.active_document().window())
    }

    pub fn frame_element(&self) -> Option<&Element> {
        self.frame_element.r()
    }

    pub fn window_proxy(&self) -> *mut JSObject {
        let window_proxy = self.reflector.get_jsobject();
        assert!(!window_proxy.get().is_null());
        window_proxy.get()
    }
}

// This isn't a DOM struct, just a convenience struct
// without a reflector, so we don't mark this as #[dom_struct]
#[must_root]
#[privatize]
#[derive(JSTraceable, HeapSizeOf)]
pub struct SessionHistoryEntry {
    document: JS<Document>,
    children: Vec<JS<BrowsingContext>>,
}

impl SessionHistoryEntry {
    fn new(document: &Document) -> SessionHistoryEntry {
        SessionHistoryEntry {
            document: JS::from_ref(document),
            children: vec![],
        }
    }
}

