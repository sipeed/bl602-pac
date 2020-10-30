#[doc = "Register `dfe_ctrl_11` reader"]
pub struct R(crate::R<DFE_CTRL_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_11_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_11` writer"]
pub struct W(crate::W<DFE_CTRL_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_11_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_11_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dfe_adc_raw_q` reader - "]
pub struct DFE_ADC_RAW_Q_R(crate::FieldReader<u16, u16>);
impl DFE_ADC_RAW_Q_R {
    pub(crate) fn new(bits: u16) -> Self {
        DFE_ADC_RAW_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFE_ADC_RAW_Q_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dfe_adc_raw_q` writer - "]
pub struct DFE_ADC_RAW_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE_ADC_RAW_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `dfe_adc_raw_i` reader - "]
pub struct DFE_ADC_RAW_I_R(crate::FieldReader<u16, u16>);
impl DFE_ADC_RAW_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        DFE_ADC_RAW_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFE_ADC_RAW_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dfe_adc_raw_i` writer - "]
pub struct DFE_ADC_RAW_I_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE_ADC_RAW_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn dfe_adc_raw_q(&self) -> DFE_ADC_RAW_Q_R {
        DFE_ADC_RAW_Q_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn dfe_adc_raw_i(&self) -> DFE_ADC_RAW_I_R {
        DFE_ADC_RAW_I_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn dfe_adc_raw_q(&mut self) -> DFE_ADC_RAW_Q_W {
        DFE_ADC_RAW_Q_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn dfe_adc_raw_i(&mut self) -> DFE_ADC_RAW_I_W {
        DFE_ADC_RAW_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_11](index.html) module"]
pub struct DFE_CTRL_11_SPEC;
impl crate::RegisterSpec for DFE_CTRL_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_11::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_11::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_11 to value 0"]
impl crate::Resettable for DFE_CTRL_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
