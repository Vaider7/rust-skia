use super::{BoundingBoxUnits, DebugAttributes, HasBase, Length};
use crate::prelude::*;
use skia_bindings as sb;

pub type Filter = RCHandle<sb::SkSVGFilter>;

impl DebugAttributes for Filter {
    const NAME: &'static str = "Filter";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("filter_units", self.get_filter_units())
                .field("primitive_units", self.get_primitive_units()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFilter {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFilter {
    type Base = sb::SkSVGContainer;
}

impl Filter {
    skia_macros::attrs! {
        SkSVGFilter => {
            x: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            y: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            width: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            height: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()],
            filter_units: BoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }],
            primitive_units: BoundingBoxUnits [get(value) => &value.fType, set(value) => sb::SkSVGObjectBoundingBoxUnits { fType: value }]
        }
    }
}
