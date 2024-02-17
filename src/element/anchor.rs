#[cfg(feature = "sparse_attr")]
use std::collections::HashMap;
use std::io::Cursor;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

#[cfg(feature = "crossorigin")]
use crate::attr::referrer_policy::ReferrerPolicy;
#[cfg(feature = "attr-core")]
use crate::attr::CoreAttr;
#[cfg(feature = "attr-styling")]
use crate::attr::StylingAttr;
#[cfg(feature = "attr-cond_proc")]
use crate::attr::CondProcAttr;
#[cfg(feature = "attr-event")]
use crate::attr::{GlobalEventAttr, DocElementEventAttr, GraphicalEventAttr};
use crate::attr::WriteInAttr;
use crate::element::xml_write;
use crate::error::Error;
use crate::push_attr;
use super::{impl_children, impl_tag, impl_to_string};

use super::{ChildKind, Children, ElementNode, TryToString, TagName, WriteXml};

pub struct Anchor {
    pub download: Option<String>,

    pub href: Option<String>,

    pub href_lang: Option<String>,

    #[cfg(feature = "exp")]
    pub ping: Option<Vec<String>>,

    #[cfg(feature = "crossorigin")]
    pub referrer_policy: Option<ReferrerPolicy>,

    #[cfg(feature = "attr-core")]
    pub core: CoreAttr,

    #[cfg(feature = "attr-styling")]
    pub styling: StylingAttr,

    #[cfg(feature = "attr-cond_proc")]
    pub cond_proc: CondProcAttr,

    #[cfg(feature = "attr-event")]
    pub global_ev: HashMap<GlobalEventAttr, String>,

    #[cfg(feature = "attr-event")]
    pub doc_el_ev: HashMap<DocElementEventAttr, String>,

    #[cfg(feature = "attr-event")]
    pub graphical_ev: HashMap<GraphicalEventAttr, String>,

    children: Option<Vec<ChildKind>>,
}

impl_children!(Anchor);
impl_tag!(Anchor, "a");
impl_to_string!(Anchor);

impl WriteXml for Anchor {
    fn write_xml(&self, writer: &mut quick_xml::Writer<Cursor<Vec<u8>>>) -> Result<(), Error> {
        let tag_name = self.tag_name();

        let mut el_owned = BytesStart::new(tag_name.as_str());
        let el = &mut el_owned;

        push_attr!(self.download, el, "download" <- String);
        push_attr!(self.href, el, "href" <- String);
        push_attr!(self.href_lang, el, "hreflang" <- String);

        #[cfg(feature = "attr-core")]
        self.core.write_in(el)?;

        #[cfg(feature = "attr-styling")]
        self.styling.write_in(el)?;

        #[cfg(feature = "attr-cond_proc")]
        self.cond_proc.write_in(el);

        #[cfg(feature = "attr-event")]
        {
            self.global_ev.write_in(el)?;
            self.doc_el_ev.write_in(el)?;
            self.graphical_ev.write_in(el)?;
        }

        #[cfg(feature = "exp")]
        push_attr!(self.ping, el, "ping" <- strings | " ");

        #[cfg(feature = "crossorigin")]
        push_attr!(self.referrer_policy, el, "referrerpolicy" <- ToString);

        xml_write!(writer, el_owned, self.children, tag_name);

        Ok(())
    }
}

impl ElementNode for Anchor {}

impl Default for Anchor {
    fn default() -> Self {
        Self {
            download: None,
            href: None,
            href_lang: None,
            children: None,
            #[cfg(feature = "exp")]
            ping: None,
            #[cfg(feature = "crossorigin")]
            referrer_policy: None,
            #[cfg(feature = "attr-core")]
            core: CoreAttr::default(),
            #[cfg(feature = "attr-styling")]
            styling: StylingAttr::default(),
            #[cfg(feature = "attr-event")]
            global_ev: HashMap::new(),
            #[cfg(feature = "attr-event")]
            doc_el_ev: HashMap::new(),
            #[cfg(feature = "attr-event")]
            graphical_ev: HashMap::new(),
        }
    }
}
