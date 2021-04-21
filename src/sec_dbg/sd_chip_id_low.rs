#[doc = "Register `sd_chip_id_low` reader"]
pub struct R(crate::R<SD_CHIP_ID_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CHIP_ID_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SD_CHIP_ID_LOW_SPEC>> for R {
    fn from(reader: crate::R<SD_CHIP_ID_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sd_chip_id_low` reader - "]
pub struct SD_CHIP_ID_LOW_R(crate::FieldReader<u32, u32>);
impl SD_CHIP_ID_LOW_R {
    pub(crate) fn new(bits: u32) -> Self {
        SD_CHIP_ID_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_CHIP_ID_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_low(&self) -> SD_CHIP_ID_LOW_R {
        SD_CHIP_ID_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "sd_chip_id_low.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_chip_id_low](index.html) module"]
pub struct SD_CHIP_ID_LOW_SPEC;
impl crate::RegisterSpec for SD_CHIP_ID_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_chip_id_low::R](R) reader structure"]
impl crate::Readable for SD_CHIP_ID_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sd_chip_id_low to value 0"]
impl crate::Resettable for SD_CHIP_ID_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
