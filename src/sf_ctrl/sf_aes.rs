#[doc = "Register `sf_aes` reader"]
pub struct R(crate::R<SF_AES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_AES_SPEC>> for R {
    fn from(reader: crate::R<SF_AES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes` writer"]
pub struct W(crate::W<SF_AES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_SPEC>;
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
impl core::convert::From<crate::W<SF_AES_SPEC>> for W {
    fn from(writer: crate::W<SF_AES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_status` reader - "]
pub struct SF_AES_STATUS_R(crate::FieldReader<u32, u32>);
impl SF_AES_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_AES_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_pref_busy` reader - "]
pub struct SF_AES_PREF_BUSY_R(crate::FieldReader<bool, bool>);
impl SF_AES_PREF_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_PREF_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_PREF_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_pref_trig` reader - "]
pub struct SF_AES_PREF_TRIG_R(crate::FieldReader<bool, bool>);
impl SF_AES_PREF_TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_PREF_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_PREF_TRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_pref_trig` writer - "]
pub struct SF_AES_PREF_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_PREF_TRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `sf_aes_mode` reader - "]
pub struct SF_AES_MODE_R(crate::FieldReader<u8, u8>);
impl SF_AES_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_AES_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_mode` writer - "]
pub struct SF_AES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `sf_aes_en` reader - "]
pub struct SF_AES_EN_R(crate::FieldReader<bool, bool>);
impl SF_AES_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_en` writer - "]
pub struct SF_AES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_EN_W<'a> {
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
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn sf_aes_status(&self) -> SF_AES_STATUS_R {
        SF_AES_STATUS_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_pref_busy(&self) -> SF_AES_PREF_BUSY_R {
        SF_AES_PREF_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&self) -> SF_AES_PREF_TRIG_R {
        SF_AES_PREF_TRIG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&self) -> SF_AES_MODE_R {
        SF_AES_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&self) -> SF_AES_EN_R {
        SF_AES_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&mut self) -> SF_AES_PREF_TRIG_W {
        SF_AES_PREF_TRIG_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&mut self) -> SF_AES_MODE_W {
        SF_AES_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&mut self) -> SF_AES_EN_W {
        SF_AES_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes](index.html) module"]
pub struct SF_AES_SPEC;
impl crate::RegisterSpec for SF_AES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes::R](R) reader structure"]
impl crate::Readable for SF_AES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes::W](W) writer structure"]
impl crate::Writable for SF_AES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_aes to value 0x40"]
impl crate::Resettable for SF_AES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
