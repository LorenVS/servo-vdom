/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Castable alternative using built in type_ids

use dom::types::*;
use dom::bindings::inheritance::*;

/// An alternative to the Castable trait, which does not depend on
/// reflectors.
pub trait Typed {
    /// Retrieves the instance type of Castable instance.
    fn get_type(&self) -> TopTypeId;

    /// Determines whether another top type is a subtype of this interface.
    fn is_subtype(ty: TopTypeId) -> bool;
}

#[macro_export]
macro_rules! make_typed(
    ($ty:ident, $upto:ty, $pattern:pat) => (
        impl Typed for $ty {
            fn get_type(&self) -> TopTypeId {
                self.upcast::<$upto>().get_type()
            }

            fn is_subtype(ty: TopTypeId) -> bool {
                match ty {
                    $pattern => true,
                    _ => false
                }
            }
        }
    );
);

// DOMPointReadOnly Subtypes

make_typed!(DOMPoint, DOMPointReadOnly,
	TopTypeId::DOMPointReadOnly(DOMPointReadOnlyTypeId::DOMPoint));

// DOMRectReadOnly Subtypes

make_typed!(DOMRect, DOMRectReadOnly,
	TopTypeId::DOMRectReadOnly(DOMRectReadOnlyTypeId::DOMRect));

// Event Subtypes

make_typed!(CloseEvent, Event,
	TopTypeId::Event(EventTypeId::CloseEvent));

make_typed!(CustomEvent, Event,
	TopTypeId::Event(EventTypeId::CustomEvent));

make_typed!(ErrorEvent, Event,
	TopTypeId::Event(EventTypeId::ErrorEvent));

make_typed!(FocusEvent, Event,
	TopTypeId::Event(EventTypeId::UIEvent(UIEventTypeId::FocusEvent)));

make_typed!(KeyboardEvent, Event,
	TopTypeId::Event(EventTypeId::UIEvent(UIEventTypeId::KeyboardEvent)));

make_typed!(MessageEvent, Event,
	TopTypeId::Event(EventTypeId::MessageEvent));

make_typed!(MouseEvent, Event,
	TopTypeId::Event(EventTypeId::UIEvent(UIEventTypeId::MouseEvent)));

make_typed!(ProgressEvent, Event,
	TopTypeId::Event(EventTypeId::ProgressEvent));

make_typed!(TouchEvent, Event,
	TopTypeId::Event(EventTypeId::UIEvent(UIEventTypeId::TouchEvent)));

make_typed!(UIEvent, Event,
	TopTypeId::Event(EventTypeId::UIEvent(_)));


// Event Target Subtypes

make_typed!(CharacterData, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::CharacterData(_))));

make_typed!(Comment, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::CharacterData(CharacterDataTypeId::Comment))));

make_typed!(Document, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Document)));

make_typed!(DocumentFragment, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::DocumentFragment)));

make_typed!(DocumentType, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::DocumentType)));

make_typed!(Element, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(_))));

make_typed!(EventSource, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::EventSource));

make_typed!(HTMLAnchorElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAnchorElement)))));

make_typed!(HTMLAppletElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAppletElement)))));

make_typed!(HTMLAreaElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAreaElement)))));

make_typed!(HTMLAudioElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMediaElement(HTMLMediaElementTypeId::HTMLAudioElement))))));

make_typed!(HTMLBaseElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBaseElement)))));

make_typed!(HTMLBodyElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBodyElement)))));

make_typed!(HTMLBRElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLBRElement)))));

make_typed!(HTMLButtonElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)))));

make_typed!(HTMLCanvasElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLCanvasElement)))));

make_typed!(HTMLDataElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDataElement)))));

make_typed!(HTMLDataListElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDataListElement)))));

make_typed!(HTMLDetailsElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDetailsElement)))));

make_typed!(HTMLDialogElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDialogElement)))));

make_typed!(HTMLDirectoryElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDirectoryElement)))));

make_typed!(HTMLDivElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDivElement)))));

make_typed!(HTMLDListElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLDListElement)))));

make_typed!(HTMLElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLElement)))));

