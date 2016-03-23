/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::js::Root;
use dom::bindings::inheritance::{ElementTypeId};
use dom::document::Document;
use dom::element::Element;
use dom::element::ElementCreator;
use dom::htmlanchorelement::HTMLAnchorElement;
use dom::htmlappletelement::HTMLAppletElement;
use dom::htmlareaelement::HTMLAreaElement;
use dom::htmlaudioelement::HTMLAudioElement;
use dom::htmlbaseelement::HTMLBaseElement;
use dom::htmlbodyelement::HTMLBodyElement;
use dom::htmlbrelement::HTMLBRElement;
use dom::htmlbuttonelement::HTMLButtonElement;
use dom::htmlcanvaselement::HTMLCanvasElement;
use dom::htmldataelement::HTMLDataElement;
use dom::htmldatalistelement::HTMLDataListElement;
use dom::htmldetailselement::HTMLDetailsElement;
use dom::htmldialogelement::HTMLDialogElement;
use dom::htmldirectoryelement::HTMLDirectoryElement;
use dom::htmldivelement::HTMLDivElement;
use dom::htmldlistelement::HTMLDListElement;
use dom::htmlelement::HTMLElement;
use dom::htmlembedelement::HTMLEmbedElement;
use dom::htmlfieldsetelement::HTMLFieldSetElement;
use dom::htmlfontelement::HTMLFontElement;
use dom::htmlformelement::HTMLFormElement;
use dom::htmlframeelement::HTMLFrameElement;
use dom::htmlframesetelement::HTMLFrameSetElement;
use dom::htmlheadelement::HTMLHeadElement;
use dom::htmlheadingelement::HTMLHeadingElement;
use dom::htmlheadingelement::HeadingLevel;
use dom::htmlhrelement::HTMLHRElement;
use dom::htmlhtmlelement::HTMLHtmlElement;
use dom::htmlimageelement::HTMLImageElement;
use dom::htmlinputelement::HTMLInputElement;
use dom::htmllabelelement::HTMLLabelElement;
use dom::htmllegendelement::HTMLLegendElement;
use dom::htmllielement::HTMLLIElement;
use dom::htmllinkelement::HTMLLinkElement;
use dom::htmlmapelement::HTMLMapElement;
use dom::htmlmetaelement::HTMLMetaElement;
use dom::htmlmeterelement::HTMLMeterElement;
use dom::htmlmodelement::HTMLModElement;
use dom::htmlobjectelement::HTMLObjectElement;
use dom::htmlolistelement::HTMLOListElement;
use dom::htmloptgroupelement::HTMLOptGroupElement;
use dom::htmloptionelement::HTMLOptionElement;
use dom::htmloutputelement::HTMLOutputElement;
use dom::htmlparagraphelement::HTMLParagraphElement;
use dom::htmlparamelement::HTMLParamElement;
use dom::htmlpreelement::HTMLPreElement;
use dom::htmlprogresselement::HTMLProgressElement;
use dom::htmlquoteelement::HTMLQuoteElement;
use dom::htmlselectelement::HTMLSelectElement;
use dom::htmlsourceelement::HTMLSourceElement;
use dom::htmlspanelement::HTMLSpanElement;
use dom::htmlstyleelement::HTMLStyleElement;
use dom::htmltablecaptionelement::HTMLTableCaptionElement;
use dom::htmltablecolelement::HTMLTableColElement;
use dom::htmltabledatacellelement::HTMLTableDataCellElement;
use dom::htmltableelement::HTMLTableElement;
use dom::htmltableheadercellelement::HTMLTableHeaderCellElement;
use dom::htmltablerowelement::HTMLTableRowElement;
use dom::htmltablesectionelement::HTMLTableSectionElement;
use dom::htmltemplateelement::HTMLTemplateElement;
use dom::htmltextareaelement::HTMLTextAreaElement;
use dom::htmltimeelement::HTMLTimeElement;
use dom::htmltitleelement::HTMLTitleElement;
use dom::htmltrackelement::HTMLTrackElement;
use dom::htmlulistelement::HTMLUListElement;
use dom::htmlunknownelement::HTMLUnknownElement;
use dom::htmlvideoelement::HTMLVideoElement;
use servo_vdom_client::patch::ElementName;
use string_cache::{Atom, QualName};
use util::str::DOMString;

