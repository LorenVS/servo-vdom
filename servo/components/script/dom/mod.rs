/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The implementation of the DOM.
//!
//! Memory management
//! =================
//!
//! Reflectors of DOM objects, and thus the DOM objects themselves, are managed
//! by the SpiderMonkey Garbage Collector. Thus, keeping alive a DOM object
//! is done through its reflector.
//!
//! For more information, see:
//!
//! * rooting pointers on the stack:
//!   the [`Root`](bindings/js/struct.Root.html) smart pointer;
//! * tracing pointers in member fields: the [`JS`](bindings/js/struct.JS.html),
//!   [`MutNullableJS`](bindings/js/struct.MutNullableJS.html) and
//!   [`MutHeap`](bindings/js/struct.MutHeap.html) smart pointers and
//!   [the tracing implementation](bindings/trace/index.html);
//! * rooting pointers from across thread boundaries or in channels: the
//!   [`Trusted`](bindings/refcounted/struct.Trusted.html) smart pointer;
//! * extracting pointers to DOM objects from their reflectors: the
//!   [`Unrooted`](bindings/js/struct.Unrooted.html) smart pointer.
//!
//! Inheritance
//! ===========
//!
//! Rust does not support struct inheritance, as would be used for the
//! object-oriented DOM APIs. To work around this issue, Servo stores an
//! instance of the superclass in the first field of its subclasses. (Note that
//! it is stored by value, rather than in a smart pointer such as `JS<T>`.)
//!
//! This implies that a pointer to an object can safely be cast to a pointer
//! to all its classes.
//!
//! This invariant is enforced by the lint in
//! `plugins::lints::inheritance_integrity`.
//!
//! Interfaces which either derive from or are derived by other interfaces
//! implement the `Castable` trait, which provides three methods `is::<T>()`,
//! `downcast::<T>()` and `upcast::<T>()` to cast across the type hierarchy
//! and check whether a given instance is of a given type.
//!
//! ```ignore
//! use dom::bindings::inheritance::Castable;
//! use dom::element::Element;
//! use dom::htmlelement::HTMLElement;
//! use dom::htmlinputelement::HTMLInputElement;
//!
//! if let Some(elem) = node.downcast::<Element> {
//!     if elem.is::<HTMLInputElement>() {
//!         return elem.upcast::<HTMLElement>();
//!     }
//! }
//! ```
//!
//! Furthermore, when discriminating a given instance against multiple
//! interface types, code generation provides a convenient TypeId enum
//! which can be used to write `match` expressions instead of multiple
//! calls to `Castable::is::<T>`. The `type_id()` method of an instance is
//! provided by the farthest interface it derives from, e.g. `EventTarget`
//! for `HTMLMediaElement`. For convenience, that method is also provided
//! on the `Node` interface to avoid unnecessary upcasts to `EventTarget`.
//!
//! ```ignore
//! use dom::bindings::inheritance::{EventTargetTypeId, NodeTypeId};
//!
//! match *node.type_id() {
//!     EventTargetTypeId::Node(NodeTypeId::CharacterData(_)) => ...,
//!     EventTargetTypeId::Node(NodeTypeId::Element(_)) => ...,
//!     ...,
//! }
//! ```
//!
//! Construction
//! ============
//!
//! DOM objects of type `T` in Servo have two constructors:
//!
//! * a `T::new_inherited` static method that returns a plain `T`, and
//! * a `T::new` static method that returns `Root<T>`.
//!
//! (The result of either method can be wrapped in `Result`, if that is
//! appropriate for the type in question.)
//!
//! The latter calls the former and boxes the result. The former should
//! only be called by the latter, and by subclasses' `new_inherited` methods.
//!
//! Mutability and aliasing
//! =======================
//!
//! Reflectors are JavaScript objects, and as such can be freely aliased. As
//! Rust does not allow mutable aliasing, mutable borrows of DOM objects are
//! not allowed. In particular, any mutable fields use `Cell` or `DOMRefCell`
//! to manage their mutability.
//!
//! Accessing fields of a DOM object
//! ================================
//!
//! All fields of DOM objects are private; accessing them from outside their
//! module is done through explicit getter or setter methods.
//!
//! Inheritance and casting
//! =======================
//!
//! For all DOM interfaces `Foo` in an inheritance chain, a
//! `dom::bindings::inheritance::FooCast` provides methods to cast
//! to other types in the inheritance chain. For example:
//!
//! ```ignore
//! # use script::dom::bindings::inheritance::{Castable};
//! # use script::dom::element::Element;
//! # use script::dom::node::Node;
//! # use script::dom::htmlelement::HTMLElement;
//! fn f(element: &Element) {
//!     let base = element.upcast::<Node>();
//!     let derived = element.downcast::<HTMLElement>();
//! }
//! ```
//!
//! Accessing DOM objects from layout
//! =================================
//!
//! Layout code can access the DOM through the
//! [`LayoutJS`](bindings/js/struct.LayoutJS.html) smart pointer. This does not
//! keep the DOM object alive; we ensure that no DOM code (Garbage Collection
//! in particular) runs while the layout thread is accessing the DOM.
//!
//! Methods accessible to layout are implemented on `LayoutJS<Foo>` using
//! `LayoutFooHelpers` traits.

