#[doc = "Register `gpdac_actrl` reader"]
pub struct R(crate::R<GPDAC_ACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_ACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_ACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_ACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_actrl` writer"]
pub struct W(crate::W<GPDAC_ACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_ACTRL_SPEC>;
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
impl From<crate::W<GPDAC_ACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_ACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_a_outmux` reader - "]
pub struct GPDAC_A_OUTMUX_R(crate::FieldReader<u8, u8>);
impl GPDAC_A_OUTMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_A_OUTMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_A_OUTMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_a_outmux` writer - "]
pub struct GPDAC_A_OUTMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_OUTMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `gpdac_a_rng` reader - "]
pub struct GPDAC_A_RNG_R(crate::FieldReader<u8, u8>);
impl GPDAC_A_RNG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_A_RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_A_RNG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_a_rng` writer - "]
pub struct GPDAC_A_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `gpdac_ioa_en` reader - "]
pub struct GPDAC_IOA_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_IOA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_IOA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_IOA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_ioa_en` writer - "]
pub struct GPDAC_IOA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_IOA_EN_W<'a> {
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
#[doc = "Field `gpdac_a_en` reader - "]
pub struct GPDAC_A_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_A_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_A_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_A_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_a_en` writer - "]
pub struct GPDAC_A_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_EN_W<'a> {
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
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&self) -> GPDAC_A_OUTMUX_R {
        GPDAC_A_OUTMUX_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&self) -> GPDAC_A_RNG_R {
        GPDAC_A_RNG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&self) -> GPDAC_IOA_EN_R {
        GPDAC_IOA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&self) -> GPDAC_A_EN_R {
        GPDAC_A_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&mut self) -> GPDAC_A_OUTMUX_W {
        GPDAC_A_OUTMUX_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&mut self) -> GPDAC_A_RNG_W {
        GPDAC_A_RNG_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&mut self) -> GPDAC_IOA_EN_W {
        GPDAC_IOA_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&mut self) -> GPDAC_A_EN_W {
        GPDAC_A_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_actrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_actrl](index.html) module"]
pub struct GPDAC_ACTRL_SPEC;
impl crate::RegisterSpec for GPDAC_ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_actrl::R](R) reader structure"]
impl crate::Readable for GPDAC_ACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_actrl::W](W) writer structure"]
impl crate::Writable for GPDAC_ACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_actrl to value 0x000c_0000"]
impl crate::Resettable for GPDAC_ACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0000
    }
}
