//! Transform an object module into a load module

use crate::references::resolve_references;
use crate::relocation::relocate;
use crate::util::{make_symbol_table, R2KStrings};
use mips_types::constants::{DATA_OFFSET, TEXT_OFFSET};
use mips_types::module::{
    R2KModule, R2KModuleHeader, R2KSection, R2KVersion, R2K_MAGIC, REFERENCES_INDEX,
    RELOCATION_INDEX,
};

pub fn obj_to_load_module(obj_module: R2KModule) -> R2KModule {
    let entry = TEXT_OFFSET;
    let mut section_sizes = obj_module.header.section_sizes;
    let mut relocation = obj_module.relocation_section;
    let mut references = obj_module.reference_section;
    let mut text_section = obj_module.text_section;
    let mut data_section = obj_module.data_section;
    let strings = R2KStrings::new(&obj_module.string_table);
    let symbols = make_symbol_table(strings, &obj_module.symbol_table);

    relocate(
        &mut text_section,
        R2KSection::Text,
        TEXT_OFFSET,
        &mut relocation,
    );
    relocate(
        &mut data_section,
        R2KSection::Data,
        DATA_OFFSET,
        &mut relocation,
    );
    resolve_references(
        &mut text_section,
        R2KSection::Text,
        strings,
        &symbols,
        &mut references,
    );
    resolve_references(
        &mut data_section,
        R2KSection::Data,
        strings,
        &symbols,
        &mut references,
    );

    // FIXME: refactor and support all relocatable sections
    assert!(
        relocation.is_empty() && references.is_empty(),
        "Only text and data relocation/referencing is currently supported"
    );

    section_sizes[RELOCATION_INDEX] = relocation.len() as u32;
    section_sizes[REFERENCES_INDEX] = references.len() as u32;

    R2KModule {
        header: R2KModuleHeader {
            magic: R2K_MAGIC,
            version: R2KVersion::Version1,
            flags: 0,
            entry,
            section_sizes,
        },
        text_section,
        rdata_section: obj_module.rdata_section,
        data_section,
        sdata_section: obj_module.sdata_section,
        sbss_size: obj_module.sbss_size,
        bss_size: obj_module.bss_size,
        relocation_section: relocation,
        reference_section: references,
        symbol_table: obj_module.symbol_table,
        string_table: obj_module.string_table,
    }
}
