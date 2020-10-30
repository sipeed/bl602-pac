#[doc = "Register `lo_sdm_ctrl_hw5` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LO_SDM_CTRL_HW5_SPEC>> for R {
    fn from(reader: crate::R<LO_SDM_CTRL_HW5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw5` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW5_SPEC>;
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
impl core::convert::From<crate::W<LO_SDM_CTRL_HW5_SPEC>> for W {
    fn from(writer: crate::W<LO_SDM_CTRL_HW5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_bypass_mode` reader - "]
pub struct LO_SDM_BYPASS_MODE_R(crate::FieldReader<u8, u8>);
impl LO_SDM_BYPASS_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_BYPASS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_BYPASS_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_bypass_mode` writer - "]
pub struct LO_SDM_BYPASS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_BYPASS_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `lo_center_freq_mhz` reader - "]
pub struct LO_CENTER_FREQ_MHZ_R(crate::FieldReader<u16, u16>);
impl LO_CENTER_FREQ_MHZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        LO_CENTER_FREQ_MHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CENTER_FREQ_MHZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_center_freq_mhz` writer - "]
pub struct LO_CENTER_FREQ_MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CENTER_FREQ_MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&self) -> LO_SDM_BYPASS_MODE_R {
        LO_SDM_BYPASS_MODE_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&self) -> LO_CENTER_FREQ_MHZ_R {
        LO_CENTER_FREQ_MHZ_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&mut self) -> LO_SDM_BYPASS_MODE_W {
        LO_SDM_BYPASS_MODE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&mut self) -> LO_CENTER_FREQ_MHZ_W {
        LO_CENTER_FREQ_MHZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw5](index.html) module"]
pub struct LO_SDM_CTRL_HW5_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw5::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw5::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw5 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
