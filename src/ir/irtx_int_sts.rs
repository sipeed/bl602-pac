#[doc = "Register `irtx_int_sts` reader"]
pub struct R(crate::R<IRTX_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRTX_INT_STS_SPEC>> for R {
    fn from(reader: crate::R<IRTX_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_int_sts` writer"]
pub struct W(crate::W<IRTX_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_INT_STS_SPEC>;
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
impl core::convert::From<crate::W<IRTX_INT_STS_SPEC>> for W {
    fn from(writer: crate::W<IRTX_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_end_en` reader - "]
pub struct CR_IRTX_END_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_END_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_END_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_END_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_end_en` writer - "]
pub struct CR_IRTX_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_END_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `cr_irtx_end_clr` reader - "]
pub struct CR_IRTX_END_CLR_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_END_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_END_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_END_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_end_clr` writer - "]
pub struct CR_IRTX_END_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_END_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cr_irtx_end_mask` reader - "]
pub struct CR_IRTX_END_MASK_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_END_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_END_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_END_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_end_mask` writer - "]
pub struct CR_IRTX_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_END_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `irtx_end_int` reader - "]
pub struct IRTX_END_INT_R(crate::FieldReader<bool, bool>);
impl IRTX_END_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRTX_END_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRTX_END_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&self) -> CR_IRTX_END_EN_R {
        CR_IRTX_END_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&self) -> CR_IRTX_END_CLR_R {
        CR_IRTX_END_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&self) -> CR_IRTX_END_MASK_R {
        CR_IRTX_END_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irtx_end_int(&self) -> IRTX_END_INT_R {
        IRTX_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&mut self) -> CR_IRTX_END_EN_W {
        CR_IRTX_END_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&mut self) -> CR_IRTX_END_CLR_W {
        CR_IRTX_END_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&mut self) -> CR_IRTX_END_MASK_W {
        CR_IRTX_END_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_int_sts](index.html) module"]
pub struct IRTX_INT_STS_SPEC;
impl crate::RegisterSpec for IRTX_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_int_sts::R](R) reader structure"]
impl crate::Readable for IRTX_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_int_sts::W](W) writer structure"]
impl crate::Writable for IRTX_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irtx_int_sts to value 0x0100_0100"]
impl crate::Resettable for IRTX_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
