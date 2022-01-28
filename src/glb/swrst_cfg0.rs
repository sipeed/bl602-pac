#[doc = "Register `swrst_cfg0` reader"]
pub struct R(crate::R<SWRST_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg0` writer"]
pub struct W(crate::W<SWRST_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG0_SPEC>;
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
impl From<crate::W<SWRST_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `swrst_s30` reader - "]
pub struct SWRST_S30_R(crate::FieldReader<bool, bool>);
impl SWRST_S30_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s30` writer - "]
pub struct SWRST_S30_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S30_W<'a> {
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
#[doc = "Field `swrst_s20` reader - "]
pub struct SWRST_S20_R(crate::FieldReader<bool, bool>);
impl SWRST_S20_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s20` writer - "]
pub struct SWRST_S20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `swrst_s01` reader - "]
pub struct SWRST_S01_R(crate::FieldReader<bool, bool>);
impl SWRST_S01_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s01` writer - "]
pub struct SWRST_S01_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S01_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `swrst_s00` reader - "]
pub struct SWRST_S00_R(crate::FieldReader<bool, bool>);
impl SWRST_S00_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S00_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S00_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s00` writer - "]
pub struct SWRST_S00_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S00_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&self) -> SWRST_S30_R {
        SWRST_S30_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&self) -> SWRST_S20_R {
        SWRST_S20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&self) -> SWRST_S01_R {
        SWRST_S01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&self) -> SWRST_S00_R {
        SWRST_S00_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&mut self) -> SWRST_S30_W {
        SWRST_S30_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&mut self) -> SWRST_S20_W {
        SWRST_S20_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&mut self) -> SWRST_S01_W {
        SWRST_S01_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&mut self) -> SWRST_S00_W {
        SWRST_S00_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg0](index.html) module"]
pub struct SWRST_CFG0_SPEC;
impl crate::RegisterSpec for SWRST_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg0::R](R) reader structure"]
impl crate::Readable for SWRST_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg0::W](W) writer structure"]
impl crate::Writable for SWRST_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets swrst_cfg0 to value 0"]
impl crate::Resettable for SWRST_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
