use super::{prelude::*, *};
use crate::{errors::*, html, http, models::*};
use encoding_rs::GBK;
// use lazy_static::lazy_static;
// use regex::Regex;

pub struct Kkdm;

impl Extractor for Kkdm {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = format!("http://comic.kukudm.com/comictype/3_{}.htm", more + 1);
        let mut fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, "#comicmain > dd > a:nth-child(2)", vec![]);
        fll.set_encoding(GBK)
            .set_href_prefix("http://comic.kukudm.com");
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&detail.url, "#comiclistn > dd > a:nth-child(1)", vec![]);
        fll.set_encoding(GBK)
            .set_href_prefix("http://comic.kukudm.com");
        detail.section_list = fll.try_get_list()?.result()?;
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                let _doc = html::parse_document(&html_s);
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kkdm_index() {
        let list = Kkdm {}.index(0).unwrap();
        assert_eq!(21, list.len());
    }

    #[test]
    fn test_kkdm_fetch_sections() {
        let mut detail = Detail::new(
            UNKNOWN_NAME,
            "http://comic.kukudm.com/comiclist/2612/index.htm",
        );
        Kkdm {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(10, detail.section_list.len());
    }
}
