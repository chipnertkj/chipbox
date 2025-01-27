use super::data;

pub enum Op {}

impl Op {
    pub fn render<const FRAME_COUNT: usize>(&self, dataset: &mut data::Dataset<FRAME_COUNT>) {}
}
