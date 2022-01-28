#[doc = "Register `vco1` reader"]
pub struct R(crate::R<VCO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco1` writer"]
pub struct W(crate::W<VCO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO1_SPEC>;
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
impl From<crate::W<VCO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_vco_idac_cw_hw` reader - "]
pub struct LO_VCO_IDAC_CW_HW_R(crate::FieldReader<u8, u8>);
impl LO_VCO_IDAC_CW_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_IDAC_CW_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_CW_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_cw_hw` writer - "]
pub struct LO_VCO_IDAC_CW_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `lo_vco_idac_cw` reader - "]
pub struct LO_VCO_IDAC_CW_R(crate::FieldReader<u8, u8>);
impl LO_VCO_IDAC_CW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_IDAC_CW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_IDAC_CW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_idac_cw` writer - "]
pub struct LO_VCO_IDAC_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `lo_vco_freq_cw_hw` reader - "]
pub struct LO_VCO_FREQ_CW_HW_R(crate::FieldReader<u8, u8>);
impl LO_VCO_FREQ_CW_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_FREQ_CW_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_FREQ_CW_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_freq_cw_hw` writer - "]
pub struct LO_VCO_FREQ_CW_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `lo_vco_freq_cw` reader - "]
pub struct LO_VCO_FREQ_CW_R(crate::FieldReader<u8, u8>);
impl LO_VCO_FREQ_CW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_VCO_FREQ_CW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_VCO_FREQ_CW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_vco_freq_cw` writer - "]
pub struct LO_VCO_FREQ_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_hw(&self) -> LO_VCO_IDAC_CW_HW_R {
        LO_VCO_IDAC_CW_HW_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw(&self) -> LO_VCO_IDAC_CW_R {
        LO_VCO_IDAC_CW_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_hw(&self) -> LO_VCO_FREQ_CW_HW_R {
        LO_VCO_FREQ_CW_HW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lo_vco_freq_cw(&self) -> LO_VCO_FREQ_CW_R {
        LO_VCO_FREQ_CW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_hw(&mut self) -> LO_VCO_IDAC_CW_HW_W {
        LO_VCO_IDAC_CW_HW_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw(&mut self) -> LO_VCO_IDAC_CW_W {
        LO_VCO_IDAC_CW_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_hw(&mut self) -> LO_VCO_FREQ_CW_HW_W {
        LO_VCO_FREQ_CW_HW_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lo_vco_freq_cw(&mut self) -> LO_VCO_FREQ_CW_W {
        LO_VCO_FREQ_CW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco1](index.html) module"]
pub struct VCO1_SPEC;
impl crate::RegisterSpec for VCO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco1::R](R) reader structure"]
impl crate::Readable for VCO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco1::W](W) writer structure"]
impl crate::Writable for VCO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vco1 to value 0"]
impl crate::Resettable for VCO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
