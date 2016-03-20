/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use canvas_traits::{CanvasMsg};
use dom::attr::Attr;
use dom::attr::AttrValue;
use dom::bindings::codegen::Bindings::HTMLCanvasElementBinding;
use dom::bindings::codegen::Bindings::HTMLCanvasElementBinding::HTMLCanvasElementMethods;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::{LayoutJS, Root};
use dom::bindings::reflector::Reflectable;
use dom::document::Document;
use dom::element::{AttributeMutation, Element, RawLayoutElementHelpers};
use dom::htmlelement::HTMLElement;
use dom::node::{Node};
use dom::virtualmethods::VirtualMethods;
use euclid::size::Size2D;
use ipc_channel::ipc::{IpcSender};
use string_cache::Atom;
use util::str::DOMString;

const DEFAULT_WIDTH: u32 = 300;
const DEFAULT_HEIGHT: u32 = 150;


#[dom_struct]
pub struct HTMLCanvasElement {
    htmlelement: HTMLElement
}

impl HTMLCanvasElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLCanvasElement {
        HTMLCanvasElement {
            htmlelement: HTMLElement::new_inherited(localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLCanvasElement> {
        let element = HTMLCanvasElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, HTMLCanvasElementBinding::Wrap)
    }

    pub fn get_size(&self) -> Size2D<i32> {
        Size2D::new(self.Width() as i32, self.Height() as i32)
    }

}

pub struct HTMLCanvasData {
    pub renderer_id: Option<usize>,
    pub ipc_renderer: Option<IpcSender<CanvasMsg>>,
    pub width: u32,
    pub height: u32,
}

pub trait LayoutHTMLCanvasElementHelpers {
    fn data(&self) -> HTMLCanvasData;
}

impl LayoutHTMLCanvasElementHelpers for LayoutJS<HTMLCanvasElement> {
    #[allow(unsafe_code)]
    fn data(&self) -> HTMLCanvasData {
        unsafe {
            let canvas = &*self.unsafe_get();
            let width_attr = canvas.upcast::<Element>().get_attr_for_layout(&ns!(), &atom!("width"));
            let height_attr = canvas.upcast::<Element>().get_attr_for_layout(&ns!(), &atom!("height"));
            HTMLCanvasData {
                renderer_id: None,
                ipc_renderer: None,
                width: width_attr.map_or(DEFAULT_WIDTH, |val| val.as_uint()),
                height: height_attr.map_or(DEFAULT_HEIGHT, |val| val.as_uint()),
            }
        }
    }
}


impl HTMLCanvasElement {

    pub fn is_valid(&self) -> bool {
        self.Height() != 0 && self.Width() != 0
    }

}

impl HTMLCanvasElementMethods for HTMLCanvasElement {
    // https://html.spec.whatwg.org/multipage/#dom-canvas-width
    make_uint_getter!(Width, "width", DEFAULT_WIDTH);

    // https://html.spec.whatwg.org/multipage/#dom-canvas-width
    make_uint_setter!(SetWidth, "width", DEFAULT_WIDTH);

    // https://html.spec.whatwg.org/multipage/#dom-canvas-height
    make_uint_getter!(Height, "height", DEFAULT_HEIGHT);

    // https://html.spec.whatwg.org/multipage/#dom-canvas-height
    make_uint_setter!(SetHeight, "height", DEFAULT_HEIGHT);
}

impl VirtualMethods for HTMLCanvasElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
    }

    fn parse_plain_attribute(&self, name: &Atom, value: DOMString) -> AttrValue {
        match name {
            &atom!("width") => AttrValue::from_u32(value, DEFAULT_WIDTH),
            &atom!("height") => AttrValue::from_u32(value, DEFAULT_HEIGHT),
            _ => self.super_type().unwrap().parse_plain_attribute(name, value),
        }
    }
}

pub mod utils {
    use dom::window::Window;
    use ipc_channel::ipc;
    use net_traits::image_cache_thread::{ImageCacheChan, ImageResponse};
    use url::Url;

    pub fn request_image_from_cache(window: &Window, url: Url) -> ImageResponse {
        let image_cache = window.image_cache_thread();
        let (response_chan, response_port) = ipc::channel().unwrap();
        image_cache.request_image(url, ImageCacheChan(response_chan), None);
        let result = response_port.recv().unwrap();
        result.image_response
    }
}
