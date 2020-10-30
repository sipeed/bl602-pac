#[doc = "Register `singen_ctrl4` reader"]
pub struct R(crate::R<SINGEN_CTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SINGEN_CTRL4_SPEC>> for R {
    fn from(reader: crate::R<SINGEN_CTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl4` writer"]
pub struct W(crate::W<SINGEN_CTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL4_SPEC>;
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
impl core::convert::From<crate::W<SINGEN_CTRL4_SPEC>> for W {
    fn from(writer: crate::W<SINGEN_CTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_fix_en_i` reader - "]
pub struct SINGEN_FIX_EN_I_R(crate::FieldReader<bool, bool>);
impl SINGEN_FIX_EN_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGEN_FIX_EN_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_FIX_EN_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_fix_en_i` writer - "]
pub struct SINGEN_FIX_EN_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_EN_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `singen_fix_i` reader - "]
pub struct SINGEN_FIX_I_R(crate::FieldReader<u16, u16>);
impl SINGEN_FIX_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_FIX_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_FIX_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_fix_i` writer - "]
pub struct SINGEN_FIX_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `singen_fix_en_q` reader - "]
pub struct SINGEN_FIX_EN_Q_R(crate::FieldReader<bool, bool>);
impl SINGEN_FIX_EN_Q_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGEN_FIX_EN_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_FIX_EN_Q_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_fix_en_q` writer - "]
pub struct SINGEN_FIX_EN_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_EN_Q_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `singen_fix_q` reader - "]
pub struct SINGEN_FIX_Q_R(crate::FieldReader<u16, u16>);
impl SINGEN_FIX_Q_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_FIX_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_FIX_Q_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_fix_q` writer - "]
pub struct SINGEN_FIX_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&self) -> SINGEN_FIX_EN_I_R {
        SINGEN_FIX_EN_I_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&self) -> SINGEN_FIX_I_R {
        SINGEN_FIX_I_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&self) -> SINGEN_FIX_EN_Q_R {
        SINGEN_FIX_EN_Q_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&self) -> SINGEN_FIX_Q_R {
        SINGEN_FIX_Q_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&mut self) -> SINGEN_FIX_EN_I_W {
        SINGEN_FIX_EN_I_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&mut self) -> SINGEN_FIX_I_W {
        SINGEN_FIX_I_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&mut self) -> SINGEN_FIX_EN_Q_W {
        SINGEN_FIX_EN_Q_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&mut self) -> SINGEN_FIX_Q_W {
        SINGEN_FIX_Q_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl4](index.html) module"]
pub struct SINGEN_CTRL4_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl4::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl4::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets singen_ctrl4 to value 0"]
impl crate::Resettable for SINGEN_CTRL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
