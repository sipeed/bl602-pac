#[doc = "Register `sf_aes_r2` reader"]
pub struct R(crate::R<SF_AES_R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AES_R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_AES_R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_AES_R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_aes_r2` writer"]
pub struct W(crate::W<SF_AES_R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AES_R2_SPEC>;
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
impl From<crate::W<SF_AES_R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_AES_R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_aes_r2_lock` reader - "]
pub struct SF_AES_R2_LOCK_R(crate::FieldReader<bool, bool>);
impl SF_AES_R2_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_R2_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_R2_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_r2_lock` writer - "]
pub struct SF_AES_R2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `sf_aes_r2_en` reader - "]
pub struct SF_AES_R2_EN_R(crate::FieldReader<bool, bool>);
impl SF_AES_R2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_R2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_R2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_r2_en` writer - "]
pub struct SF_AES_R2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `sf_aes_r2_hw_key_en` reader - "]
pub struct SF_AES_R2_HW_KEY_EN_R(crate::FieldReader<bool, bool>);
impl SF_AES_R2_HW_KEY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_R2_HW_KEY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_R2_HW_KEY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_r2_hw_key_en` writer - "]
pub struct SF_AES_R2_HW_KEY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_HW_KEY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `sf_aes_r2_start` reader - "]
pub struct SF_AES_R2_START_R(crate::FieldReader<u16, u16>);
impl SF_AES_R2_START_R {
    pub(crate) fn new(bits: u16) -> Self {
        SF_AES_R2_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_R2_START_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_r2_start` writer - "]
pub struct SF_AES_R2_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 14)) | ((value as u32 & 0x3fff) << 14);
        self.w
    }
}
#[doc = "Field `sf_aes_r2_end` reader - "]
pub struct SF_AES_R2_END_R(crate::FieldReader<u16, u16>);
impl SF_AES_R2_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        SF_AES_R2_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_R2_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_r2_end` writer - "]
pub struct SF_AES_R2_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r2_lock(&self) -> SF_AES_R2_LOCK_R {
        SF_AES_R2_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r2_en(&self) -> SF_AES_R2_EN_R {
        SF_AES_R2_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r2_hw_key_en(&self) -> SF_AES_R2_HW_KEY_EN_R {
        SF_AES_R2_HW_KEY_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r2_start(&self) -> SF_AES_R2_START_R {
        SF_AES_R2_START_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r2_end(&self) -> SF_AES_R2_END_R {
        SF_AES_R2_END_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r2_lock(&mut self) -> SF_AES_R2_LOCK_W {
        SF_AES_R2_LOCK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r2_en(&mut self) -> SF_AES_R2_EN_W {
        SF_AES_R2_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r2_hw_key_en(&mut self) -> SF_AES_R2_HW_KEY_EN_W {
        SF_AES_R2_HW_KEY_EN_W { w: self }
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r2_start(&mut self) -> SF_AES_R2_START_W {
        SF_AES_R2_START_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r2_end(&mut self) -> SF_AES_R2_END_W {
        SF_AES_R2_END_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_aes_r2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_r2](index.html) module"]
pub struct SF_AES_R2_SPEC;
impl crate::RegisterSpec for SF_AES_R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_aes_r2::R](R) reader structure"]
impl crate::Readable for SF_AES_R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_aes_r2::W](W) writer structure"]
impl crate::Writable for SF_AES_R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_aes_r2 to value 0x3fff"]
impl crate::Resettable for SF_AES_R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}
