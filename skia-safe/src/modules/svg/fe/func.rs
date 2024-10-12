use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgFeFuncKind = sb::SkSVGFeFuncType;
pub type FeFunc = RCHandle<sb::SkSVGFeFunc>;

impl NativeRefCountedBase for sb::SkSVGFeFunc {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeFunc {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for FeFunc {
    const NAME: &'static str = "FeFunc";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("amplitude", &self.get_amplitude())
                .field("exponent", &self.get_exponent())
                .field("intercept", &self.get_intercept())
                .field("offset", &self.get_offset())
                .field("slope", &self.get_slope())
                .field("table_values", &self.get_table_values())
                .field("kind", self.get_kind()),
        );
    }
}

impl FeFunc {
    pub fn get_table_values(&self) -> &[scalar] {
        unsafe {
            safer::from_raw_parts(
                sb::C_SkSVGFeFunc_getTableValues(self.native()),
                self.get_table_values_count(),
            )
        }
    }

    pub fn get_table_values_count(&self) -> usize {
        unsafe { sb::C_SkSVGFeFunc_getTableValuesCount(self.native()) }
    }

    skia_macros::attrs! {
        SkSVGFeFunc => {
            *amplitude: scalar [get(value) => value, set(value) => value],
            *exponent: scalar [get(value) => value, set(value) => value],
            *intercept: scalar [get(value) => value, set(value) => value],
            *offset: scalar [get(value) => value, set(value) => value],
            *slope: scalar [get(value) => value, set(value) => value],
            "type" as kind: SvgFeFuncKind [get(value) => value, set(value) => value]
        }
    }
}