pub fn create_element_simple(
                      id: u64,
                      name: Atom,
                      document: &Document,
                      creator: ElementCreator)
                      -> Root<Element> {

    macro_rules! make(
        ($ctor:ident) => ({
            let obj = $ctor::new(id, name, None, document);
            Root::upcast(obj)
        });
        ($ctor:ident, $($arg:expr),+) => ({
            let obj = $ctor::new(id, name, None, document, $($arg),+);
            Root::upcast(obj)
        })
    );

    // This is a big match, and the IDs for inline-interned atoms are not very structured.
    // Perhaps we should build a perfect hash from those IDs instead.
    match name {
        atom!("a")          => make!(HTMLAnchorElement),
        atom!("abbr")       => make!(HTMLElement),
        atom!("acronym")    => make!(HTMLElement),
        atom!("address")    => make!(HTMLElement),
        atom!("applet")     => make!(HTMLAppletElement),
        atom!("area")       => make!(HTMLAreaElement),
        atom!("article")    => make!(HTMLElement),
        atom!("aside")      => make!(HTMLElement),
        atom!("audio")      => make!(HTMLAudioElement),
        atom!("b")          => make!(HTMLElement),
        atom!("base")       => make!(HTMLBaseElement),
        atom!("bdi")        => make!(HTMLElement),
        atom!("bdo")        => make!(HTMLElement),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:bgsound
        atom!("bgsound")    => make!(HTMLUnknownElement),
        atom!("big")        => make!(HTMLElement),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:blink
        atom!("blink")      => make!(HTMLUnknownElement),
        // https://html.spec.whatwg.org/multipage/#the-blockquote-element
        atom!("blockquote") => make!(HTMLQuoteElement),
        atom!("body")       => make!(HTMLBodyElement),
        atom!("br")         => make!(HTMLBRElement),
        atom!("button")     => make!(HTMLButtonElement),
        atom!("canvas")     => make!(HTMLCanvasElement),
        atom!("caption")    => make!(HTMLTableCaptionElement),
        atom!("center")     => make!(HTMLElement),
        atom!("cite")       => make!(HTMLElement),
        atom!("code")       => make!(HTMLElement),
        atom!("col")        => make!(HTMLTableColElement),
        atom!("colgroup")   => make!(HTMLTableColElement),
        atom!("data")       => make!(HTMLDataElement),
        atom!("datalist")   => make!(HTMLDataListElement),
        atom!("dd")         => make!(HTMLElement),
        atom!("del")        => make!(HTMLModElement),
        atom!("details")    => make!(HTMLDetailsElement),
        atom!("dfn")        => make!(HTMLElement),
        atom!("dialog")     => make!(HTMLDialogElement),
        atom!("dir")        => make!(HTMLDirectoryElement),
        atom!("div")        => make!(HTMLDivElement),
        atom!("dl")         => make!(HTMLDListElement),
        atom!("dt")         => make!(HTMLElement),
        atom!("em")         => make!(HTMLElement),
        atom!("embed")      => make!(HTMLEmbedElement),
        atom!("fieldset")   => make!(HTMLFieldSetElement),
        atom!("figcaption") => make!(HTMLElement),
        atom!("figure")     => make!(HTMLElement),
        atom!("font")       => make!(HTMLFontElement),
        atom!("footer")     => make!(HTMLElement),
        atom!("form")       => make!(HTMLFormElement),
        atom!("frame")      => make!(HTMLFrameElement),
        atom!("frameset")   => make!(HTMLFrameSetElement),
        atom!("h1")         => make!(HTMLHeadingElement, HeadingLevel::Heading1),
        atom!("h2")         => make!(HTMLHeadingElement, HeadingLevel::Heading2),
        atom!("h3")         => make!(HTMLHeadingElement, HeadingLevel::Heading3),
        atom!("h4")         => make!(HTMLHeadingElement, HeadingLevel::Heading4),
        atom!("h5")         => make!(HTMLHeadingElement, HeadingLevel::Heading5),
        atom!("h6")         => make!(HTMLHeadingElement, HeadingLevel::Heading6),
        atom!("head")       => make!(HTMLHeadElement),
        atom!("header")     => make!(HTMLElement),
        atom!("hgroup")     => make!(HTMLElement),
        atom!("hr")         => make!(HTMLHRElement),
        atom!("html")       => make!(HTMLHtmlElement),
        atom!("i")          => make!(HTMLElement),
        atom!("img")        => make!(HTMLImageElement),
        atom!("input")      => make!(HTMLInputElement),
        atom!("ins")        => make!(HTMLModElement),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:isindex-2
        atom!("isindex")    => make!(HTMLUnknownElement),
        atom!("kbd")        => make!(HTMLElement),
        atom!("label")      => make!(HTMLLabelElement),
        atom!("legend")     => make!(HTMLLegendElement),
        atom!("li")         => make!(HTMLLIElement),
        atom!("link")       => make!(HTMLLinkElement, creator),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:listing
        atom!("listing")    => make!(HTMLPreElement),
        atom!("main")       => make!(HTMLElement),
        atom!("map")        => make!(HTMLMapElement),
        atom!("mark")       => make!(HTMLElement),
        atom!("marquee")    => make!(HTMLElement),
        atom!("meta")       => make!(HTMLMetaElement),
        atom!("meter")      => make!(HTMLMeterElement),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:multicol
        atom!("multicol")   => make!(HTMLUnknownElement),
        atom!("nav")        => make!(HTMLElement),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:nextid
        atom!("nextid")     => make!(HTMLUnknownElement),
        atom!("nobr")       => make!(HTMLElement),
        atom!("noframes")   => make!(HTMLElement),
        atom!("noscript")   => make!(HTMLElement),
        atom!("object")     => make!(HTMLObjectElement),
        atom!("ol")         => make!(HTMLOListElement),
        atom!("optgroup")   => make!(HTMLOptGroupElement),
        atom!("option")     => make!(HTMLOptionElement),
        atom!("output")     => make!(HTMLOutputElement),
        atom!("p")          => make!(HTMLParagraphElement),
        atom!("param")      => make!(HTMLParamElement),
        atom!("plaintext")  => make!(HTMLPreElement),
        atom!("pre")        => make!(HTMLPreElement),
        atom!("progress")   => make!(HTMLProgressElement),
        atom!("q")          => make!(HTMLQuoteElement),
        atom!("rp")         => make!(HTMLElement),
        atom!("rt")         => make!(HTMLElement),
        atom!("ruby")       => make!(HTMLElement),
        atom!("s")          => make!(HTMLElement),
        atom!("samp")       => make!(HTMLElement),
        atom!("section")    => make!(HTMLElement),
        atom!("select")     => make!(HTMLSelectElement),
        atom!("small")      => make!(HTMLElement),
        atom!("source")     => make!(HTMLSourceElement),
        // https://html.spec.whatwg.org/multipage/#other-elements,-attributes-and-apis:spacer
        atom!("spacer")     => make!(HTMLUnknownElement),
        atom!("span")       => make!(HTMLSpanElement),
        atom!("strike")     => make!(HTMLElement),
        atom!("strong")     => make!(HTMLElement),
        atom!("style")      => make!(HTMLStyleElement),
        atom!("sub")        => make!(HTMLElement),
        atom!("summary")    => make!(HTMLElement),
        atom!("sup")        => make!(HTMLElement),
        atom!("table")      => make!(HTMLTableElement),
        atom!("tbody")      => make!(HTMLTableSectionElement),
        atom!("td")         => make!(HTMLTableDataCellElement),
        atom!("template")   => make!(HTMLTemplateElement),
        atom!("textarea")   => make!(HTMLTextAreaElement),
        // https://html.spec.whatwg.org/multipage/#the-tfoot-element:concept-element-dom
        atom!("tfoot")      => make!(HTMLTableSectionElement),
        atom!("th")         => make!(HTMLTableHeaderCellElement),
        // https://html.spec.whatwg.org/multipage/#the-thead-element:concept-element-dom
        atom!("thead")      => make!(HTMLTableSectionElement),
        atom!("time")       => make!(HTMLTimeElement),
        atom!("title")      => make!(HTMLTitleElement),
        atom!("tr")         => make!(HTMLTableRowElement),
        atom!("tt")         => make!(HTMLElement),
        atom!("track")      => make!(HTMLTrackElement),
        atom!("u")          => make!(HTMLElement),
        atom!("ul")         => make!(HTMLUListElement),
        atom!("var")        => make!(HTMLElement),
        atom!("video")      => make!(HTMLVideoElement),
        atom!("wbr")        => make!(HTMLElement),
        atom!("xmp")        => make!(HTMLPreElement),
        _                   => make!(HTMLUnknownElement),
    }
}


