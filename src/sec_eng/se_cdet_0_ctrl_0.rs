#[doc = "Register `se_cdet_0_ctrl_0` reader"]
pub struct R(crate::R<SE_CDET_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_CDET_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_CDET_0_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_CDET_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_cdet_0_ctrl_0` writer"]
pub struct W(crate::W<SE_CDET_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_CDET_0_CTRL_0_SPEC>;
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
impl From<crate::W<SE_CDET_0_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_CDET_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_cdet_0_g_loop_min` reader - "]
pub struct SE_CDET_0_G_LOOP_MIN_R(crate::FieldReader<u8, u8>);
impl SE_CDET_0_G_LOOP_MIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_CDET_0_G_LOOP_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_0_G_LOOP_MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_0_g_loop_min` writer - "]
pub struct SE_CDET_0_G_LOOP_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_G_LOOP_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `se_cdet_0_g_loop_max` reader - "]
pub struct SE_CDET_0_G_LOOP_MAX_R(crate::FieldReader<u8, u8>);
impl SE_CDET_0_G_LOOP_MAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_CDET_0_G_LOOP_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_0_G_LOOP_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_0_g_loop_max` writer - "]
pub struct SE_CDET_0_G_LOOP_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_G_LOOP_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `se_cdet_0_status` reader - "]
pub struct SE_CDET_0_STATUS_R(crate::FieldReader<u16, u16>);
impl SE_CDET_0_STATUS_R {
    pub(crate) fn new(bits: u16) -> Self {
        SE_CDET_0_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_0_STATUS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_0_error` reader - "]
pub struct SE_CDET_0_ERROR_R(crate::FieldReader<bool, bool>);
impl SE_CDET_0_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_CDET_0_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_0_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_0_en` reader - "]
pub struct SE_CDET_0_EN_R(crate::FieldReader<bool, bool>);
impl SE_CDET_0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_CDET_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_0_en` writer - "]
pub struct SE_CDET_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&self) -> SE_CDET_0_G_LOOP_MIN_R {
        SE_CDET_0_G_LOOP_MIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&self) -> SE_CDET_0_G_LOOP_MAX_R {
        SE_CDET_0_G_LOOP_MAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn se_cdet_0_status(&self) -> SE_CDET_0_STATUS_R {
        SE_CDET_0_STATUS_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_0_error(&self) -> SE_CDET_0_ERROR_R {
        SE_CDET_0_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&self) -> SE_CDET_0_EN_R {
        SE_CDET_0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&mut self) -> SE_CDET_0_G_LOOP_MIN_W {
        SE_CDET_0_G_LOOP_MIN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&mut self) -> SE_CDET_0_G_LOOP_MAX_W {
        SE_CDET_0_G_LOOP_MAX_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&mut self) -> SE_CDET_0_EN_W {
        SE_CDET_0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_cdet_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_cdet_0_ctrl_0](index.html) module"]
pub struct SE_CDET_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_CDET_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_cdet_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_CDET_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_cdet_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_CDET_0_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_cdet_0_ctrl_0 to value 0x2164_0004"]
impl crate::Resettable for SE_CDET_0_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2164_0004
    }
}
