#[doc = "Register `gpdac_bctrl` reader"]
pub struct R(crate::R<GPDAC_BCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_BCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPDAC_BCTRL_SPEC>> for R {
    fn from(reader: crate::R<GPDAC_BCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_bctrl` writer"]
pub struct W(crate::W<GPDAC_BCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_BCTRL_SPEC>;
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
impl core::convert::From<crate::W<GPDAC_BCTRL_SPEC>> for W {
    fn from(writer: crate::W<GPDAC_BCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_b_outmux` reader - "]
pub struct GPDAC_B_OUTMUX_R(crate::FieldReader<u8, u8>);
impl GPDAC_B_OUTMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_B_OUTMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_B_OUTMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_b_outmux` writer - "]
pub struct GPDAC_B_OUTMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_B_OUTMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `gpdac_b_rng` reader - "]
pub struct GPDAC_B_RNG_R(crate::FieldReader<u8, u8>);
impl GPDAC_B_RNG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_B_RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_B_RNG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_b_rng` writer - "]
pub struct GPDAC_B_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_B_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `gpdac_iob_en` reader - "]
pub struct GPDAC_IOB_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_IOB_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_IOB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_IOB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_iob_en` writer - "]
pub struct GPDAC_IOB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_IOB_EN_W<'a> {
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
#[doc = "Field `gpdac_b_en` reader - "]
pub struct GPDAC_B_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_B_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_B_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_B_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_b_en` writer - "]
pub struct GPDAC_B_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_B_EN_W<'a> {
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
    pub fn gpdac_b_outmux(&self) -> GPDAC_B_OUTMUX_R {
        GPDAC_B_OUTMUX_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&self) -> GPDAC_B_RNG_R {
        GPDAC_B_RNG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&self) -> GPDAC_IOB_EN_R {
        GPDAC_IOB_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&self) -> GPDAC_B_EN_R {
        GPDAC_B_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_b_outmux(&mut self) -> GPDAC_B_OUTMUX_W {
        GPDAC_B_OUTMUX_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&mut self) -> GPDAC_B_RNG_W {
        GPDAC_B_RNG_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&mut self) -> GPDAC_IOB_EN_W {
        GPDAC_IOB_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&mut self) -> GPDAC_B_EN_W {
        GPDAC_B_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_bctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_bctrl](index.html) module"]
pub struct GPDAC_BCTRL_SPEC;
impl crate::RegisterSpec for GPDAC_BCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_bctrl::R](R) reader structure"]
impl crate::Readable for GPDAC_BCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_bctrl::W](W) writer structure"]
impl crate::Writable for GPDAC_BCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_bctrl to value 0x000c_0000"]
impl crate::Resettable for GPDAC_BCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0000
    }
}