make_typed!(HTMLEmbedElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLEmbedElement)))));

make_typed!(HTMLFieldSetElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFieldSetElement)))));

make_typed!(HTMLFontElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFontElement)))));

make_typed!(HTMLFormElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFormElement)))));

make_typed!(HTMLFrameElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameElement)))));

make_typed!(HTMLFrameSetElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameSetElement)))));

make_typed!(HTMLHeadElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadElement)))));

make_typed!(HTMLHeadingElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHeadingElement)))));

make_typed!(HTMLHRElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHRElement)))));

make_typed!(HTMLHtmlElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLHtmlElement)))));

make_typed!(HTMLImageElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLImageElement)))));

make_typed!(HTMLInputElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)))));

make_typed!(HTMLLabelElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLabelElement)))));

make_typed!(HTMLLegendElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLegendElement)))));

make_typed!(HTMLLIElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLIElement)))));

make_typed!(HTMLLinkElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLinkElement)))));

make_typed!(HTMLMapElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMapElement)))));

make_typed!(HTMLMediaElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMediaElement(_))))));

make_typed!(HTMLMetaElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMetaElement)))));

make_typed!(HTMLMeterElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMeterElement)))));

make_typed!(HTMLModElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLModElement)))));

make_typed!(HTMLObjectElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)))));

make_typed!(HTMLOListElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOListElement)))));

make_typed!(HTMLOptGroupElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptGroupElement)))));

make_typed!(HTMLOptionElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOptionElement)))));

make_typed!(HTMLOutputElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLOutputElement)))));

make_typed!(HTMLParagraphElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLParagraphElement)))));

make_typed!(HTMLParamElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLParamElement)))));

make_typed!(HTMLPreElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLPreElement)))));

make_typed!(HTMLProgressElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLProgressElement)))));

make_typed!(HTMLQuoteElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLQuoteElement)))));

make_typed!(HTMLSelectElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)))));

make_typed!(HTMLSourceElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSourceElement)))));

make_typed!(HTMLSpanElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSpanElement)))));

make_typed!(HTMLStyleElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLStyleElement)))));

make_typed!(HTMLTableCaptionElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCaptionElement)))));

make_typed!(HTMLTableCellElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCellElement(_))))));

make_typed!(HTMLTableColElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableColElement)))));

make_typed!(HTMLTableDataCellElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCellElement(HTMLTableCellElementTypeId::HTMLTableDataCellElement))))));

make_typed!(HTMLTableElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableElement)))));

make_typed!(HTMLTableHeaderCellElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableCellElement(HTMLTableCellElementTypeId::HTMLTableHeaderCellElement))))));

make_typed!(HTMLTableRowElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableRowElement)))));

make_typed!(HTMLTableSectionElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTableSectionElement)))));

make_typed!(HTMLTemplateElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTemplateElement)))));

make_typed!(HTMLTextAreaElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)))));

make_typed!(HTMLTimeElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTimeElement)))));

make_typed!(HTMLTitleElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTitleElement)))));

make_typed!(HTMLTrackElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTrackElement)))));

make_typed!(HTMLUListElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUListElement)))));

make_typed!(HTMLVideoElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLMediaElement(HTMLMediaElementTypeId::HTMLVideoElement))))));

make_typed!(HTMLUnknownElement, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLUnknownElement)))));

make_typed!(Node, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(_)));

make_typed!(ProcessingInstruction, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::CharacterData(CharacterDataTypeId::ProcessingInstruction))));

make_typed!(Text, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Node(NodeTypeId::CharacterData(CharacterDataTypeId::Text))));

make_typed!(Window, EventTarget,
	TopTypeId::EventTarget(EventTargetTypeId::Window));

// HTML Collection Subtypes

make_typed!(HTMLFormControlsCollection, HTMLCollection,
	TopTypeId::HTMLCollection(HTMLCollectionTypeId::HTMLFormControlsCollection));

// NodeList Subtypes

make_typed!(RadioNodeList, NodeList,
	TopTypeId::NodeList(NodeListTypeId::RadioNodeList));