#[macro_use]
pub mod macros;

pub mod activation;
pub mod attr;
pub mod create;
#[allow(unsafe_code)]
pub mod bindings;
pub mod browsingcontext;
pub mod characterdata;
pub mod comment;
pub mod cssstyledeclaration;
pub mod document;
pub mod documentfragment;
pub mod documenttype;
pub mod domexception;
pub mod domimplementation;
pub mod dompoint;
pub mod dompointreadonly;
pub mod domquad;
pub mod domrect;
pub mod domrectlist;
pub mod domrectreadonly;
pub mod domtokenlist;
pub mod element;
pub mod event;
pub mod eventsource;
pub mod eventtarget;
pub mod focusevent;
pub mod formdata;
pub mod htmlanchorelement;
pub mod htmlappletelement;
pub mod htmlareaelement;
pub mod htmlaudioelement;
pub mod htmlbaseelement;
pub mod htmlbodyelement;
pub mod htmlbrelement;
pub mod htmlbuttonelement;
pub mod htmlcanvaselement;
pub mod htmlcollection;
pub mod htmldataelement;
pub mod htmldatalistelement;
pub mod htmldetailselement;
pub mod htmldialogelement;
pub mod htmldirectoryelement;
pub mod htmldivelement;
pub mod htmldlistelement;
pub mod htmlelement;
pub mod htmlembedelement;
pub mod htmlfieldsetelement;
pub mod htmlfontelement;
pub mod htmlformcontrolscollection;
pub mod htmlformelement;
pub mod htmlframeelement;
pub mod htmlframesetelement;
pub mod htmlheadelement;
pub mod htmlheadingelement;
pub mod htmlhrelement;
pub mod htmlhtmlelement;
pub mod htmlimageelement;
pub mod htmlinputelement;
pub mod htmllabelelement;
pub mod htmllegendelement;
pub mod htmllielement;
pub mod htmllinkelement;
pub mod htmlmapelement;
pub mod htmlmediaelement;
pub mod htmlmetaelement;
pub mod htmlmeterelement;
pub mod htmlmodelement;
pub mod htmlobjectelement;
pub mod htmlolistelement;
pub mod htmloptgroupelement;
pub mod htmloptionelement;
pub mod htmloutputelement;
pub mod htmlparagraphelement;
pub mod htmlparamelement;
pub mod htmlpreelement;
pub mod htmlprogresselement;
pub mod htmlquoteelement;
pub mod htmlselectelement;
pub mod htmlsourceelement;
pub mod htmlspanelement;
pub mod htmlstyleelement;
pub mod htmltablecaptionelement;
pub mod htmltablecellelement;
pub mod htmltablecolelement;
pub mod htmltabledatacellelement;
pub mod htmltableelement;
pub mod htmltableheadercellelement;
pub mod htmltablerowelement;
pub mod htmltablesectionelement;
pub mod htmltemplateelement;
pub mod htmltextareaelement;
pub mod htmltimeelement;
pub mod htmltitleelement;
pub mod htmltrackelement;
pub mod htmlulistelement;
pub mod htmlunknownelement;
pub mod htmlvideoelement;
pub mod imagedata;
pub mod keyboardevent;
pub mod mouseevent;
pub mod namednodemap;
pub mod node;
pub mod nodeiterator;
pub mod nodelist;
pub mod processinginstruction;
pub mod radionodelist;
pub mod screen;
pub mod text;
pub mod touch;
pub mod touchevent;
pub mod touchlist;
pub mod uievent;
pub mod values;
pub mod virtualmethods;
pub mod window;

