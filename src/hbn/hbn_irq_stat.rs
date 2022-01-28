#[doc = "Register `HBN_IRQ_STAT` reader"]
pub struct R(crate::R<HBN_IRQ_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_IRQ_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_IRQ_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_IRQ_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `irq_stat` reader - "]
pub struct IRQ_STAT_R(crate::FieldReader<u32, u32>);
impl IRQ_STAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        IRQ_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_STAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_stat(&self) -> IRQ_STAT_R {
        IRQ_STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "HBN_IRQ_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_stat](index.html) module"]
pub struct HBN_IRQ_STAT_SPEC;
impl crate::RegisterSpec for HBN_IRQ_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_irq_stat::R](R) reader structure"]
impl crate::Readable for HBN_IRQ_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HBN_IRQ_STAT to value 0"]
impl crate::Resettable for HBN_IRQ_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
