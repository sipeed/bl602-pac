#[doc = "Register `clkpll_rz` reader"]
pub struct R(crate::R<CLKPLL_RZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_RZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLKPLL_RZ_SPEC>> for R {
    fn from(reader: crate::R<CLKPLL_RZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_rz` writer"]
pub struct W(crate::W<CLKPLL_RZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_RZ_SPEC>;
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
impl core::convert::From<crate::W<CLKPLL_RZ_SPEC>> for W {
    fn from(writer: crate::W<CLKPLL_RZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_rz` reader - "]
pub struct CLKPLL_RZ_R(crate::FieldReader<u8, u8>);
impl CLKPLL_RZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_RZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_RZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_rz` writer - "]
pub struct CLKPLL_RZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `clkpll_cz` reader - "]
pub struct CLKPLL_CZ_R(crate::FieldReader<u8, u8>);
impl CLKPLL_CZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_CZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_CZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_cz` writer - "]
pub struct CLKPLL_CZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `clkpll_c3` reader - "]
pub struct CLKPLL_C3_R(crate::FieldReader<u8, u8>);
impl CLKPLL_C3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_C3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_C3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_c3` writer - "]
pub struct CLKPLL_C3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_C3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `clkpll_r4_short` reader - "]
pub struct CLKPLL_R4_SHORT_R(crate::FieldReader<bool, bool>);
impl CLKPLL_R4_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_R4_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_R4_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_r4_short` writer - "]
pub struct CLKPLL_R4_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_R4_SHORT_W<'a> {
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
#[doc = "Field `clkpll_r4` reader - "]
pub struct CLKPLL_R4_R(crate::FieldReader<u8, u8>);
impl CLKPLL_R4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_R4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_R4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_r4` writer - "]
pub struct CLKPLL_R4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_R4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `clkpll_c4_en` reader - "]
pub struct CLKPLL_C4_EN_R(crate::FieldReader<bool, bool>);
impl CLKPLL_C4_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_C4_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_C4_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_c4_en` writer - "]
pub struct CLKPLL_C4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_C4_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&self) -> CLKPLL_RZ_R {
        CLKPLL_RZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&self) -> CLKPLL_CZ_R {
        CLKPLL_CZ_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&self) -> CLKPLL_C3_R {
        CLKPLL_C3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&self) -> CLKPLL_R4_SHORT_R {
        CLKPLL_R4_SHORT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&self) -> CLKPLL_R4_R {
        CLKPLL_R4_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&self) -> CLKPLL_C4_EN_R {
        CLKPLL_C4_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&mut self) -> CLKPLL_RZ_W {
        CLKPLL_RZ_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&mut self) -> CLKPLL_CZ_W {
        CLKPLL_CZ_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&mut self) -> CLKPLL_C3_W {
        CLKPLL_C3_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&mut self) -> CLKPLL_R4_SHORT_W {
        CLKPLL_R4_SHORT_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&mut self) -> CLKPLL_R4_W {
        CLKPLL_R4_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&mut self) -> CLKPLL_C4_EN_W {
        CLKPLL_C4_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_rz.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_rz](index.html) module"]
pub struct CLKPLL_RZ_SPEC;
impl crate::RegisterSpec for CLKPLL_RZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_rz::R](R) reader structure"]
impl crate::Readable for CLKPLL_RZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_rz::W](W) writer structure"]
impl crate::Writable for CLKPLL_RZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkpll_rz to value 0x0005_a020"]
impl crate::Resettable for CLKPLL_RZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_a020
    }
}
