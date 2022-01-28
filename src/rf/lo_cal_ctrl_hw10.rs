#[doc = "Register `lo_cal_ctrl_hw10` reader"]
pub struct R(crate::R<LO_CAL_CTRL_HW10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_CAL_CTRL_HW10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_CAL_CTRL_HW10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_CAL_CTRL_HW10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_cal_ctrl_hw10` writer"]
pub struct W(crate::W<LO_CAL_CTRL_HW10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_CAL_CTRL_HW10_SPEC>;
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
impl From<crate::W<LO_CAL_CTRL_HW10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_CAL_CTRL_HW10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_vco_freq_cw_2480` reader - "]
pub struct LO_VCO_FREQ_CW_2480_R(crate::FieldReader<u8, u8>);
impl LO_VCO_FREQ_CW_2480_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_FREQ_CW_2480_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_FREQ_CW_2480_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_freq_cw_2480` writer - "]
pub struct LO_VCO_FREQ_CW_2480_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_2480_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `lo_vco_idac_cw_2480` reader - "]
pub struct LO_VCO_IDAC_CW_2480_R(crate::FieldReader<u8, u8>);
impl LO_VCO_IDAC_CW_2480_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_IDAC_CW_2480_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_CW_2480_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_cw_2480` writer - "]
pub struct LO_VCO_IDAC_CW_2480_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_2480_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `lo_vco_freq_cw_2476` reader - "]
pub struct LO_VCO_FREQ_CW_2476_R(crate::FieldReader<u8, u8>);
impl LO_VCO_FREQ_CW_2476_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_FREQ_CW_2476_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_FREQ_CW_2476_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_freq_cw_2476` writer - "]
pub struct LO_VCO_FREQ_CW_2476_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_2476_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `lo_vco_idac_cw_2476` reader - "]
pub struct LO_VCO_IDAC_CW_2476_R(crate::FieldReader<u8, u8>);
impl LO_VCO_IDAC_CW_2476_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_IDAC_CW_2476_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_CW_2476_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_cw_2476` writer - "]
pub struct LO_VCO_IDAC_CW_2476_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_2476_W<'a> {
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
    pub fn lo_vco_freq_cw_2480(&self) -> LO_VCO_FREQ_CW_2480_R {
        LO_VCO_FREQ_CW_2480_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2480(&self) -> LO_VCO_IDAC_CW_2480_R {
        LO_VCO_IDAC_CW_2480_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2476(&self) -> LO_VCO_FREQ_CW_2476_R {
        LO_VCO_FREQ_CW_2476_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2476(&self) -> LO_VCO_IDAC_CW_2476_R {
        LO_VCO_IDAC_CW_2476_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2480(&mut self) -> LO_VCO_FREQ_CW_2480_W {
        LO_VCO_FREQ_CW_2480_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2480(&mut self) -> LO_VCO_IDAC_CW_2480_W {
        LO_VCO_IDAC_CW_2480_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2476(&mut self) -> LO_VCO_FREQ_CW_2476_W {
        LO_VCO_FREQ_CW_2476_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2476(&mut self) -> LO_VCO_IDAC_CW_2476_W {
        LO_VCO_IDAC_CW_2476_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_cal_ctrl_hw10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw10](index.html) module"]
pub struct LO_CAL_CTRL_HW10_SPEC;
impl crate::RegisterSpec for LO_CAL_CTRL_HW10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_cal_ctrl_hw10::R](R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw10::W](W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw10 to value 0"]
impl crate::Resettable for LO_CAL_CTRL_HW10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
