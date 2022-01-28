#[doc = "Register `singen_ctrl1` reader"]
pub struct R(crate::R<SINGEN_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGEN_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGEN_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl1` writer"]
pub struct W(crate::W<SINGEN_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL1_SPEC>;
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
impl From<crate::W<SINGEN_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGEN_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_mode_i` reader - "]
pub struct SINGEN_MODE_I_R(crate::FieldReader<u8, u8>);
impl SINGEN_MODE_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINGEN_MODE_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_MODE_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_mode_i` writer - "]
pub struct SINGEN_MODE_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_MODE_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `singen_clkdiv_i` reader - "]
pub struct SINGEN_CLKDIV_I_R(crate::FieldReader<u16, u16>);
impl SINGEN_CLKDIV_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_CLKDIV_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_CLKDIV_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_clkdiv_i` writer - "]
pub struct SINGEN_CLKDIV_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_CLKDIV_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `singen_mode_q` reader - "]
pub struct SINGEN_MODE_Q_R(crate::FieldReader<u8, u8>);
impl SINGEN_MODE_Q_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINGEN_MODE_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_MODE_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_mode_q` writer - "]
pub struct SINGEN_MODE_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_MODE_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `singen_clkdiv_q` reader - "]
pub struct SINGEN_CLKDIV_Q_R(crate::FieldReader<u16, u16>);
impl SINGEN_CLKDIV_Q_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_CLKDIV_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_CLKDIV_Q_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_clkdiv_q` writer - "]
pub struct SINGEN_CLKDIV_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_CLKDIV_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&self) -> SINGEN_MODE_I_R {
        SINGEN_MODE_I_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&self) -> SINGEN_CLKDIV_I_R {
        SINGEN_CLKDIV_I_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&self) -> SINGEN_MODE_Q_R {
        SINGEN_MODE_Q_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&self) -> SINGEN_CLKDIV_Q_R {
        SINGEN_CLKDIV_Q_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&mut self) -> SINGEN_MODE_I_W {
        SINGEN_MODE_I_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&mut self) -> SINGEN_CLKDIV_I_W {
        SINGEN_CLKDIV_I_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&mut self) -> SINGEN_MODE_Q_W {
        SINGEN_MODE_Q_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&mut self) -> SINGEN_CLKDIV_Q_W {
        SINGEN_CLKDIV_Q_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl1](index.html) module"]
pub struct SINGEN_CTRL1_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl1::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl1::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets singen_ctrl1 to value 0"]
impl crate::Resettable for SINGEN_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
