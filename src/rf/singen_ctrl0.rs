#[doc = "Register `singen_ctrl0` reader"]
pub struct R(crate::R<SINGEN_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SINGEN_CTRL0_SPEC>> for R {
    fn from(reader: crate::R<SINGEN_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl0` writer"]
pub struct W(crate::W<SINGEN_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL0_SPEC>;
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
impl core::convert::From<crate::W<SINGEN_CTRL0_SPEC>> for W {
    fn from(writer: crate::W<SINGEN_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_en` reader - "]
pub struct SINGEN_EN_R(crate::FieldReader<bool, bool>);
impl SINGEN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_en` writer - "]
pub struct SINGEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `singen_clkdiv_n` reader - "]
pub struct SINGEN_CLKDIV_N_R(crate::FieldReader<u8, u8>);
impl SINGEN_CLKDIV_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINGEN_CLKDIV_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_CLKDIV_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_clkdiv_n` writer - "]
pub struct SINGEN_CLKDIV_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_CLKDIV_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `singen_unsign_en` reader - "]
pub struct SINGEN_UNSIGN_EN_R(crate::FieldReader<bool, bool>);
impl SINGEN_UNSIGN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGEN_UNSIGN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_UNSIGN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_unsign_en` writer - "]
pub struct SINGEN_UNSIGN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_UNSIGN_EN_W<'a> {
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
#[doc = "Field `singen_inc_step0` reader - "]
pub struct SINGEN_INC_STEP0_R(crate::FieldReader<u16, u16>);
impl SINGEN_INC_STEP0_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_INC_STEP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_INC_STEP0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_inc_step0` writer - "]
pub struct SINGEN_INC_STEP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_INC_STEP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `singen_inc_step1` reader - "]
pub struct SINGEN_INC_STEP1_R(crate::FieldReader<u16, u16>);
impl SINGEN_INC_STEP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_INC_STEP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_INC_STEP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_inc_step1` writer - "]
pub struct SINGEN_INC_STEP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_INC_STEP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&self) -> SINGEN_EN_R {
        SINGEN_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&self) -> SINGEN_CLKDIV_N_R {
        SINGEN_CLKDIV_N_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&self) -> SINGEN_UNSIGN_EN_R {
        SINGEN_UNSIGN_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&self) -> SINGEN_INC_STEP0_R {
        SINGEN_INC_STEP0_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&self) -> SINGEN_INC_STEP1_R {
        SINGEN_INC_STEP1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&mut self) -> SINGEN_EN_W {
        SINGEN_EN_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&mut self) -> SINGEN_CLKDIV_N_W {
        SINGEN_CLKDIV_N_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&mut self) -> SINGEN_UNSIGN_EN_W {
        SINGEN_UNSIGN_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&mut self) -> SINGEN_INC_STEP0_W {
        SINGEN_INC_STEP0_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&mut self) -> SINGEN_INC_STEP1_W {
        SINGEN_INC_STEP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl0](index.html) module"]
pub struct SINGEN_CTRL0_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl0::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl0::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets singen_ctrl0 to value 0"]
impl crate::Resettable for SINGEN_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