pub mod types {
	pub use dom::attr::Attr;
	pub use dom::cssstyledeclaration::CSSStyleDeclaration;
	pub use dom::characterdata::CharacterData;
	pub use dom::comment::Comment;
	pub use dom::domexception::DOMException;
	pub use dom::domimplementation::DOMImplementation;
	pub use dom::dompoint::DOMPoint;
	pub use dom::dompointreadonly::DOMPointReadOnly;
	pub use dom::domquad::DOMQuad;
	pub use dom::domrect::DOMRect;
	pub use dom::domrectlist::DOMRectList;
	pub use dom::domrectreadonly::DOMRectReadOnly;
	pub use dom::domtokenlist::DOMTokenList;
	pub use dom::document::Document;
	pub use dom::documentfragment::DocumentFragment;
	pub use dom::documenttype::DocumentType;
	pub use dom::element::Element;
	pub use dom::event::Event;
	pub use dom::eventsource::EventSource;
	pub use dom::eventtarget::EventTarget;
	pub use dom::focusevent::FocusEvent;
	pub use dom::formdata::FormData;
	pub use dom::htmlanchorelement::HTMLAnchorElement;
	pub use dom::htmlappletelement::HTMLAppletElement;
	pub use dom::htmlareaelement::HTMLAreaElement;
	pub use dom::htmlaudioelement::HTMLAudioElement;
	pub use dom::htmlbrelement::HTMLBRElement;
	pub use dom::htmlbaseelement::HTMLBaseElement;
	pub use dom::htmlbodyelement::HTMLBodyElement;
	pub use dom::htmlbuttonelement::HTMLButtonElement;
	pub use dom::htmlcanvaselement::HTMLCanvasElement;
	pub use dom::htmlcollection::HTMLCollection;
	pub use dom::htmldlistelement::HTMLDListElement;
	pub use dom::htmldataelement::HTMLDataElement;
	pub use dom::htmldatalistelement::HTMLDataListElement;
	pub use dom::htmldetailselement::HTMLDetailsElement;
	pub use dom::htmldialogelement::HTMLDialogElement;
	pub use dom::htmldirectoryelement::HTMLDirectoryElement;
	pub use dom::htmldivelement::HTMLDivElement;
	pub use dom::htmlelement::HTMLElement;
	pub use dom::htmlembedelement::HTMLEmbedElement;
	pub use dom::htmlfieldsetelement::HTMLFieldSetElement;
	pub use dom::htmlfontelement::HTMLFontElement;
	pub use dom::htmlformcontrolscollection::HTMLFormControlsCollection;
	pub use dom::htmlformelement::HTMLFormElement;
	pub use dom::htmlframeelement::HTMLFrameElement;
	pub use dom::htmlframesetelement::HTMLFrameSetElement;
	pub use dom::htmlhrelement::HTMLHRElement;
	pub use dom::htmlheadelement::HTMLHeadElement;
	pub use dom::htmlheadingelement::HTMLHeadingElement;
	pub use dom::htmlhtmlelement::HTMLHtmlElement;
	pub use dom::htmlimageelement::HTMLImageElement;
	pub use dom::htmlinputelement::HTMLInputElement;
	pub use dom::htmllielement::HTMLLIElement;
	pub use dom::htmllabelelement::HTMLLabelElement;
	pub use dom::htmllegendelement::HTMLLegendElement;
	pub use dom::htmllinkelement::HTMLLinkElement;
	pub use dom::htmlmapelement::HTMLMapElement;
	pub use dom::htmlmediaelement::HTMLMediaElement;
	pub use dom::htmlmetaelement::HTMLMetaElement;
	pub use dom::htmlmeterelement::HTMLMeterElement;
	pub use dom::htmlmodelement::HTMLModElement;
	pub use dom::htmlolistelement::HTMLOListElement;
	pub use dom::htmlobjectelement::HTMLObjectElement;
	pub use dom::htmloptgroupelement::HTMLOptGroupElement;
	pub use dom::htmloptionelement::HTMLOptionElement;
	pub use dom::htmloutputelement::HTMLOutputElement;
	pub use dom::htmlparagraphelement::HTMLParagraphElement;
	pub use dom::htmlparamelement::HTMLParamElement;
	pub use dom::htmlpreelement::HTMLPreElement;
	pub use dom::htmlprogresselement::HTMLProgressElement;
	pub use dom::htmlquoteelement::HTMLQuoteElement;
	pub use dom::htmlselectelement::HTMLSelectElement;
	pub use dom::htmlsourceelement::HTMLSourceElement;
	pub use dom::htmlspanelement::HTMLSpanElement;
	pub use dom::htmlstyleelement::HTMLStyleElement;
	pub use dom::htmltablecaptionelement::HTMLTableCaptionElement;
	pub use dom::htmltablecellelement::HTMLTableCellElement;
	pub use dom::htmltablecolelement::HTMLTableColElement;
	pub use dom::htmltabledatacellelement::HTMLTableDataCellElement;
	pub use dom::htmltableelement::HTMLTableElement;
	pub use dom::htmltableheadercellelement::HTMLTableHeaderCellElement;
	pub use dom::htmltablerowelement::HTMLTableRowElement;
	pub use dom::htmltablesectionelement::HTMLTableSectionElement;
	pub use dom::htmltemplateelement::HTMLTemplateElement;
	pub use dom::htmltextareaelement::HTMLTextAreaElement;
	pub use dom::htmltimeelement::HTMLTimeElement;
	pub use dom::htmltitleelement::HTMLTitleElement;
	pub use dom::htmltrackelement::HTMLTrackElement;
	pub use dom::htmlulistelement::HTMLUListElement;
	pub use dom::htmlunknownelement::HTMLUnknownElement;
	pub use dom::htmlvideoelement::HTMLVideoElement;
	pub use dom::imagedata::ImageData;
	pub use dom::keyboardevent::KeyboardEvent;
	pub use dom::mouseevent::MouseEvent;
	pub use dom::namednodemap::NamedNodeMap;
	pub use dom::node::Node;
	pub use dom::nodeiterator::NodeIterator;
	pub use dom::nodelist::NodeList;
	pub use dom::processinginstruction::ProcessingInstruction;
	pub use dom::radionodelist::RadioNodeList;
	pub use dom::screen::Screen;
	pub use dom::text::Text;
	pub use dom::touch::Touch;
	pub use dom::touchevent::TouchEvent;
	pub use dom::touchlist::TouchList;
	pub use dom::uievent::UIEvent;
	pub use dom::window::Window;
}