#[doc = "Register `tzc_rom0_r1` reader"]
pub struct R(crate::R<TZC_ROM0_R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ROM0_R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_ROM0_R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_ROM0_R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tzc_rom0_r1_end` reader - "]
pub type TZC_ROM0_R1_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_rom0_r1_start` reader - "]
pub type TZC_ROM0_R1_START_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r1_end(&self) -> TZC_ROM0_R1_END_R {
        TZC_ROM0_R1_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r1_start(&self) -> TZC_ROM0_R1_START_R {
        TZC_ROM0_R1_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "tzc_rom0_r1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom0_r1](index.html) module"]
pub struct TZC_ROM0_R1_SPEC;
impl crate::RegisterSpec for TZC_ROM0_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_rom0_r1::R](R) reader structure"]
impl crate::Readable for TZC_ROM0_R1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tzc_rom0_r1 to value 0xffff"]
impl crate::Resettable for TZC_ROM0_R1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
