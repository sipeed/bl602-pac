#[doc = "Register `ef_crc_ctrl_5` reader"]
pub struct R(crate::R<EF_CRC_CTRL_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CRC_CTRL_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CRC_CTRL_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CRC_CTRL_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ef_crc_dout` reader - "]
pub struct EF_CRC_DOUT_R(crate::FieldReader<u32, u32>);
impl EF_CRC_DOUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        EF_CRC_DOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_DOUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_dout(&self) -> EF_CRC_DOUT_R {
        EF_CRC_DOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ef_crc_ctrl_5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_5](index.html) module"]
pub struct EF_CRC_CTRL_5_SPEC;
impl crate::RegisterSpec for EF_CRC_CTRL_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_crc_ctrl_5::R](R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ef_crc_ctrl_5 to value 0xffff_ffff"]
impl crate::Resettable for EF_CRC_CTRL_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
