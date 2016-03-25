/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The `Castable` trait.

use dom::bindings::conversions::{DerivedFrom};
use dom::bindings::typed::Typed;
use dom::types::*;
use std::mem;

/// A trait to hold the cast functions of IDL interfaces that either derive
/// or are derived from other interfaces.
pub trait Castable: Typed + Sized {
    /// Check whether a DOM object implements one of its deriving interfaces.
    fn is<T>(&self) -> bool
        where T: DerivedFrom<Self> + Typed
    {
        let ty = self.get_type();
        T::is_subtype(&ty)
    }

    /// Cast a DOM object upwards to one of the interfaces it derives from.
    fn upcast<T>(&self) -> &T
        where T: Castable,
              Self: DerivedFrom<T>
    {
        unsafe { mem::transmute(self) }
    }

    /// Cast a DOM object downwards to one of the interfaces it might implement.
    fn downcast<T>(&self) -> Option<&T>
        where T: DerivedFrom<Self>
    {
        if self.is::<T>() {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }
}

/// Define the type hierarchy!
/// 
/// #[derive(Clone, Copy, Debug)]
pub enum TopTypeId {
    /// ID used by abstract interfaces.
    Abstract,
    /// ID used by interfaces that are not castable.
    Alone,
    /// ID used by interfaces that derive from DOMPointReadOnly.
    DOMPointReadOnly(DOMPointReadOnlyTypeId),
    /// ID used by interfaces that derive from DOMRectReadOnly.
    DOMRectReadOnly(DOMRectReadOnlyTypeId),
    /// ID used by interfaces that derive from Event.
    Event(EventTypeId),
    /// ID used by interfaces that derive from EventTarget.
    EventTarget(EventTargetTypeId),
    /// ID used by interfaces that derive from HTMLCollection.
    HTMLCollection(HTMLCollectionTypeId),
    /// ID used by interfaces that derive from NodeList.
    NodeList(NodeListTypeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeTypeId {
    CharacterData(CharacterDataTypeId),
    Document,
    DocumentFragment,
    DocumentType,
    Element(ElementTypeId)
}

#[derive(Clone, Copy, Debug)]
pub enum EventTargetTypeId {
    EventSource,
    Node(NodeTypeId),
    Window
}

impl EventTarget {
    pub fn type_id(&self) -> EventTargetTypeId {
        if let TopTypeId::EventTarget(type_id) = self.get_type() {
            type_id
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTMLTableCellElementTypeId {
    HTMLTableDataCellElement,
    HTMLTableHeaderCellElement
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMPointReadOnlyTypeId {
    DOMPointReadOnly,
    DOMPoint
}

impl DOMPointReadOnly {
    pub fn type_id(&self) -> DOMPointReadOnlyTypeId {
        if let TopTypeId::DOMPointReadOnly(type_id) = self.get_type() {
            type_id
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTMLCollectionTypeId {
    HTMLCollection,
    HTMLFormControlsCollection
}

impl HTMLCollection {
    pub fn type_id(&self) -> HTMLCollectionTypeId {
        if let TopTypeId::HTMLCollection(type_id) = self.get_type() {
            type_id
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTMLMediaElementTypeId {
    HTMLAudioElement,
    HTMLVideoElement
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIEventTypeId {
    UIEvent,
    FocusEvent,
    KeyboardEvent,
    MouseEvent,
    TouchEvent
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementTypeId {
    Element,
    HTMLElement(HTMLElementTypeId)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataTypeId {
    Comment,
    ProcessingInstruction,
    Text
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMRectReadOnlyTypeId {
    DOMRectReadOnly,
    DOMRect
}

impl DOMRectReadOnly {
    pub fn type_id(&self) -> DOMRectReadOnlyTypeId {
        if let TopTypeId::DOMRectReadOnly(type_id) = self.get_type() {
            type_id
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HTMLElementTypeId {
    HTMLElement,
    HTMLAnchorElement,
    HTMLAppletElement,
    HTMLAreaElement,
    HTMLBRElement,
    HTMLBaseElement,
    HTMLBodyElement,
    HTMLButtonElement,
    HTMLCanvasElement,
    HTMLDListElement,
    HTMLDataElement,
    HTMLDataListElement,
    HTMLDetailsElement,
    HTMLDialogElement,
    HTMLDirectoryElement,
    HTMLDivElement,
    HTMLEmbedElement,
    HTMLFieldSetElement,
    HTMLFontElement,
    HTMLFormElement,
    HTMLFrameElement,
    HTMLFrameSetElement,
    HTMLHRElement,
    HTMLHeadElement,
    HTMLHeadingElement,
    HTMLHtmlElement,
    HTMLImageElement,
    HTMLInputElement,
    HTMLLIElement,
    HTMLLabelElement,
    HTMLLegendElement,
    HTMLLinkElement,
    HTMLMapElement,
    HTMLMediaElement(HTMLMediaElementTypeId),
    HTMLMetaElement,
    HTMLMeterElement,
    HTMLModElement,
    HTMLOListElement,
    HTMLObjectElement,
    HTMLOptGroupElement,
    HTMLOptionElement,
    HTMLOutputElement,
    HTMLParagraphElement,
    HTMLParamElement,
    HTMLPreElement,
    HTMLProgressElement,
    HTMLQuoteElement,
    HTMLSelectElement,
    HTMLSourceElement,
    HTMLSpanElement,
    HTMLStyleElement,
    HTMLTableCaptionElement,
    HTMLTableCellElement(HTMLTableCellElementTypeId),
    HTMLTableColElement,
    HTMLTableElement,
    HTMLTableRowElement,
    HTMLTableSectionElement,
    HTMLTemplateElement,
    HTMLTextAreaElement,
    HTMLTimeElement,
    HTMLTitleElement,
    HTMLTrackElement,
    HTMLUListElement,
    HTMLUnknownElement
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeListTypeId {
    NodeList,
    RadioNodeList
}

impl NodeList {
    pub fn type_id(&self) -> NodeListTypeId {
        if let TopTypeId::NodeList(type_id) = self.get_type() {
            type_id
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventTypeId {
    Event,
    CloseEvent,
    CustomEvent,
    ErrorEvent,
    MessageEvent,
    ProgressEvent,
    UIEvent(UIEventTypeId)
}

impl Event {
    pub fn type_id(&self) -> EventTypeId {
        if let TopTypeId::Event(type_id) = self.get_type() {
            type_id
        } else {
            unreachable!();
        }
    }
}

impl Castable for CharacterData {}
impl DerivedFrom<EventTarget> for CharacterData {}
impl DerivedFrom<Node> for CharacterData {}
impl DerivedFrom<CharacterData> for CharacterData {}

impl Castable for CloseEvent {}
impl DerivedFrom<Event> for CloseEvent {}

impl Castable for Comment {}
impl DerivedFrom<EventTarget> for Comment {}
impl DerivedFrom<Node> for Comment {}
impl DerivedFrom<CharacterData> for Comment {}

impl Castable for CustomEvent {}
impl DerivedFrom<Event> for CustomEvent {}

impl Castable for DOMPoint {}
impl DerivedFrom<DOMPointReadOnly> for DOMPoint {}

impl Castable for DOMPointReadOnly {}
impl DerivedFrom<DOMPointReadOnly> for DOMPointReadOnly {}

impl Castable for DOMRect {}
impl DerivedFrom<DOMRectReadOnly> for DOMRect {}

impl Castable for DOMRectReadOnly {}
impl DerivedFrom<DOMRectReadOnly> for DOMRectReadOnly {}

impl Castable for Document {}
impl DerivedFrom<EventTarget> for Document {}
impl DerivedFrom<Node> for Document {}

impl Castable for DocumentFragment {}
impl DerivedFrom<EventTarget> for DocumentFragment {}
impl DerivedFrom<Node> for DocumentFragment {}

impl Castable for DocumentType {}
impl DerivedFrom<EventTarget> for DocumentType {}
impl DerivedFrom<Node> for DocumentType {}

impl Castable for Element {}
impl DerivedFrom<EventTarget> for Element {}
impl DerivedFrom<Node> for Element {}
impl DerivedFrom<Element> for Element {}

impl Castable for ErrorEvent {}
impl DerivedFrom<Event> for ErrorEvent {}

impl Castable for Event {}
impl DerivedFrom<Event> for Event {}

impl Castable for EventSource {}
impl DerivedFrom<EventTarget> for EventSource {}

impl Castable for EventTarget {}
impl DerivedFrom<EventTarget> for EventTarget {}

impl Castable for FocusEvent {}
impl DerivedFrom<Event> for FocusEvent {}
impl DerivedFrom<UIEvent> for FocusEvent {}

impl Castable for HTMLAnchorElement {}
impl DerivedFrom<EventTarget> for HTMLAnchorElement {}
impl DerivedFrom<Node> for HTMLAnchorElement {}
impl DerivedFrom<Element> for HTMLAnchorElement {}
impl DerivedFrom<HTMLElement> for HTMLAnchorElement {}

impl Castable for HTMLAppletElement {}
impl DerivedFrom<EventTarget> for HTMLAppletElement {}
impl DerivedFrom<Node> for HTMLAppletElement {}
impl DerivedFrom<Element> for HTMLAppletElement {}
impl DerivedFrom<HTMLElement> for HTMLAppletElement {}

impl Castable for HTMLAreaElement {}
impl DerivedFrom<EventTarget> for HTMLAreaElement {}
impl DerivedFrom<Node> for HTMLAreaElement {}
impl DerivedFrom<Element> for HTMLAreaElement {}
impl DerivedFrom<HTMLElement> for HTMLAreaElement {}

impl Castable for HTMLAudioElement {}
impl DerivedFrom<EventTarget> for HTMLAudioElement {}
impl DerivedFrom<Node> for HTMLAudioElement {}
impl DerivedFrom<Element> for HTMLAudioElement {}
impl DerivedFrom<HTMLElement> for HTMLAudioElement {}
impl DerivedFrom<HTMLMediaElement> for HTMLAudioElement {}

impl Castable for HTMLBRElement {}
impl DerivedFrom<EventTarget> for HTMLBRElement {}
impl DerivedFrom<Node> for HTMLBRElement {}
impl DerivedFrom<Element> for HTMLBRElement {}
impl DerivedFrom<HTMLElement> for HTMLBRElement {}

impl Castable for HTMLBaseElement {}
impl DerivedFrom<EventTarget> for HTMLBaseElement {}
impl DerivedFrom<Node> for HTMLBaseElement {}
impl DerivedFrom<Element> for HTMLBaseElement {}
impl DerivedFrom<HTMLElement> for HTMLBaseElement {}

impl Castable for HTMLBodyElement {}
impl DerivedFrom<EventTarget> for HTMLBodyElement {}
impl DerivedFrom<Node> for HTMLBodyElement {}
impl DerivedFrom<Element> for HTMLBodyElement {}
impl DerivedFrom<HTMLElement> for HTMLBodyElement {}

impl Castable for HTMLButtonElement {}
impl DerivedFrom<EventTarget> for HTMLButtonElement {}
impl DerivedFrom<Node> for HTMLButtonElement {}
impl DerivedFrom<Element> for HTMLButtonElement {}
impl DerivedFrom<HTMLElement> for HTMLButtonElement {}

impl Castable for HTMLCanvasElement {}
impl DerivedFrom<EventTarget> for HTMLCanvasElement {}
impl DerivedFrom<Node> for HTMLCanvasElement {}
impl DerivedFrom<Element> for HTMLCanvasElement {}
impl DerivedFrom<HTMLElement> for HTMLCanvasElement {}

impl Castable for HTMLCollection {}
impl DerivedFrom<HTMLCollection> for HTMLCollection {}

impl Castable for HTMLDListElement {}
impl DerivedFrom<EventTarget> for HTMLDListElement {}
impl DerivedFrom<Node> for HTMLDListElement {}
impl DerivedFrom<Element> for HTMLDListElement {}
impl DerivedFrom<HTMLElement> for HTMLDListElement {}

impl Castable for HTMLDataElement {}
impl DerivedFrom<EventTarget> for HTMLDataElement {}
impl DerivedFrom<Node> for HTMLDataElement {}
impl DerivedFrom<Element> for HTMLDataElement {}
impl DerivedFrom<HTMLElement> for HTMLDataElement {}

impl Castable for HTMLDataListElement {}
impl DerivedFrom<EventTarget> for HTMLDataListElement {}
impl DerivedFrom<Node> for HTMLDataListElement {}
impl DerivedFrom<Element> for HTMLDataListElement {}
impl DerivedFrom<HTMLElement> for HTMLDataListElement {}

impl Castable for HTMLDetailsElement {}
impl DerivedFrom<EventTarget> for HTMLDetailsElement {}
impl DerivedFrom<Node> for HTMLDetailsElement {}
impl DerivedFrom<Element> for HTMLDetailsElement {}
impl DerivedFrom<HTMLElement> for HTMLDetailsElement {}

impl Castable for HTMLDialogElement {}
impl DerivedFrom<EventTarget> for HTMLDialogElement {}
impl DerivedFrom<Node> for HTMLDialogElement {}
impl DerivedFrom<Element> for HTMLDialogElement {}
impl DerivedFrom<HTMLElement> for HTMLDialogElement {}

impl Castable for HTMLDirectoryElement {}
impl DerivedFrom<EventTarget> for HTMLDirectoryElement {}
impl DerivedFrom<Node> for HTMLDirectoryElement {}
impl DerivedFrom<Element> for HTMLDirectoryElement {}
impl DerivedFrom<HTMLElement> for HTMLDirectoryElement {}

impl Castable for HTMLDivElement {}
impl DerivedFrom<EventTarget> for HTMLDivElement {}
impl DerivedFrom<Node> for HTMLDivElement {}
impl DerivedFrom<Element> for HTMLDivElement {}
impl DerivedFrom<HTMLElement> for HTMLDivElement {}

impl Castable for HTMLElement {}
impl DerivedFrom<EventTarget> for HTMLElement {}
impl DerivedFrom<Node> for HTMLElement {}
impl DerivedFrom<Element> for HTMLElement {}
impl DerivedFrom<HTMLElement> for HTMLElement {}

impl Castable for HTMLEmbedElement {}
impl DerivedFrom<EventTarget> for HTMLEmbedElement {}
impl DerivedFrom<Node> for HTMLEmbedElement {}
impl DerivedFrom<Element> for HTMLEmbedElement {}
impl DerivedFrom<HTMLElement> for HTMLEmbedElement {}

impl Castable for HTMLFieldSetElement {}
impl DerivedFrom<EventTarget> for HTMLFieldSetElement {}
impl DerivedFrom<Node> for HTMLFieldSetElement {}
impl DerivedFrom<Element> for HTMLFieldSetElement {}
impl DerivedFrom<HTMLElement> for HTMLFieldSetElement {}

impl Castable for HTMLFontElement {}
impl DerivedFrom<EventTarget> for HTMLFontElement {}
impl DerivedFrom<Node> for HTMLFontElement {}
impl DerivedFrom<Element> for HTMLFontElement {}
impl DerivedFrom<HTMLElement> for HTMLFontElement {}

impl Castable for HTMLFormControlsCollection {}
impl DerivedFrom<HTMLCollection> for HTMLFormControlsCollection {}

impl Castable for HTMLFormElement {}
impl DerivedFrom<EventTarget> for HTMLFormElement {}
impl DerivedFrom<Node> for HTMLFormElement {}
impl DerivedFrom<Element> for HTMLFormElement {}
impl DerivedFrom<HTMLElement> for HTMLFormElement {}

impl Castable for HTMLFrameElement {}
impl DerivedFrom<EventTarget> for HTMLFrameElement {}
impl DerivedFrom<Node> for HTMLFrameElement {}
impl DerivedFrom<Element> for HTMLFrameElement {}
impl DerivedFrom<HTMLElement> for HTMLFrameElement {}

impl Castable for HTMLFrameSetElement {}
impl DerivedFrom<EventTarget> for HTMLFrameSetElement {}
impl DerivedFrom<Node> for HTMLFrameSetElement {}
impl DerivedFrom<Element> for HTMLFrameSetElement {}
impl DerivedFrom<HTMLElement> for HTMLFrameSetElement {}

impl Castable for HTMLHRElement {}
impl DerivedFrom<EventTarget> for HTMLHRElement {}
impl DerivedFrom<Node> for HTMLHRElement {}
impl DerivedFrom<Element> for HTMLHRElement {}
impl DerivedFrom<HTMLElement> for HTMLHRElement {}

impl Castable for HTMLHeadElement {}
impl DerivedFrom<EventTarget> for HTMLHeadElement {}
impl DerivedFrom<Node> for HTMLHeadElement {}
impl DerivedFrom<Element> for HTMLHeadElement {}
impl DerivedFrom<HTMLElement> for HTMLHeadElement {}

impl Castable for HTMLHeadingElement {}
impl DerivedFrom<EventTarget> for HTMLHeadingElement {}
impl DerivedFrom<Node> for HTMLHeadingElement {}
impl DerivedFrom<Element> for HTMLHeadingElement {}
impl DerivedFrom<HTMLElement> for HTMLHeadingElement {}

impl Castable for HTMLHtmlElement {}
impl DerivedFrom<EventTarget> for HTMLHtmlElement {}
impl DerivedFrom<Node> for HTMLHtmlElement {}
impl DerivedFrom<Element> for HTMLHtmlElement {}
impl DerivedFrom<HTMLElement> for HTMLHtmlElement {}

impl Castable for HTMLImageElement {}
impl DerivedFrom<EventTarget> for HTMLImageElement {}
impl DerivedFrom<Node> for HTMLImageElement {}
impl DerivedFrom<Element> for HTMLImageElement {}
impl DerivedFrom<HTMLElement> for HTMLImageElement {}

impl Castable for HTMLInputElement {}
impl DerivedFrom<EventTarget> for HTMLInputElement {}
impl DerivedFrom<Node> for HTMLInputElement {}
impl DerivedFrom<Element> for HTMLInputElement {}
impl DerivedFrom<HTMLElement> for HTMLInputElement {}

impl Castable for HTMLLIElement {}
impl DerivedFrom<EventTarget> for HTMLLIElement {}
impl DerivedFrom<Node> for HTMLLIElement {}
impl DerivedFrom<Element> for HTMLLIElement {}
impl DerivedFrom<HTMLElement> for HTMLLIElement {}

impl Castable for HTMLLabelElement {}
impl DerivedFrom<EventTarget> for HTMLLabelElement {}
impl DerivedFrom<Node> for HTMLLabelElement {}
impl DerivedFrom<Element> for HTMLLabelElement {}
impl DerivedFrom<HTMLElement> for HTMLLabelElement {}

impl Castable for HTMLLegendElement {}
impl DerivedFrom<EventTarget> for HTMLLegendElement {}
impl DerivedFrom<Node> for HTMLLegendElement {}
impl DerivedFrom<Element> for HTMLLegendElement {}
impl DerivedFrom<HTMLElement> for HTMLLegendElement {}

impl Castable for HTMLLinkElement {}
impl DerivedFrom<EventTarget> for HTMLLinkElement {}
impl DerivedFrom<Node> for HTMLLinkElement {}
impl DerivedFrom<Element> for HTMLLinkElement {}
impl DerivedFrom<HTMLElement> for HTMLLinkElement {}

impl Castable for HTMLMapElement {}
impl DerivedFrom<EventTarget> for HTMLMapElement {}
impl DerivedFrom<Node> for HTMLMapElement {}
impl DerivedFrom<Element> for HTMLMapElement {}
impl DerivedFrom<HTMLElement> for HTMLMapElement {}

impl Castable for HTMLMediaElement {}
impl DerivedFrom<EventTarget> for HTMLMediaElement {}
impl DerivedFrom<Node> for HTMLMediaElement {}
impl DerivedFrom<Element> for HTMLMediaElement {}
impl DerivedFrom<HTMLElement> for HTMLMediaElement {}
impl DerivedFrom<HTMLMediaElement> for HTMLMediaElement {}

impl Castable for HTMLMetaElement {}
impl DerivedFrom<EventTarget> for HTMLMetaElement {}
impl DerivedFrom<Node> for HTMLMetaElement {}
impl DerivedFrom<Element> for HTMLMetaElement {}
impl DerivedFrom<HTMLElement> for HTMLMetaElement {}

impl Castable for HTMLMeterElement {}
impl DerivedFrom<EventTarget> for HTMLMeterElement {}
impl DerivedFrom<Node> for HTMLMeterElement {}
impl DerivedFrom<Element> for HTMLMeterElement {}
impl DerivedFrom<HTMLElement> for HTMLMeterElement {}

impl Castable for HTMLModElement {}
impl DerivedFrom<EventTarget> for HTMLModElement {}
impl DerivedFrom<Node> for HTMLModElement {}
impl DerivedFrom<Element> for HTMLModElement {}
impl DerivedFrom<HTMLElement> for HTMLModElement {}

impl Castable for HTMLOListElement {}
impl DerivedFrom<EventTarget> for HTMLOListElement {}
impl DerivedFrom<Node> for HTMLOListElement {}
impl DerivedFrom<Element> for HTMLOListElement {}
impl DerivedFrom<HTMLElement> for HTMLOListElement {}

impl Castable for HTMLObjectElement {}
impl DerivedFrom<EventTarget> for HTMLObjectElement {}
impl DerivedFrom<Node> for HTMLObjectElement {}
impl DerivedFrom<Element> for HTMLObjectElement {}
impl DerivedFrom<HTMLElement> for HTMLObjectElement {}

impl Castable for HTMLOptGroupElement {}
impl DerivedFrom<EventTarget> for HTMLOptGroupElement {}
impl DerivedFrom<Node> for HTMLOptGroupElement {}
impl DerivedFrom<Element> for HTMLOptGroupElement {}
impl DerivedFrom<HTMLElement> for HTMLOptGroupElement {}

impl Castable for HTMLOptionElement {}
impl DerivedFrom<EventTarget> for HTMLOptionElement {}
impl DerivedFrom<Node> for HTMLOptionElement {}
impl DerivedFrom<Element> for HTMLOptionElement {}
impl DerivedFrom<HTMLElement> for HTMLOptionElement {}

impl Castable for HTMLOutputElement {}
impl DerivedFrom<EventTarget> for HTMLOutputElement {}
impl DerivedFrom<Node> for HTMLOutputElement {}
impl DerivedFrom<Element> for HTMLOutputElement {}
impl DerivedFrom<HTMLElement> for HTMLOutputElement {}

impl Castable for HTMLParagraphElement {}
impl DerivedFrom<EventTarget> for HTMLParagraphElement {}
impl DerivedFrom<Node> for HTMLParagraphElement {}
impl DerivedFrom<Element> for HTMLParagraphElement {}
impl DerivedFrom<HTMLElement> for HTMLParagraphElement {}

impl Castable for HTMLParamElement {}
impl DerivedFrom<EventTarget> for HTMLParamElement {}
impl DerivedFrom<Node> for HTMLParamElement {}
impl DerivedFrom<Element> for HTMLParamElement {}
impl DerivedFrom<HTMLElement> for HTMLParamElement {}

impl Castable for HTMLPreElement {}
impl DerivedFrom<EventTarget> for HTMLPreElement {}
impl DerivedFrom<Node> for HTMLPreElement {}
impl DerivedFrom<Element> for HTMLPreElement {}
impl DerivedFrom<HTMLElement> for HTMLPreElement {}

impl Castable for HTMLProgressElement {}
impl DerivedFrom<EventTarget> for HTMLProgressElement {}
impl DerivedFrom<Node> for HTMLProgressElement {}
impl DerivedFrom<Element> for HTMLProgressElement {}
impl DerivedFrom<HTMLElement> for HTMLProgressElement {}

impl Castable for HTMLQuoteElement {}
impl DerivedFrom<EventTarget> for HTMLQuoteElement {}
impl DerivedFrom<Node> for HTMLQuoteElement {}
impl DerivedFrom<Element> for HTMLQuoteElement {}
impl DerivedFrom<HTMLElement> for HTMLQuoteElement {}

impl Castable for HTMLSelectElement {}
impl DerivedFrom<EventTarget> for HTMLSelectElement {}
impl DerivedFrom<Node> for HTMLSelectElement {}
impl DerivedFrom<Element> for HTMLSelectElement {}
impl DerivedFrom<HTMLElement> for HTMLSelectElement {}

impl Castable for HTMLSourceElement {}
impl DerivedFrom<EventTarget> for HTMLSourceElement {}
impl DerivedFrom<Node> for HTMLSourceElement {}
impl DerivedFrom<Element> for HTMLSourceElement {}
impl DerivedFrom<HTMLElement> for HTMLSourceElement {}

impl Castable for HTMLSpanElement {}
impl DerivedFrom<EventTarget> for HTMLSpanElement {}
impl DerivedFrom<Node> for HTMLSpanElement {}
impl DerivedFrom<Element> for HTMLSpanElement {}
impl DerivedFrom<HTMLElement> for HTMLSpanElement {}

impl Castable for HTMLStyleElement {}
impl DerivedFrom<EventTarget> for HTMLStyleElement {}
impl DerivedFrom<Node> for HTMLStyleElement {}
impl DerivedFrom<Element> for HTMLStyleElement {}
impl DerivedFrom<HTMLElement> for HTMLStyleElement {}

impl Castable for HTMLTableCaptionElement {}
impl DerivedFrom<EventTarget> for HTMLTableCaptionElement {}
impl DerivedFrom<Node> for HTMLTableCaptionElement {}
impl DerivedFrom<Element> for HTMLTableCaptionElement {}
impl DerivedFrom<HTMLElement> for HTMLTableCaptionElement {}

impl Castable for HTMLTableCellElement {}
impl DerivedFrom<EventTarget> for HTMLTableCellElement {}
impl DerivedFrom<Node> for HTMLTableCellElement {}
impl DerivedFrom<Element> for HTMLTableCellElement {}
impl DerivedFrom<HTMLElement> for HTMLTableCellElement {}
impl DerivedFrom<HTMLTableCellElement> for HTMLTableCellElement {}

impl Castable for HTMLTableColElement {}
impl DerivedFrom<EventTarget> for HTMLTableColElement {}
impl DerivedFrom<Node> for HTMLTableColElement {}
impl DerivedFrom<Element> for HTMLTableColElement {}
impl DerivedFrom<HTMLElement> for HTMLTableColElement {}

impl Castable for HTMLTableDataCellElement {}
impl DerivedFrom<EventTarget> for HTMLTableDataCellElement {}
impl DerivedFrom<Node> for HTMLTableDataCellElement {}
impl DerivedFrom<Element> for HTMLTableDataCellElement {}
impl DerivedFrom<HTMLElement> for HTMLTableDataCellElement {}
impl DerivedFrom<HTMLTableCellElement> for HTMLTableDataCellElement {}

impl Castable for HTMLTableElement {}
impl DerivedFrom<EventTarget> for HTMLTableElement {}
impl DerivedFrom<Node> for HTMLTableElement {}
impl DerivedFrom<Element> for HTMLTableElement {}
impl DerivedFrom<HTMLElement> for HTMLTableElement {}

impl Castable for HTMLTableHeaderCellElement {}
impl DerivedFrom<EventTarget> for HTMLTableHeaderCellElement {}
impl DerivedFrom<Node> for HTMLTableHeaderCellElement {}
impl DerivedFrom<Element> for HTMLTableHeaderCellElement {}
impl DerivedFrom<HTMLElement> for HTMLTableHeaderCellElement {}
impl DerivedFrom<HTMLTableCellElement> for HTMLTableHeaderCellElement {}

impl Castable for HTMLTableRowElement {}
impl DerivedFrom<EventTarget> for HTMLTableRowElement {}
impl DerivedFrom<Node> for HTMLTableRowElement {}
impl DerivedFrom<Element> for HTMLTableRowElement {}
impl DerivedFrom<HTMLElement> for HTMLTableRowElement {}

impl Castable for HTMLTableSectionElement {}
impl DerivedFrom<EventTarget> for HTMLTableSectionElement {}
impl DerivedFrom<Node> for HTMLTableSectionElement {}
impl DerivedFrom<Element> for HTMLTableSectionElement {}
impl DerivedFrom<HTMLElement> for HTMLTableSectionElement {}

impl Castable for HTMLTemplateElement {}
impl DerivedFrom<EventTarget> for HTMLTemplateElement {}
impl DerivedFrom<Node> for HTMLTemplateElement {}
impl DerivedFrom<Element> for HTMLTemplateElement {}
impl DerivedFrom<HTMLElement> for HTMLTemplateElement {}

impl Castable for HTMLTextAreaElement {}
impl DerivedFrom<EventTarget> for HTMLTextAreaElement {}
impl DerivedFrom<Node> for HTMLTextAreaElement {}
impl DerivedFrom<Element> for HTMLTextAreaElement {}
impl DerivedFrom<HTMLElement> for HTMLTextAreaElement {}

impl Castable for HTMLTimeElement {}
impl DerivedFrom<EventTarget> for HTMLTimeElement {}
impl DerivedFrom<Node> for HTMLTimeElement {}
impl DerivedFrom<Element> for HTMLTimeElement {}
impl DerivedFrom<HTMLElement> for HTMLTimeElement {}

impl Castable for HTMLTitleElement {}
impl DerivedFrom<EventTarget> for HTMLTitleElement {}
impl DerivedFrom<Node> for HTMLTitleElement {}
impl DerivedFrom<Element> for HTMLTitleElement {}
impl DerivedFrom<HTMLElement> for HTMLTitleElement {}

impl Castable for HTMLTrackElement {}
impl DerivedFrom<EventTarget> for HTMLTrackElement {}
impl DerivedFrom<Node> for HTMLTrackElement {}
impl DerivedFrom<Element> for HTMLTrackElement {}
impl DerivedFrom<HTMLElement> for HTMLTrackElement {}

impl Castable for HTMLUListElement {}
impl DerivedFrom<EventTarget> for HTMLUListElement {}
impl DerivedFrom<Node> for HTMLUListElement {}
impl DerivedFrom<Element> for HTMLUListElement {}
impl DerivedFrom<HTMLElement> for HTMLUListElement {}

impl Castable for HTMLUnknownElement {}
impl DerivedFrom<EventTarget> for HTMLUnknownElement {}
impl DerivedFrom<Node> for HTMLUnknownElement {}
impl DerivedFrom<Element> for HTMLUnknownElement {}
impl DerivedFrom<HTMLElement> for HTMLUnknownElement {}

impl Castable for HTMLVideoElement {}
impl DerivedFrom<EventTarget> for HTMLVideoElement {}
impl DerivedFrom<Node> for HTMLVideoElement {}
impl DerivedFrom<Element> for HTMLVideoElement {}
impl DerivedFrom<HTMLElement> for HTMLVideoElement {}
impl DerivedFrom<HTMLMediaElement> for HTMLVideoElement {}

impl Castable for KeyboardEvent {}
impl DerivedFrom<Event> for KeyboardEvent {}
impl DerivedFrom<UIEvent> for KeyboardEvent {}

impl Castable for MessageEvent {}
impl DerivedFrom<Event> for MessageEvent {}

impl Castable for MouseEvent {}
impl DerivedFrom<Event> for MouseEvent {}
impl DerivedFrom<UIEvent> for MouseEvent {}

impl Castable for Node {}
impl DerivedFrom<EventTarget> for Node {}
impl DerivedFrom<Node> for Node {}

impl Castable for NodeList {}
impl DerivedFrom<NodeList> for NodeList {}

impl Castable for ProcessingInstruction {}
impl DerivedFrom<EventTarget> for ProcessingInstruction {}
impl DerivedFrom<Node> for ProcessingInstruction {}
impl DerivedFrom<CharacterData> for ProcessingInstruction {}

impl Castable for ProgressEvent {}
impl DerivedFrom<Event> for ProgressEvent {}

impl Castable for RadioNodeList {}
impl DerivedFrom<NodeList> for RadioNodeList {}

impl Castable for Text {}
impl DerivedFrom<EventTarget> for Text {}
impl DerivedFrom<Node> for Text {}
impl DerivedFrom<CharacterData> for Text {}

impl Castable for TouchEvent {}
impl DerivedFrom<Event> for TouchEvent {}
impl DerivedFrom<UIEvent> for TouchEvent {}

impl Castable for UIEvent {}
impl DerivedFrom<Event> for UIEvent {}
impl DerivedFrom<UIEvent> for UIEvent {}

impl Castable for Window {}
impl DerivedFrom<EventTarget> for Window {}