pub fn create_element_named(
                      id: u64,
                      name: ElementName,
                      document: &Document,
                      creator: ElementCreator)
                      -> Root<Element> {

    macro_rules! make(
        ($ctor:ident, $atom:expr) => ({
            let obj = $ctor::new(id, $atom, None, document);
            Root::upcast(obj)
        });
        ($ctor:ident, $atom:expr, $($arg:expr),+) => ({
            let obj = $ctor::new(id, $atom, None, document, $($arg),+);
            Root::upcast(obj)
        })
    );

    // This is a big match, and the IDs for inline-interned atoms are not very structured.
    // Perhaps we should build a perfect hash from those IDs instead.
    match name {
        ElementName::A          => make!(HTMLAnchorElement, atom!("a")),
        ElementName::Acronym    => make!(HTMLElement, atom!("acronym")),
        ElementName::Address    => make!(HTMLElement, atom!("address")),
        ElementName::Applet     => make!(HTMLAppletElement, atom!("applet")),
        ElementName::Area       => make!(HTMLAreaElement, atom!("area")),
        ElementName::Article    => make!(HTMLElement, atom!("article")),
        ElementName::Aside      => make!(HTMLElement, atom!("aside")),
        ElementName::Audio      => make!(HTMLAudioElement, atom!("audio")),
        ElementName::B          => make!(HTMLElement, atom!("b")),
        ElementName::Base       => make!(HTMLBaseElement, atom!("base")),
        ElementName::Bdi        => make!(HTMLElement, atom!("bdi")),
        ElementName::Bdo        => make!(HTMLElement, atom!("bdo")),
        ElementName::Bgsound    => make!(HTMLUnknownElement, atom!("bgsound")),
        ElementName::Big        => make!(HTMLElement, atom!("big")),
        ElementName::Blink      => make!(HTMLUnknownElement, atom!("blink")),
        ElementName::Blockquote => make!(HTMLQuoteElement, atom!("blockquote")),
        ElementName::Body       => make!(HTMLBodyElement, atom!("body")),
        ElementName::Br         => make!(HTMLBRElement, atom!("br")),
        ElementName::Button     => make!(HTMLButtonElement, atom!("button")),
        ElementName::Canvas     => make!(HTMLCanvasElement, atom!("canvas")),
        ElementName::Caption    => make!(HTMLTableCaptionElement, atom!("caption")),
        ElementName::Center     => make!(HTMLElement, atom!("center")),
        ElementName::Cite       => make!(HTMLElement, atom!("cite")),
        ElementName::Code       => make!(HTMLElement, atom!("code")),
        ElementName::Col        => make!(HTMLTableColElement, atom!("col")),
        ElementName::Colgroup   => make!(HTMLTableColElement, atom!("colgroup")),
        ElementName::Data       => make!(HTMLDataElement, atom!("data")),
        ElementName::Datalist   => make!(HTMLDataListElement, atom!("datalist")),
        ElementName::Dd         => make!(HTMLElement, atom!("dd")),
        ElementName::Del        => make!(HTMLModElement, atom!("del")),
        ElementName::Details    => make!(HTMLDetailsElement, atom!("details")),
        ElementName::Dfn        => make!(HTMLElement, atom!("dfn")),
        ElementName::Dialog     => make!(HTMLDialogElement, atom!("dialog")),
        ElementName::Dir        => make!(HTMLDirectoryElement, atom!("dir")),
        ElementName::Div        => make!(HTMLDivElement, atom!("div")),
        ElementName::Dl         => make!(HTMLDListElement, atom!("dl")),
        ElementName::Dt         => make!(HTMLElement, atom!("dt")),
        ElementName::Em         => make!(HTMLElement, atom!("em")),
        ElementName::Embed      => make!(HTMLEmbedElement, atom!("embed")),
        ElementName::Fieldset   => make!(HTMLFieldSetElement, atom!("fieldset")),
        ElementName::Figcaption => make!(HTMLElement, atom!("figcaption")),
        ElementName::Figure     => make!(HTMLElement, atom!("figure")),
        ElementName::Font       => make!(HTMLFontElement, atom!("font")),
        ElementName::Footer     => make!(HTMLElement, atom!("footer")),
        ElementName::Form       => make!(HTMLFormElement, atom!("form")),
        ElementName::Frame      => make!(HTMLFrameElement, atom!("frame")),
        ElementName::Frameset   => make!(HTMLFrameSetElement, atom!("frameset")),
        ElementName::H1         => make!(HTMLHeadingElement, atom!("h1"), HeadingLevel::Heading1),
        ElementName::H2         => make!(HTMLHeadingElement, atom!("h2"), HeadingLevel::Heading2),
        ElementName::H3         => make!(HTMLHeadingElement, atom!("h3"), HeadingLevel::Heading3),
        ElementName::H4         => make!(HTMLHeadingElement, atom!("h4"), HeadingLevel::Heading4),
        ElementName::H5         => make!(HTMLHeadingElement, atom!("h5"), HeadingLevel::Heading5),
        ElementName::H6         => make!(HTMLHeadingElement, atom!("h6"), HeadingLevel::Heading6),
        ElementName::Head       => make!(HTMLHeadElement, atom!("head")),
        ElementName::Header     => make!(HTMLElement, atom!("header")),
        ElementName::Hgroup     => make!(HTMLElement, atom!("hgroup")),
        ElementName::Hr         => make!(HTMLHRElement, atom!("hr")),
        ElementName::Html       => make!(HTMLHtmlElement, atom!("html")),
        ElementName::I          => make!(HTMLElement, atom!("i")),
        ElementName::Img        => make!(HTMLImageElement, atom!("img")),
        ElementName::Input      => make!(HTMLInputElement, atom!("input")),
        ElementName::Ins        => make!(HTMLModElement, atom!("ins")),
        ElementName::Isindex    => make!(HTMLUnknownElement, atom!("isindex")),
        ElementName::Kbd        => make!(HTMLElement, atom!("kbd")),
        ElementName::Label      => make!(HTMLLabelElement, atom!("label")),
        ElementName::Legend     => make!(HTMLLegendElement, atom!("legend")),
        ElementName::Li         => make!(HTMLLIElement, atom!("li")),
        ElementName::Link       => make!(HTMLLinkElement, atom!("link"), creator),
        ElementName::Listing    => make!(HTMLPreElement, atom!("listing")),
        ElementName::Main       => make!(HTMLElement, atom!("main")),
        ElementName::Map        => make!(HTMLMapElement, atom!("map")),
        ElementName::Mark       => make!(HTMLElement, atom!("mark")),
        ElementName::Marquee    => make!(HTMLElement, atom!("marquee")),
        ElementName::Meta       => make!(HTMLMetaElement, atom!("meta")),
        ElementName::Meter      => make!(HTMLMeterElement, atom!("meter")),
        ElementName::Multicol   => make!(HTMLUnknownElement, atom!("multicol")),
        ElementName::Nav        => make!(HTMLElement, atom!("nav")),
        ElementName::Nextid     => make!(HTMLUnknownElement, atom!("nextid")),
        ElementName::Nobr       => make!(HTMLElement, atom!("nobr")),
        ElementName::Noframes   => make!(HTMLElement, atom!("noframes")),
        ElementName::Noscript   => make!(HTMLElement, atom!("noscript")),
        ElementName::Object     => make!(HTMLObjectElement, atom!("object")),
        ElementName::Ol         => make!(HTMLOListElement, atom!("ol")),
        ElementName::Optgroup   => make!(HTMLOptGroupElement, atom!("optgroup")),
        ElementName::Option     => make!(HTMLOptionElement, atom!("option")),
        ElementName::Output     => make!(HTMLOutputElement, atom!("output")),
        ElementName::P          => make!(HTMLParagraphElement, atom!("p")),
        ElementName::Param      => make!(HTMLParamElement, atom!("param")),
        ElementName::Plaintext  => make!(HTMLPreElement, atom!("plaintext")),
        ElementName::Pre        => make!(HTMLPreElement, atom!("pre")),
        ElementName::Progress   => make!(HTMLProgressElement, atom!("progress")),
        ElementName::Q          => make!(HTMLQuoteElement, atom!("q")),
        ElementName::Rp         => make!(HTMLElement, atom!("rp")),
        ElementName::Rt         => make!(HTMLElement, atom!("rt")),
        ElementName::Ruby       => make!(HTMLElement, atom!("ruby")),
        ElementName::S          => make!(HTMLElement, atom!("s")),
        ElementName::Samp       => make!(HTMLElement, atom!("samp")),
        ElementName::Section    => make!(HTMLElement, atom!("section")),
        ElementName::Select     => make!(HTMLSelectElement, atom!("select")),
        ElementName::Small      => make!(HTMLElement, atom!("small")),
        ElementName::Source     => make!(HTMLSourceElement, atom!("source")),
        ElementName::Spacer     => make!(HTMLUnknownElement, atom!("spacer")),
        ElementName::Span       => make!(HTMLSpanElement, atom!("span")),
        ElementName::Strike     => make!(HTMLElement, atom!("strike")),
        ElementName::Strong     => make!(HTMLElement, atom!("strong")),
        ElementName::Style      => make!(HTMLStyleElement, atom!("style")),
        ElementName::Sub        => make!(HTMLElement, atom!("sub")),
        ElementName::Summary    => make!(HTMLElement, atom!("summary")),
        ElementName::Sup        => make!(HTMLElement, atom!("sup")),
        ElementName::Table      => make!(HTMLTableElement, atom!("table")),
        ElementName::Tbody      => make!(HTMLTableSectionElement, atom!("tbody")),
        ElementName::Td         => make!(HTMLTableDataCellElement, atom!("td")),
        ElementName::Template   => make!(HTMLTemplateElement, atom!("template")),
        ElementName::Textarea   => make!(HTMLTextAreaElement, atom!("textarea")),
        ElementName::Tfoot      => make!(HTMLTableSectionElement, atom!("tfoot")),
        ElementName::Th         => make!(HTMLTableHeaderCellElement, atom!("th")),
        ElementName::Thead      => make!(HTMLTableSectionElement, atom!("thead")),
        ElementName::Time       => make!(HTMLTimeElement, atom!("time")),
        ElementName::Title      => make!(HTMLTitleElement, atom!("title")),
        ElementName::Tr         => make!(HTMLTableRowElement, atom!("tr")),
        ElementName::Tt         => make!(HTMLElement, atom!("tt")),
        ElementName::Track      => make!(HTMLTrackElement, atom!("track")),
        ElementName::U          => make!(HTMLElement, atom!("u")),
        ElementName::Ul         => make!(HTMLUListElement, atom!("ul")),
        ElementName::Var        => make!(HTMLElement, atom!("var")),
        ElementName::Video      => make!(HTMLVideoElement, atom!("video")),
        ElementName::Wbr        => make!(HTMLElement, atom!("wbr")),
        ElementName::Xmp        => make!(HTMLPreElement, atom!("xmp")),
    }
}
