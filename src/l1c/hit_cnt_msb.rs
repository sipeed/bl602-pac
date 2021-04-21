#[doc = "Register `hit_cnt_msb` reader"]
pub struct R(crate::R<HIT_CNT_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIT_CNT_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HIT_CNT_MSB_SPEC>> for R {
    fn from(reader: crate::R<HIT_CNT_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `hit_cnt_msb` reader - "]
pub struct HIT_CNT_MSB_R(crate::FieldReader<u32, u32>);
impl HIT_CNT_MSB_R {
    pub(crate) fn new(bits: u32) -> Self {
        HIT_CNT_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIT_CNT_MSB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_msb(&self) -> HIT_CNT_MSB_R {
        HIT_CNT_MSB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "hit_cnt_msb.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hit_cnt_msb](index.html) module"]
pub struct HIT_CNT_MSB_SPEC;
impl crate::RegisterSpec for HIT_CNT_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hit_cnt_msb::R](R) reader structure"]
impl crate::Readable for HIT_CNT_MSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets hit_cnt_msb to value 0"]
impl crate::Resettable for HIT_CNT_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
