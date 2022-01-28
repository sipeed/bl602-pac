#[doc = "Register `gpdac_config` reader"]
pub struct R(crate::R<GPDAC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_config` writer"]
pub struct W(crate::W<GPDAC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_CONFIG_SPEC>;
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
impl From<crate::W<GPDAC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rsvd_31_24` reader - "]
pub struct RSVD_31_24_R(crate::FieldReader<u8, u8>);
impl RSVD_31_24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_31_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_31_24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_31_24` writer - "]
pub struct RSVD_31_24_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `gpdac_ch_b_sel` reader - "]
pub struct GPDAC_CH_B_SEL_R(crate::FieldReader<u8, u8>);
impl GPDAC_CH_B_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_CH_B_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_CH_B_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_ch_b_sel` writer - "]
pub struct GPDAC_CH_B_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_CH_B_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `gpdac_ch_a_sel` reader - "]
pub struct GPDAC_CH_A_SEL_R(crate::FieldReader<u8, u8>);
impl GPDAC_CH_A_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_CH_A_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_CH_A_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_ch_a_sel` writer - "]
pub struct GPDAC_CH_A_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_CH_A_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `gpdac_mode` reader - "]
pub struct GPDAC_MODE_R(crate::FieldReader<u8, u8>);
impl GPDAC_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_mode` writer - "]
pub struct GPDAC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `dsm_mode` reader - "]
pub struct DSM_MODE_R(crate::FieldReader<u8, u8>);
impl DSM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSM_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSM_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dsm_mode` writer - "]
pub struct DSM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `gpdac_en2` reader - "]
pub struct GPDAC_EN2_R(crate::FieldReader<bool, bool>);
impl GPDAC_EN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_EN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_EN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_en2` writer - "]
pub struct GPDAC_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_EN2_W<'a> {
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
#[doc = "Field `gpdac_en` reader - "]
pub struct GPDAC_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_en` writer - "]
pub struct GPDAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_EN_W<'a> {
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
    pub fn rsvd_31_24(&self) -> RSVD_31_24_R {
        RSVD_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&self) -> GPDAC_CH_B_SEL_R {
        GPDAC_CH_B_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&self) -> GPDAC_CH_A_SEL_R {
        GPDAC_CH_A_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&self) -> GPDAC_MODE_R {
        GPDAC_MODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&self) -> DSM_MODE_R {
        DSM_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&self) -> GPDAC_EN2_R {
        GPDAC_EN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&self) -> GPDAC_EN_R {
        GPDAC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&mut self) -> RSVD_31_24_W {
        RSVD_31_24_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&mut self) -> GPDAC_CH_B_SEL_W {
        GPDAC_CH_B_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&mut self) -> GPDAC_CH_A_SEL_W {
        GPDAC_CH_A_SEL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&mut self) -> GPDAC_MODE_W {
        GPDAC_MODE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&mut self) -> DSM_MODE_W {
        DSM_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&mut self) -> GPDAC_EN2_W {
        GPDAC_EN2_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&mut self) -> GPDAC_EN_W {
        GPDAC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_config](index.html) module"]
pub struct GPDAC_CONFIG_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_config::R](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_config::W](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_config to value 0"]
impl crate::Resettable for GPDAC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
