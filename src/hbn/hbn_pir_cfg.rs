#[doc = "Register `HBN_PIR_CFG` reader"]
pub struct R(crate::R<HBN_PIR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_PIR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_PIR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_PIR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_PIR_CFG` writer"]
pub struct W(crate::W<HBN_PIR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_PIR_CFG_SPEC>;
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
impl From<crate::W<HBN_PIR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_PIR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_nosync` reader - "]
pub struct GPADC_NOSYNC_R(crate::FieldReader<bool, bool>);
impl GPADC_NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_NOSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_NOSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_nosync` writer - "]
pub struct GPADC_NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NOSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `gpadc_cgen` reader - "]
pub struct GPADC_CGEN_R(crate::FieldReader<bool, bool>);
impl GPADC_CGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_CGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_cgen` writer - "]
pub struct GPADC_CGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `pir_en` reader - "]
pub struct PIR_EN_R(crate::FieldReader<bool, bool>);
impl PIR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pir_en` writer - "]
pub struct PIR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `pir_dis` reader - "]
pub struct PIR_DIS_R(crate::FieldReader<u8, u8>);
impl PIR_DIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIR_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIR_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pir_dis` writer - "]
pub struct PIR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `pir_lpf_sel` reader - "]
pub struct PIR_LPF_SEL_R(crate::FieldReader<bool, bool>);
impl PIR_LPF_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIR_LPF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIR_LPF_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pir_lpf_sel` writer - "]
pub struct PIR_LPF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_LPF_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `pir_hpf_sel` reader - "]
pub struct PIR_HPF_SEL_R(crate::FieldReader<u8, u8>);
impl PIR_HPF_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIR_HPF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIR_HPF_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pir_hpf_sel` writer - "]
pub struct PIR_HPF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_HPF_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_nosync(&self) -> GPADC_NOSYNC_R {
        GPADC_NOSYNC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cgen(&self) -> GPADC_CGEN_R {
        GPADC_CGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&self) -> PIR_EN_R {
        PIR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&self) -> PIR_DIS_R {
        PIR_DIS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&self) -> PIR_LPF_SEL_R {
        PIR_LPF_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&self) -> PIR_HPF_SEL_R {
        PIR_HPF_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_nosync(&mut self) -> GPADC_NOSYNC_W {
        GPADC_NOSYNC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cgen(&mut self) -> GPADC_CGEN_W {
        GPADC_CGEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&mut self) -> PIR_EN_W {
        PIR_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&mut self) -> PIR_DIS_W {
        PIR_DIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&mut self) -> PIR_LPF_SEL_W {
        PIR_LPF_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&mut self) -> PIR_HPF_SEL_W {
        PIR_HPF_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_PIR_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_cfg](index.html) module"]
pub struct HBN_PIR_CFG_SPEC;
impl crate::RegisterSpec for HBN_PIR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_pir_cfg::R](R) reader structure"]
impl crate::Readable for HBN_PIR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_pir_cfg::W](W) writer structure"]
impl crate::Writable for HBN_PIR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_PIR_CFG to value 0"]
impl crate::Resettable for HBN_PIR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
