use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;

use crate::{Document, Element};

/// Produces an index for each [`Element`] in the [`Document']. Provides lookups for an element's
/// parent. This requires ownership of the document and pins the document.
///
/// # Example
///
/// ```
/// use exile::Document;
/// let xml = "<a><b/><c><d>x</d><e/></c></a>";
/// let doc = exile::parse(xml).unwrap();
///
/// // create an index consuming the document
/// let index = doc.index();
///
/// // get the element at index 3 (depth-first numbering).
/// let d = index.element(3).unwrap();
///
/// // get that element's parent
/// let c = index.parent(&d).unwrap();
///
/// // prove it!
/// assert_eq!("c", c.name());
/// ```
#[derive(Clone, Debug)]
pub struct Index {
    /// Ownership held in a way that lets us use pointers to elements.
    doc: RefCell<Pin<Box<Document>>>,
    /// The index and mapping of the elements.
    info: Info,
}

impl Index {
    /// Create an index from `doc`.
    pub fn build(doc: Document) -> Self {
        let refcell = RefCell::new(Pin::new(Box::new(doc)));
        let something = unsafe { refcell.as_ptr().as_ref().unwrap() };
        let elements = Info::build(something);
        Self {
            doc: refcell,
            info: elements,
        }
    }

    /// Access the inner [`Document`].
    pub fn doc(&self) -> &Document {
        unsafe { self.doc.as_ptr().as_ref().unwrap() }
    }

    /// Access the root [`Element`] of the [`Document`].
    pub fn root(&self) -> &Element {
        self.doc().root()
    }

    /// Destroy the index and return ownership of the inner [`Document`].
    pub fn into_doc(self) -> Document {
        let inner = self.doc.into_inner();
        let inner_inner = Pin::<Box<Document>>::into_inner(inner);
        *inner_inner
    }

    /// Gets the element at `index`.
    pub fn element(&self, index: usize) -> Option<&Element> {
        self.info
            .vec
            .get(index)
            .map(|value| unsafe { value.element.as_ref().unwrap() })
    }

    /// Gets the index of the given `element`.
    pub fn index(&self, element: &Element) -> Option<usize> {
        let ptr: *const Element = element;
        self.info.map.get(&ptr).copied()
    }

    /// Gets the parent of the given `element`.
    pub fn parent(&self, element: &Element) -> Option<&Element> {
        let child_index = self.index(element)?;

        self.parent_index(child_index)
            .and_then(|parent_index| self.element(parent_index))
    }

    /// Gets the index of the parent of the `element` (which is given by index in this case).
    pub fn parent_index(&self, element: usize) -> Option<usize> {
        self.info
            .vec
            .get(element)
            .map(|info| info.parent)
            .and_then(|ix| if ix == usize::MAX { None } else { Some(ix) })
    }
}

/// A private type used to hold a pointer to an element and this index of its parent.
#[derive(Copy, Clone, Debug)]
struct Pointer {
    parent: usize,
    element: *const Element,
}

/// Holds the element pointers and their index mapping.
#[derive(Clone, Debug, Default)]
struct Info {
    vec: Vec<Pointer>,
    map: HashMap<*const Element, usize>,
}

impl Info {
    fn build(doc: &Document) -> Self {
        let mut elements = Info::default();
        elements.build_recursively(usize::MAX, doc.root());
        elements
    }

    fn build_recursively(&mut self, parent_index: usize, element: &Element) {
        let element_index = self.insert(parent_index, element);
        for next_child in element.children() {
            self.build_recursively(element_index, next_child)
        }
    }

    fn insert(&mut self, parent_index: usize, element: &Element) -> usize {
        let index = self.vec.len();
        let link = Pointer {
            parent: parent_index,
            element,
        };
        self.map.insert(element, index);
        self.vec.push(link);
        index
    }
}

#[test]
fn index_test() {
    const XML: &str = r#"
<a expected_index="0">
    <b expected_index="1" expected_parent="0" expected_parent_name="a"/>
    <c expected_index="2" expected_parent="0" expected_parent_name="a"/>
    <d expected_index="3" expected_parent="0" expected_parent_name="a">
        <e expected_index="4" expected_parent="3" expected_parent_name="d">
            <f expected_index="5" expected_parent="4" expected_parent_name="e">
                <g  expected_index="6" expected_parent="5" expected_parent_name="f"/>
                <h  expected_index="7" expected_parent="5" expected_parent_name="f"/>
                <i  expected_index="8" expected_parent="5" expected_parent_name="f">
                    <j expected_index="9" expected_parent="8" expected_parent_name="i"/>
                </i>
            </f>
        </e>
    </d>
</a>
    "#;
    let doc = crate::parse(XML).unwrap();
    let index = Index::build(doc);
    // let _wassup = format!("{:?}", index);
    // println!("{}", _wassup);
    for i in 0usize..9 {
        let element = index.element(i).unwrap();
        let expected_index = element
            .attribute("expected_index")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(i, expected_index);
        let get_index_result = index.index(element).unwrap();
        assert_eq!(i, get_index_result);
        if i == 0 {
            assert!(index.parent_index(0).is_none())
        } else {
            let parent_index = index.parent_index(i).unwrap();
            let expected_parent_index = element
                .attribute("expected_parent")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            assert_eq!(expected_parent_index, parent_index);
            let parent = index.parent(element).unwrap();
            let expected_parent_name = element.attribute("expected_parent_name").unwrap();
            let actual_parent_name = parent.name();
            assert_eq!(expected_parent_name.as_str(), actual_parent_name);
        }
    }
    assert_eq!("a", index.doc().root().name());
    let doc = index.into_doc();
    assert_eq!("a", doc.root().name());
}
