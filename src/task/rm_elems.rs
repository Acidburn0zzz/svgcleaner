/****************************************************************************
**
** svgcleaner could help you to clean up your SVG files
** from unnecessary data.
** Copyright (C) 2012-2016 Evgeniy Reizner
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation; either version 2 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License along
** with this program; if not, write to the Free Software Foundation, Inc.,
** 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
**
****************************************************************************/

use super::short::EId;

use svgdom::Document;

pub fn remove_element(doc: &Document, id: EId) {
    for node in doc.descendants().svg().filter(|n| n.is_tag_id(id)).collect::<Vec<_>>() {
        node.remove();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{Document, WriteToString, ElementId};

    macro_rules! test {
        ($name:ident, $id:expr, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let doc = Document::from_data($in_text).unwrap();
                remove_element(&doc, $id);
                assert_eq_text!(doc.to_string_with_opt(&write_opt_for_tests!()), $out_text);
            }
        )
    }

    test!(rm_1, ElementId::Title,
b"<svg>
    <title/>
</svg>",
"<svg/>
");

    test!(rm_2, ElementId::Title,
b"<svg>
    <title/>
    <title/>
    <rect/>
    <title/>
</svg>",
"<svg>
    <rect/>
</svg>
");

    test!(rm_3, ElementId::Title,
b"<svg>
    <title>
        <title/>
        <rect/>
    </title>
    <rect/>
</svg>",
"<svg>
    <rect/>
</svg>
");
}
