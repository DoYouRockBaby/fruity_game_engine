use crate::runtime_tuple::TupleEntryInfo;

pub struct RuntimeTupleReader<'s> {
    entry_infos: &'s [TupleEntryInfo],
    buffer: &'s [u8],
}
