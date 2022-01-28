#[doc = "Register `adda2` reader"]
pub struct R(crate::R<ADDA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adda2` writer"]
pub struct W(crate::W<ADDA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDA2_SPEC>;
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
impl From<crate::W<ADDA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_clk_div_sel` reader - "]
pub struct ADC_CLK_DIV_SEL_R(crate::FieldReader<bool, bool>);
impl ADC_CLK_DIV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLK_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_DIV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_clk_div_sel` writer - "]
pub struct ADC_CLK_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_DIV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `adc_clk_inv` reader - "]
pub struct ADC_CLK_INV_R(crate::FieldReader<bool, bool>);
impl ADC_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_clk_inv` writer - "]
pub struct ADC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `adc_clk_sync_inv` reader - "]
pub struct ADC_CLK_SYNC_INV_R(crate::FieldReader<bool, bool>);
impl ADC_CLK_SYNC_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLK_SYNC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_SYNC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_clk_sync_inv` writer - "]
pub struct ADC_CLK_SYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_SYNC_INV_W<'a> {
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
#[doc = "Field `adc_gt_rm` reader - "]
pub struct ADC_GT_RM_R(crate::FieldReader<bool, bool>);
impl ADC_GT_RM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_GT_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_GT_RM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_gt_rm` writer - "]
pub struct ADC_GT_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_GT_RM_W<'a> {
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
#[doc = "Field `adc_sar_ascal_en` reader - "]
pub struct ADC_SAR_ASCAL_EN_R(crate::FieldReader<bool, bool>);
impl ADC_SAR_ASCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_SAR_ASCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_SAR_ASCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_sar_ascal_en` writer - "]
pub struct ADC_SAR_ASCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SAR_ASCAL_EN_W<'a> {
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
#[doc = "Field `adc_dvdd_sel` reader - "]
pub struct ADC_DVDD_SEL_R(crate::FieldReader<u8, u8>);
impl ADC_DVDD_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_DVDD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DVDD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_dvdd_sel` writer - "]
pub struct ADC_DVDD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DVDD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `adc_dly_ctl` reader - "]
pub struct ADC_DLY_CTL_R(crate::FieldReader<u8, u8>);
impl ADC_DLY_CTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_DLY_CTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DLY_CTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_dly_ctl` writer - "]
pub struct ADC_DLY_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DLY_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `adc_vref_sel` reader - "]
pub struct ADC_VREF_SEL_R(crate::FieldReader<u8, u8>);
impl ADC_VREF_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_VREF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_VREF_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_vref_sel` writer - "]
pub struct ADC_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VREF_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&self) -> ADC_CLK_DIV_SEL_R {
        ADC_CLK_DIV_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&self) -> ADC_CLK_INV_R {
        ADC_CLK_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&self) -> ADC_CLK_SYNC_INV_R {
        ADC_CLK_SYNC_INV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&self) -> ADC_GT_RM_R {
        ADC_GT_RM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&self) -> ADC_SAR_ASCAL_EN_R {
        ADC_SAR_ASCAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&self) -> ADC_DVDD_SEL_R {
        ADC_DVDD_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&self) -> ADC_DLY_CTL_R {
        ADC_DLY_CTL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&self) -> ADC_VREF_SEL_R {
        ADC_VREF_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&mut self) -> ADC_CLK_DIV_SEL_W {
        ADC_CLK_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&mut self) -> ADC_CLK_INV_W {
        ADC_CLK_INV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&mut self) -> ADC_CLK_SYNC_INV_W {
        ADC_CLK_SYNC_INV_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&mut self) -> ADC_GT_RM_W {
        ADC_GT_RM_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&mut self) -> ADC_SAR_ASCAL_EN_W {
        ADC_SAR_ASCAL_EN_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&mut self) -> ADC_DVDD_SEL_W {
        ADC_DVDD_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&mut self) -> ADC_DLY_CTL_W {
        ADC_DLY_CTL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&mut self) -> ADC_VREF_SEL_W {
        ADC_VREF_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adda2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda2](index.html) module"]
pub struct ADDA2_SPEC;
impl crate::RegisterSpec for ADDA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adda2::R](R) reader structure"]
impl crate::Readable for ADDA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adda2::W](W) writer structure"]
impl crate::Writable for ADDA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adda2 to value 0"]
impl crate::Resettable for ADDA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
