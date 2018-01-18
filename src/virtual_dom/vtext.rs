//! This module contains the implementation of a virtual text node `VText`.

use std::fmt;
use std::cmp::PartialEq;
use std::marker::PhantomData;
use stdweb::web::{INode, Element, TextNode, document};
use virtual_dom::{VTag, VNode, VComp};
use html::{ScopeEnv, Component};

/// A type for a virtual
/// [TextNode](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// represenation.
pub struct VText<CTX, COMP: Component<CTX>> {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `Element`.
    pub reference: Option<TextNode>,
    _ctx: PhantomData<CTX>,
    _comp: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VText<CTX, COMP> {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
            _ctx: PhantomData,
            _comp: PhantomData,
        }
    }

    /// Remove VTag from parent.
    pub fn remove(self, parent: &Element) {
        let node = self.reference.expect("tried to remove not rendered VText from DOM");
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VText");
        }
    }

    /// Renders virtual node over existent `TextNode`, but
    /// only if value of text had changed.
    pub fn apply(&mut self, parent: &Element, opposite: Option<VNode<CTX, COMP>>, _: ScopeEnv<CTX, COMP>) {
        match opposite {
            // If element matched this type
            Some(VNode::VText(VText { text, reference: Some(element), .. })) => {
                if self.text != text {
                    element.set_node_value(Some(&self.text));
                }
                self.reference = Some(element);
            }
            // If element exists, but have a wrong type
            Some(VNode::VTag(VTag { reference: Some(wrong), .. })) |
            Some(VNode::VComp(VComp { reference: Some(wrong), .. })) => {
                let element = document().create_text_node(&self.text);
                parent.replace_child(&element, &wrong);
                self.reference = Some(element);
            }
            // If element not exists
            Some(VNode::VTag(VTag { reference: None, .. })) |
            Some(VNode::VComp(VComp { reference: None, .. })) |
            Some(VNode::VText(VText { reference: None, .. })) |
            None => {
                let element = document().create_text_node(&self.text);
                parent.append_child(&element);
                self.reference = Some(element);
            }
        }
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VText<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VText<CTX, COMP> {
    fn eq(&self, other: &VText<CTX, COMP>) -> bool {
        return self.text == other.text;
    }
}
