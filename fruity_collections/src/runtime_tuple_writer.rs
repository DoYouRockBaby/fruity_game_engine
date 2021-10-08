use crate::runtime_tuple::TupleEntryInfo;

pub struct RuntimeTupleWriter<'s> {
    entry_infos: &'s [TupleEntryInfo],
    buffer: &'s mut [u8],
}
