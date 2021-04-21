#[doc = "Register `pds_ram1` reader"]
pub struct R(crate::R<PDS_RAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDS_RAM1_SPEC>> for R {
    fn from(reader: crate::R<PDS_RAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_ram1` writer"]
pub struct W(crate::W<PDS_RAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<PDS_RAM1_SPEC>> for W {
    fn from(writer: crate::W<PDS_RAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_np_sram_pwr` reader - "]
pub struct CR_NP_SRAM_PWR_R(crate::FieldReader<u8, u8>);
impl CR_NP_SRAM_PWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_NP_SRAM_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_NP_SRAM_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_np_sram_pwr` writer - "]
pub struct CR_NP_SRAM_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_NP_SRAM_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&self) -> CR_NP_SRAM_PWR_R {
        CR_NP_SRAM_PWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&mut self) -> CR_NP_SRAM_PWR_W {
        CR_NP_SRAM_PWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_ram1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ram1](index.html) module"]
pub struct PDS_RAM1_SPEC;
impl crate::RegisterSpec for PDS_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ram1::R](R) reader structure"]
impl crate::Readable for PDS_RAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ram1::W](W) writer structure"]
impl crate::Writable for PDS_RAM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pds_ram1 to value 0"]
impl crate::Resettable for PDS_RAM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
