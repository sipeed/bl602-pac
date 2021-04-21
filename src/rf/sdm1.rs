#[doc = "Register `sdm1` reader"]
pub struct R(crate::R<SDM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SDM1_SPEC>> for R {
    fn from(reader: crate::R<SDM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdm1` writer"]
pub struct W(crate::W<SDM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDM1_SPEC>;
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
impl core::convert::From<crate::W<SDM1_SPEC>> for W {
    fn from(writer: crate::W<SDM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_flag` reader - "]
pub struct LO_SDM_FLAG_R(crate::FieldReader<bool, bool>);
impl LO_SDM_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SDM_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_flag` writer - "]
pub struct LO_SDM_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_FLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `lo_sdm_rstb_hw` reader - "]
pub struct LO_SDM_RSTB_HW_R(crate::FieldReader<bool, bool>);
impl LO_SDM_RSTB_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SDM_RSTB_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_RSTB_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_rstb_hw` writer - "]
pub struct LO_SDM_RSTB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_RSTB_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `lo_sdm_rstb` reader - "]
pub struct LO_SDM_RSTB_R(crate::FieldReader<bool, bool>);
impl LO_SDM_RSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SDM_RSTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_RSTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_rstb` writer - "]
pub struct LO_SDM_RSTB_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_RSTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `lo_sdm_bypass` reader - "]
pub struct LO_SDM_BYPASS_R(crate::FieldReader<bool, bool>);
impl LO_SDM_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SDM_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_bypass` writer - "]
pub struct LO_SDM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel` reader - "]
pub struct LO_SDM_DITHER_SEL_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel` writer - "]
pub struct LO_SDM_DITHER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_sdm_bypass_hw` reader - "]
pub struct LO_SDM_BYPASS_HW_R(crate::FieldReader<bool, bool>);
impl LO_SDM_BYPASS_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SDM_BYPASS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_BYPASS_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_bypass_hw` writer - "]
pub struct LO_SDM_BYPASS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_BYPASS_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_hw` reader - "]
pub struct LO_SDM_DITHER_SEL_HW_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_hw` writer - "]
pub struct LO_SDM_DITHER_SEL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&self) -> LO_SDM_FLAG_R {
        LO_SDM_FLAG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&self) -> LO_SDM_RSTB_HW_R {
        LO_SDM_RSTB_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&self) -> LO_SDM_RSTB_R {
        LO_SDM_RSTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&self) -> LO_SDM_BYPASS_R {
        LO_SDM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&self) -> LO_SDM_DITHER_SEL_R {
        LO_SDM_DITHER_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&self) -> LO_SDM_BYPASS_HW_R {
        LO_SDM_BYPASS_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&self) -> LO_SDM_DITHER_SEL_HW_R {
        LO_SDM_DITHER_SEL_HW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&mut self) -> LO_SDM_FLAG_W {
        LO_SDM_FLAG_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&mut self) -> LO_SDM_RSTB_HW_W {
        LO_SDM_RSTB_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&mut self) -> LO_SDM_RSTB_W {
        LO_SDM_RSTB_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&mut self) -> LO_SDM_BYPASS_W {
        LO_SDM_BYPASS_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&mut self) -> LO_SDM_DITHER_SEL_W {
        LO_SDM_DITHER_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&mut self) -> LO_SDM_BYPASS_HW_W {
        LO_SDM_BYPASS_HW_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&mut self) -> LO_SDM_DITHER_SEL_HW_W {
        LO_SDM_DITHER_SEL_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdm1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm1](index.html) module"]
pub struct SDM1_SPEC;
impl crate::RegisterSpec for SDM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdm1::R](R) reader structure"]
impl crate::Readable for SDM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdm1::W](W) writer structure"]
impl crate::Writable for SDM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sdm1 to value 0"]
impl crate::Resettable for SDM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
