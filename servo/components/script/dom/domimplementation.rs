/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use document_loader::DocumentLoader;

use dom::bindings::codegen::Bindings::DOMImplementationBinding::DOMImplementationMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector};
use dom::bindings::xmlname::validate_qualified_name;
use dom::document::DocumentSource;
use dom::document::{Document, IsHTMLDocument};
use dom::documenttype::DocumentType;
use dom::htmlbodyelement::HTMLBodyElement;
use dom::htmlheadelement::HTMLHeadElement;
use dom::htmlhtmlelement::HTMLHtmlElement;
use dom::htmltitleelement::HTMLTitleElement;
use dom::node::Node;
use dom::text::Text;
use util::str::DOMString;

// https://dom.spec.whatwg.org/#domimplementation
#[dom_struct]
pub struct DOMImplementation {
    reflector_: Reflector,
    document: JS<Document>,
}

impl DOMImplementation {
    fn new_inherited(document: &Document) -> DOMImplementation {
        DOMImplementation {
            reflector_: Reflector::new(),
            document: JS::from_ref(document),
        }
    }

    pub fn new(document: &Document) -> Root<DOMImplementation> {
        let window = document.window();
        Root::new_box(box DOMImplementation::new_inherited(document))
    }
}

// https://dom.spec.whatwg.org/#domimplementation
impl DOMImplementationMethods for DOMImplementation {
    // https://dom.spec.whatwg.org/#dom-domimplementation-createdocumenttype
    fn CreateDocumentType(&self,
                          qualified_name: DOMString,
                          pubid: DOMString,
                          sysid: DOMString)
                          -> Fallible<Root<DocumentType>> {
        try!(validate_qualified_name(&qualified_name));
        Ok(DocumentType::new(qualified_name, Some(pubid), Some(sysid), &self.document))
    }

    // https://dom.spec.whatwg.org/#dom-domimplementation-createhtmldocument
    fn CreateHTMLDocument(&self, title: Option<DOMString>) -> Root<Document> {
        let win = self.document.window();
        let loader = DocumentLoader::new(&self.document.loader());

        // Step 1-2.
        let doc = Document::new(win,
                                None,
                                None,
                                IsHTMLDocument::HTMLDocument,
                                None,
                                None,
                                DocumentSource::NotFromParser,
                                loader);

        {
            // Step 3.
            let doc_node = doc.upcast::<Node>();
            let doc_type = DocumentType::new(DOMString::from("html"), None, None, doc.r());
            doc_node.AppendChild(doc_type.upcast()).unwrap();
        }

        {
            // Step 4.
            let doc_node = doc.upcast::<Node>();
            let doc_html = Root::upcast::<Node>(HTMLHtmlElement::new(atom!("html"),
                                                                     None,
                                                                     doc.r()));
            doc_node.AppendChild(&doc_html).expect("Appending failed");

            {
                // Step 5.
                let doc_head = Root::upcast::<Node>(HTMLHeadElement::new(atom!("head"),
                                                                         None,
                                                                         doc.r()));
                doc_html.AppendChild(&doc_head).unwrap();

                // Step 6.
                match title {
                    None => (),
                    Some(title_str) => {
                        // Step 6.1.
                        let doc_title =
                            Root::upcast::<Node>(HTMLTitleElement::new(atom!("title"),
                                                                       None,
                                                                       doc.r()));
                        doc_head.AppendChild(&doc_title).unwrap();

                        // Step 6.2.
                        let title_text = Text::new(title_str, doc.r());
                        doc_title.AppendChild(title_text.upcast()).unwrap();
                    }
                }
            }

            // Step 7.
            let doc_body = HTMLBodyElement::new(atom!("body"), None, doc.r());
            doc_html.AppendChild(doc_body.upcast()).unwrap();
        }

        // Step 8.
        // FIXME: https://github.com/mozilla/servo/issues/1522

        // Step 9.
        doc
    }

    // https://dom.spec.whatwg.org/#dom-domimplementation-hasfeature
    fn HasFeature(&self) -> bool {
        true
    }
}
