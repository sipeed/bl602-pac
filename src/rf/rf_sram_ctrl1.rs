#[doc = "Register `rf_sram_ctrl1` reader"]
pub struct R(crate::R<RF_SRAM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SRAM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_SRAM_CTRL1_SPEC>> for R {
    fn from(reader: crate::R<RF_SRAM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_sram_ctrl1` writer"]
pub struct W(crate::W<RF_SRAM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SRAM_CTRL1_SPEC>;
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
impl core::convert::From<crate::W<RF_SRAM_CTRL1_SPEC>> for W {
    fn from(writer: crate::W<RF_SRAM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_sram_adc_done_cnt` reader - "]
pub struct RF_SRAM_ADC_DONE_CNT_R(crate::FieldReader<u16, u16>);
impl RF_SRAM_ADC_DONE_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_SRAM_ADC_DONE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SRAM_ADC_DONE_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_sram_adc_done_cnt` writer - "]
pub struct RF_SRAM_ADC_DONE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_DONE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `rf_sram_adc_sts_clr` reader - "]
pub struct RF_SRAM_ADC_STS_CLR_R(crate::FieldReader<bool, bool>);
impl RF_SRAM_ADC_STS_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_SRAM_ADC_STS_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SRAM_ADC_STS_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_sram_adc_sts_clr` writer - "]
pub struct RF_SRAM_ADC_STS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_STS_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `rf_sram_adc_loop_en` reader - "]
pub struct RF_SRAM_ADC_LOOP_EN_R(crate::FieldReader<bool, bool>);
impl RF_SRAM_ADC_LOOP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_SRAM_ADC_LOOP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SRAM_ADC_LOOP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_sram_adc_loop_en` writer - "]
pub struct RF_SRAM_ADC_LOOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_LOOP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `rf_sram_adc_en` reader - "]
pub struct RF_SRAM_ADC_EN_R(crate::FieldReader<bool, bool>);
impl RF_SRAM_ADC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_SRAM_ADC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SRAM_ADC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_sram_adc_en` writer - "]
pub struct RF_SRAM_ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `rf_sram_adc_done` reader - "]
pub struct RF_SRAM_ADC_DONE_R(crate::FieldReader<bool, bool>);
impl RF_SRAM_ADC_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_SRAM_ADC_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SRAM_ADC_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_sram_adc_done` writer - "]
pub struct RF_SRAM_ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&self) -> RF_SRAM_ADC_DONE_CNT_R {
        RF_SRAM_ADC_DONE_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&self) -> RF_SRAM_ADC_STS_CLR_R {
        RF_SRAM_ADC_STS_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&self) -> RF_SRAM_ADC_LOOP_EN_R {
        RF_SRAM_ADC_LOOP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&self) -> RF_SRAM_ADC_EN_R {
        RF_SRAM_ADC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&self) -> RF_SRAM_ADC_DONE_R {
        RF_SRAM_ADC_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&mut self) -> RF_SRAM_ADC_DONE_CNT_W {
        RF_SRAM_ADC_DONE_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&mut self) -> RF_SRAM_ADC_STS_CLR_W {
        RF_SRAM_ADC_STS_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&mut self) -> RF_SRAM_ADC_LOOP_EN_W {
        RF_SRAM_ADC_LOOP_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&mut self) -> RF_SRAM_ADC_EN_W {
        RF_SRAM_ADC_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&mut self) -> RF_SRAM_ADC_DONE_W {
        RF_SRAM_ADC_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_sram_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl1](index.html) module"]
pub struct RF_SRAM_CTRL1_SPEC;
impl crate::RegisterSpec for RF_SRAM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_sram_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_sram_ctrl1 to value 0"]
impl crate::Resettable for RF_SRAM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
