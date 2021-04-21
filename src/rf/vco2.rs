#[doc = "Register `vco2` reader"]
pub struct R(crate::R<VCO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VCO2_SPEC>> for R {
    fn from(reader: crate::R<VCO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco2` writer"]
pub struct W(crate::W<VCO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO2_SPEC>;
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
impl core::convert::From<crate::W<VCO2_SPEC>> for W {
    fn from(writer: crate::W<VCO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `acal_inc_en_hw` reader - "]
pub struct ACAL_INC_EN_HW_R(crate::FieldReader<bool, bool>);
impl ACAL_INC_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_INC_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_INC_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_inc_en_hw` writer - "]
pub struct ACAL_INC_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_INC_EN_HW_W<'a> {
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
#[doc = "Field `acal_vco_ud` reader - "]
pub struct ACAL_VCO_UD_R(crate::FieldReader<bool, bool>);
impl ACAL_VCO_UD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_VCO_UD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_VCO_UD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_vco_ud` writer - "]
pub struct ACAL_VCO_UD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_VCO_UD_W<'a> {
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
#[doc = "Field `acal_vref_cw` reader - "]
pub struct ACAL_VREF_CW_R(crate::FieldReader<u8, u8>);
impl ACAL_VREF_CW_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACAL_VREF_CW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_VREF_CW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_vref_cw` writer - "]
pub struct ACAL_VREF_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_VREF_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `lo_vco_short_idac_filter` reader - "]
pub struct LO_VCO_SHORT_IDAC_FILTER_R(crate::FieldReader<bool, bool>);
impl LO_VCO_SHORT_IDAC_FILTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_VCO_SHORT_IDAC_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_SHORT_IDAC_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_short_idac_filter` writer - "]
pub struct LO_VCO_SHORT_IDAC_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_SHORT_IDAC_FILTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `lo_vco_short_vbias_filter` reader - "]
pub struct LO_VCO_SHORT_VBIAS_FILTER_R(crate::FieldReader<bool, bool>);
impl LO_VCO_SHORT_VBIAS_FILTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_VCO_SHORT_VBIAS_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_SHORT_VBIAS_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_short_vbias_filter` writer - "]
pub struct LO_VCO_SHORT_VBIAS_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_SHORT_VBIAS_FILTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `lo_vco_idac_boot` reader - "]
pub struct LO_VCO_IDAC_BOOT_R(crate::FieldReader<bool, bool>);
impl LO_VCO_IDAC_BOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_VCO_IDAC_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_boot` writer - "]
pub struct LO_VCO_IDAC_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_BOOT_W<'a> {
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
#[doc = "Field `lo_vco_vbias_cw` reader - "]
pub struct LO_VCO_VBIAS_CW_R(crate::FieldReader<u8, u8>);
impl LO_VCO_VBIAS_CW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_VBIAS_CW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_VBIAS_CW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_vbias_cw` writer - "]
pub struct LO_VCO_VBIAS_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_VBIAS_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&self) -> ACAL_INC_EN_HW_R {
        ACAL_INC_EN_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&self) -> ACAL_VCO_UD_R {
        ACAL_VCO_UD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&self) -> ACAL_VREF_CW_R {
        ACAL_VREF_CW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&self) -> LO_VCO_SHORT_IDAC_FILTER_R {
        LO_VCO_SHORT_IDAC_FILTER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&self) -> LO_VCO_SHORT_VBIAS_FILTER_R {
        LO_VCO_SHORT_VBIAS_FILTER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&self) -> LO_VCO_IDAC_BOOT_R {
        LO_VCO_IDAC_BOOT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&self) -> LO_VCO_VBIAS_CW_R {
        LO_VCO_VBIAS_CW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&mut self) -> ACAL_INC_EN_HW_W {
        ACAL_INC_EN_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&mut self) -> ACAL_VCO_UD_W {
        ACAL_VCO_UD_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&mut self) -> ACAL_VREF_CW_W {
        ACAL_VREF_CW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&mut self) -> LO_VCO_SHORT_IDAC_FILTER_W {
        LO_VCO_SHORT_IDAC_FILTER_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&mut self) -> LO_VCO_SHORT_VBIAS_FILTER_W {
        LO_VCO_SHORT_VBIAS_FILTER_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&mut self) -> LO_VCO_IDAC_BOOT_W {
        LO_VCO_IDAC_BOOT_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&mut self) -> LO_VCO_VBIAS_CW_W {
        LO_VCO_VBIAS_CW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco2](index.html) module"]
pub struct VCO2_SPEC;
impl crate::RegisterSpec for VCO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco2::R](R) reader structure"]
impl crate::Readable for VCO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco2::W](W) writer structure"]
impl crate::Writable for VCO2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vco2 to value 0"]
impl crate::Resettable for VCO2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
