#[doc = "Register `lo_cal_ctrl_hw7` reader"]
pub struct R(crate::R<LO_CAL_CTRL_HW7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_CAL_CTRL_HW7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LO_CAL_CTRL_HW7_SPEC>> for R {
    fn from(reader: crate::R<LO_CAL_CTRL_HW7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_cal_ctrl_hw7` writer"]
pub struct W(crate::W<LO_CAL_CTRL_HW7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_CAL_CTRL_HW7_SPEC>;
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
impl core::convert::From<crate::W<LO_CAL_CTRL_HW7_SPEC>> for W {
    fn from(writer: crate::W<LO_CAL_CTRL_HW7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_vco_freq_cw_2456` reader - "]
pub struct LO_VCO_FREQ_CW_2456_R(crate::FieldReader<u8, u8>);
impl LO_VCO_FREQ_CW_2456_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_FREQ_CW_2456_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_FREQ_CW_2456_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_freq_cw_2456` writer - "]
pub struct LO_VCO_FREQ_CW_2456_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_2456_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `lo_vco_idac_cw_2456` reader - "]
pub struct LO_VCO_IDAC_CW_2456_R(crate::FieldReader<u8, u8>);
impl LO_VCO_IDAC_CW_2456_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_IDAC_CW_2456_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_CW_2456_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_cw_2456` writer - "]
pub struct LO_VCO_IDAC_CW_2456_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_2456_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `lo_vco_freq_cw_2452` reader - "]
pub struct LO_VCO_FREQ_CW_2452_R(crate::FieldReader<u8, u8>);
impl LO_VCO_FREQ_CW_2452_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_FREQ_CW_2452_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_FREQ_CW_2452_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_freq_cw_2452` writer - "]
pub struct LO_VCO_FREQ_CW_2452_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_2452_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `lo_vco_idac_cw_2452` reader - "]
pub struct LO_VCO_IDAC_CW_2452_R(crate::FieldReader<u8, u8>);
impl LO_VCO_IDAC_CW_2452_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_IDAC_CW_2452_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_CW_2452_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_cw_2452` writer - "]
pub struct LO_VCO_IDAC_CW_2452_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_2452_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2456(&self) -> LO_VCO_FREQ_CW_2456_R {
        LO_VCO_FREQ_CW_2456_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2456(&self) -> LO_VCO_IDAC_CW_2456_R {
        LO_VCO_IDAC_CW_2456_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2452(&self) -> LO_VCO_FREQ_CW_2452_R {
        LO_VCO_FREQ_CW_2452_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2452(&self) -> LO_VCO_IDAC_CW_2452_R {
        LO_VCO_IDAC_CW_2452_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2456(&mut self) -> LO_VCO_FREQ_CW_2456_W {
        LO_VCO_FREQ_CW_2456_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2456(&mut self) -> LO_VCO_IDAC_CW_2456_W {
        LO_VCO_IDAC_CW_2456_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2452(&mut self) -> LO_VCO_FREQ_CW_2452_W {
        LO_VCO_FREQ_CW_2452_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2452(&mut self) -> LO_VCO_IDAC_CW_2452_W {
        LO_VCO_IDAC_CW_2452_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_cal_ctrl_hw7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw7](index.html) module"]
pub struct LO_CAL_CTRL_HW7_SPEC;
impl crate::RegisterSpec for LO_CAL_CTRL_HW7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_cal_ctrl_hw7::R](R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw7::W](W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw7 to value 0"]
impl crate::Resettable for LO_CAL_CTRL_HW7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
