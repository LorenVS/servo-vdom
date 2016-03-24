use dom::create::create_element_named;
use dom::bindings::js::Root;
use dom::bindings::inheritance::Castable;
use dom::document::Document;
use dom::element::{Element,ElementCreator};
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use dom::text::Text;
use servo_vdom_client::patch::*;
use std::io::{Read,Result,Error,ErrorKind};
use style::properties::parse_one_declaration;
use util::str::DOMString;

/// Reads a text node from a reader.
pub fn read_text_node<T:Read>(reader: &mut T, doc: &Document) -> Result<Root<Text>> {
	let (id,text) = try!(reader.read_text());
	Ok(Text::new(id, DOMString::from(text), doc))
}

pub fn read_element<T:Read>(reader: &mut T, doc: &Document) -> Result<Root<Element>> {
	let (id,name) = try!(reader.read_el());
	let element = create_element_named(id, name, doc, ElementCreator::ParserCreated);

	while let Some(attr) = try!(reader.read_attr()) {
		match attr {
			AttributeVal::Class(val) => {
				element.set_tokenlist_attribute(&atom!("class"), DOMString::from(val));
			},
			AttributeVal::Style(key, val) => {
				if let Some(htmlel) = element.downcast::<HTMLElement>() {
					htmlel.Style().SetPropertyValue(DOMString::from(key), DOMString::from(val));
				}
			}
		}
	}

	while let Some(child) = try!(read_node(reader, doc)) {
		element.upcast::<Node>().AppendChild(&*child);
	}

	Ok(element)
}

pub fn read_node<T:Read>(reader: &mut T, doc: &Document) -> Result<Option<Root<Node>>> {
	if let Some(node_type) = try!(reader.read_node_type()) {
		match node_type {
			NodeType::Text => read_text_node(reader, doc).map(|t| Some(Root::from_ref(t.upcast()))),
			NodeType::Element => read_element(reader, doc).map(|e| Some(Root::from_ref(e.upcast())))
		}
	} else {
		Ok(None)
	}
}