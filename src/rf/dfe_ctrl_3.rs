#[doc = "Register `dfe_ctrl_3` reader"]
pub struct R(crate::R<DFE_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_3_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_3` writer"]
pub struct W(crate::W<DFE_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_3_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_3_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_adc_4s_q_en` reader - "]
pub struct RX_ADC_4S_Q_EN_R(crate::FieldReader<bool, bool>);
impl RX_ADC_4S_Q_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ADC_4S_Q_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ADC_4S_Q_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_adc_4s_q_en` writer - "]
pub struct RX_ADC_4S_Q_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_Q_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `rx_adc_4s_q_val` reader - "]
pub struct RX_ADC_4S_Q_VAL_R(crate::FieldReader<u16, u16>);
impl RX_ADC_4S_Q_VAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_ADC_4S_Q_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ADC_4S_Q_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_adc_4s_q_val` writer - "]
pub struct RX_ADC_4S_Q_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_Q_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `rx_adc_4s_i_en` reader - "]
pub struct RX_ADC_4S_I_EN_R(crate::FieldReader<bool, bool>);
impl RX_ADC_4S_I_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ADC_4S_I_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ADC_4S_I_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_adc_4s_i_en` writer - "]
pub struct RX_ADC_4S_I_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_I_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `rx_adc_4s_i_val` reader - "]
pub struct RX_ADC_4S_I_VAL_R(crate::FieldReader<u16, u16>);
impl RX_ADC_4S_I_VAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_ADC_4S_I_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ADC_4S_I_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_adc_4s_i_val` writer - "]
pub struct RX_ADC_4S_I_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_I_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&self) -> RX_ADC_4S_Q_EN_R {
        RX_ADC_4S_Q_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&self) -> RX_ADC_4S_Q_VAL_R {
        RX_ADC_4S_Q_VAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&self) -> RX_ADC_4S_I_EN_R {
        RX_ADC_4S_I_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&self) -> RX_ADC_4S_I_VAL_R {
        RX_ADC_4S_I_VAL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&mut self) -> RX_ADC_4S_Q_EN_W {
        RX_ADC_4S_Q_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&mut self) -> RX_ADC_4S_Q_VAL_W {
        RX_ADC_4S_Q_VAL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&mut self) -> RX_ADC_4S_I_EN_W {
        RX_ADC_4S_I_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&mut self) -> RX_ADC_4S_I_VAL_W {
        RX_ADC_4S_I_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_3](index.html) module"]
pub struct DFE_CTRL_3_SPEC;
impl crate::RegisterSpec for DFE_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_3::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_3::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_3 to value 0"]
impl crate::Resettable for DFE_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
