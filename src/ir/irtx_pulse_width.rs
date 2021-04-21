#[doc = "Register `irtx_pulse_width` reader"]
pub struct R(crate::R<IRTX_PULSE_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_PULSE_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRTX_PULSE_WIDTH_SPEC>> for R {
    fn from(reader: crate::R<IRTX_PULSE_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_pulse_width` writer"]
pub struct W(crate::W<IRTX_PULSE_WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_PULSE_WIDTH_SPEC>;
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
impl core::convert::From<crate::W<IRTX_PULSE_WIDTH_SPEC>> for W {
    fn from(writer: crate::W<IRTX_PULSE_WIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_mod_ph1_w` reader - "]
pub struct CR_IRTX_MOD_PH1_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_MOD_PH1_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_MOD_PH1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_MOD_PH1_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_mod_ph1_w` writer - "]
pub struct CR_IRTX_MOD_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_MOD_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `cr_irtx_mod_ph0_w` reader - "]
pub struct CR_IRTX_MOD_PH0_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_MOD_PH0_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_MOD_PH0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_MOD_PH0_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_mod_ph0_w` writer - "]
pub struct CR_IRTX_MOD_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_MOD_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cr_irtx_pw_unit` reader - "]
pub struct CR_IRTX_PW_UNIT_R(crate::FieldReader<u16, u16>);
impl CR_IRTX_PW_UNIT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_IRTX_PW_UNIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_PW_UNIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_pw_unit` writer - "]
pub struct CR_IRTX_PW_UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_PW_UNIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&self) -> CR_IRTX_MOD_PH1_W_R {
        CR_IRTX_MOD_PH1_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&self) -> CR_IRTX_MOD_PH0_W_R {
        CR_IRTX_MOD_PH0_W_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&self) -> CR_IRTX_PW_UNIT_R {
        CR_IRTX_PW_UNIT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&mut self) -> CR_IRTX_MOD_PH1_W_W {
        CR_IRTX_MOD_PH1_W_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&mut self) -> CR_IRTX_MOD_PH0_W_W {
        CR_IRTX_MOD_PH0_W_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&mut self) -> CR_IRTX_PW_UNIT_W {
        CR_IRTX_PW_UNIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_pulse_width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_pulse_width](index.html) module"]
pub struct IRTX_PULSE_WIDTH_SPEC;
impl crate::RegisterSpec for IRTX_PULSE_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_pulse_width::R](R) reader structure"]
impl crate::Readable for IRTX_PULSE_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_pulse_width::W](W) writer structure"]
impl crate::Writable for IRTX_PULSE_WIDTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irtx_pulse_width to value 0x2211_0464"]
impl crate::Resettable for IRTX_PULSE_WIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2211_0464
    }
}
