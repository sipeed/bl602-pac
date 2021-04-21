#[doc = "Register `HBN_SRAM` reader"]
pub struct R(crate::R<HBN_SRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_SRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HBN_SRAM_SPEC>> for R {
    fn from(reader: crate::R<HBN_SRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_SRAM` writer"]
pub struct W(crate::W<HBN_SRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_SRAM_SPEC>;
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
impl core::convert::From<crate::W<HBN_SRAM_SPEC>> for W {
    fn from(writer: crate::W<HBN_SRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `retram_slp` reader - "]
pub struct RETRAM_SLP_R(crate::FieldReader<bool, bool>);
impl RETRAM_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRAM_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRAM_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `retram_slp` writer - "]
pub struct RETRAM_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAM_SLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `retram_ret` reader - "]
pub struct RETRAM_RET_R(crate::FieldReader<bool, bool>);
impl RETRAM_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRAM_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRAM_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `retram_ret` writer - "]
pub struct RETRAM_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAM_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&self) -> RETRAM_SLP_R {
        RETRAM_SLP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&self) -> RETRAM_RET_R {
        RETRAM_RET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&mut self) -> RETRAM_SLP_W {
        RETRAM_SLP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&mut self) -> RETRAM_RET_W {
        RETRAM_RET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_SRAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_sram](index.html) module"]
pub struct HBN_SRAM_SPEC;
impl crate::RegisterSpec for HBN_SRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_sram::R](R) reader structure"]
impl crate::Readable for HBN_SRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_sram::W](W) writer structure"]
impl crate::Writable for HBN_SRAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_SRAM to value 0"]
impl crate::Resettable for HBN_SRAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
