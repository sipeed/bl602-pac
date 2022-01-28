#[doc = "Register `ef_crc_ctrl_3` reader"]
pub struct R(crate::R<EF_CRC_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CRC_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CRC_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CRC_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_crc_ctrl_3` writer"]
pub struct W(crate::W<EF_CRC_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CRC_CTRL_3_SPEC>;
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
impl From<crate::W<EF_CRC_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_CRC_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_crc_iv` reader - "]
pub struct EF_CRC_IV_R(crate::FieldReader<u32, u32>);
impl EF_CRC_IV_R {
    pub(crate) fn new(bits: u32) -> Self {
        EF_CRC_IV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_IV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_iv` writer - "]
pub struct EF_CRC_IV_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_IV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_iv(&self) -> EF_CRC_IV_R {
        EF_CRC_IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_iv(&mut self) -> EF_CRC_IV_W {
        EF_CRC_IV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_crc_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_3](index.html) module"]
pub struct EF_CRC_CTRL_3_SPEC;
impl crate::RegisterSpec for EF_CRC_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_crc_ctrl_3::R](R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_3::W](W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_crc_ctrl_3 to value 0xffff_ffff"]
impl crate::Resettable for EF_CRC_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
