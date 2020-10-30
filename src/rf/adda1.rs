#[doc = "Register `adda1` reader"]
pub struct R(crate::R<ADDA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDA1_SPEC>> for R {
    fn from(reader: crate::R<ADDA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adda1` writer"]
pub struct W(crate::W<ADDA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDA1_SPEC>;
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
impl core::convert::From<crate::W<ADDA1_SPEC>> for W {
    fn from(writer: crate::W<ADDA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adda_ldo_dvdd_sel_hw` reader - "]
pub struct ADDA_LDO_DVDD_SEL_HW_R(crate::FieldReader<u8, u8>);
impl ADDA_LDO_DVDD_SEL_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDA_LDO_DVDD_SEL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDA_LDO_DVDD_SEL_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adda_ldo_dvdd_sel_hw` writer - "]
pub struct ADDA_LDO_DVDD_SEL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `adda_ldo_dvdd_sel` reader - "]
pub struct ADDA_LDO_DVDD_SEL_R(crate::FieldReader<u8, u8>);
impl ADDA_LDO_DVDD_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDA_LDO_DVDD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDA_LDO_DVDD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adda_ldo_dvdd_sel` writer - "]
pub struct ADDA_LDO_DVDD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `adda_ldo_byps` reader - "]
pub struct ADDA_LDO_BYPS_R(crate::FieldReader<bool, bool>);
impl ADDA_LDO_BYPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDA_LDO_BYPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDA_LDO_BYPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adda_ldo_byps` writer - "]
pub struct ADDA_LDO_BYPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_BYPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `dac_clk_sync_inv` reader - "]
pub struct DAC_CLK_SYNC_INV_R(crate::FieldReader<bool, bool>);
impl DAC_CLK_SYNC_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CLK_SYNC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_SYNC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dac_clk_sync_inv` writer - "]
pub struct DAC_CLK_SYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_SYNC_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `dac_rccalsel` reader - "]
pub struct DAC_RCCALSEL_R(crate::FieldReader<bool, bool>);
impl DAC_RCCALSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC_RCCALSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_RCCALSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dac_rccalsel` writer - "]
pub struct DAC_RCCALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_RCCALSEL_W<'a> {
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
#[doc = "Field `dac_clk_sel` reader - "]
pub struct DAC_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl DAC_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dac_clk_sel` writer - "]
pub struct DAC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `dac_bias_sel` reader - "]
pub struct DAC_BIAS_SEL_R(crate::FieldReader<u8, u8>);
impl DAC_BIAS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_BIAS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_BIAS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dac_bias_sel` writer - "]
pub struct DAC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_BIAS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `dac_dvdd_sel` reader - "]
pub struct DAC_DVDD_SEL_R(crate::FieldReader<u8, u8>);
impl DAC_DVDD_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_DVDD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DVDD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dac_dvdd_sel` writer - "]
pub struct DAC_DVDD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DVDD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&self) -> ADDA_LDO_DVDD_SEL_HW_R {
        ADDA_LDO_DVDD_SEL_HW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&self) -> ADDA_LDO_DVDD_SEL_R {
        ADDA_LDO_DVDD_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&self) -> ADDA_LDO_BYPS_R {
        ADDA_LDO_BYPS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&self) -> DAC_CLK_SYNC_INV_R {
        DAC_CLK_SYNC_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&self) -> DAC_RCCALSEL_R {
        DAC_RCCALSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&self) -> DAC_CLK_SEL_R {
        DAC_CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&self) -> DAC_BIAS_SEL_R {
        DAC_BIAS_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&self) -> DAC_DVDD_SEL_R {
        DAC_DVDD_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&mut self) -> ADDA_LDO_DVDD_SEL_HW_W {
        ADDA_LDO_DVDD_SEL_HW_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&mut self) -> ADDA_LDO_DVDD_SEL_W {
        ADDA_LDO_DVDD_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&mut self) -> ADDA_LDO_BYPS_W {
        ADDA_LDO_BYPS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&mut self) -> DAC_CLK_SYNC_INV_W {
        DAC_CLK_SYNC_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&mut self) -> DAC_RCCALSEL_W {
        DAC_RCCALSEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&mut self) -> DAC_CLK_SEL_W {
        DAC_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&mut self) -> DAC_BIAS_SEL_W {
        DAC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&mut self) -> DAC_DVDD_SEL_W {
        DAC_DVDD_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adda1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda1](index.html) module"]
pub struct ADDA1_SPEC;
impl crate::RegisterSpec for ADDA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adda1::R](R) reader structure"]
impl crate::Readable for ADDA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adda1::W](W) writer structure"]
impl crate::Writable for ADDA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adda1 to value 0"]
impl crate::Resettable for ADDA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
