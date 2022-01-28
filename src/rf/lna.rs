#[doc = "Register `lna` reader"]
pub struct R(crate::R<LNA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LNA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LNA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LNA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lna` writer"]
pub struct W(crate::W<LNA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LNA_SPEC>;
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
impl From<crate::W<LNA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LNA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lna_lg_gsel` reader - "]
pub struct LNA_LG_GSEL_R(crate::FieldReader<u8, u8>);
impl LNA_LG_GSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LG_GSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LG_GSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_lg_gsel` writer - "]
pub struct LNA_LG_GSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LG_GSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `lna_cap_lg` reader - "]
pub struct LNA_CAP_LG_R(crate::FieldReader<u8, u8>);
impl LNA_CAP_LG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_CAP_LG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_CAP_LG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_cap_lg` writer - "]
pub struct LNA_CAP_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CAP_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `lna_rfb_match` reader - "]
pub struct LNA_RFB_MATCH_R(crate::FieldReader<u8, u8>);
impl LNA_RFB_MATCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_RFB_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_RFB_MATCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_rfb_match` writer - "]
pub struct LNA_RFB_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_RFB_MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `lna_load_csw_hw` reader - "]
pub struct LNA_LOAD_CSW_HW_R(crate::FieldReader<u8, u8>);
impl LNA_LOAD_CSW_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LOAD_CSW_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LOAD_CSW_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_load_csw_hw` writer - "]
pub struct LNA_LOAD_CSW_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `lna_load_csw` reader - "]
pub struct LNA_LOAD_CSW_R(crate::FieldReader<u8, u8>);
impl LNA_LOAD_CSW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LOAD_CSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LOAD_CSW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_load_csw` writer - "]
pub struct LNA_LOAD_CSW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `lna_bm_hw` reader - "]
pub struct LNA_BM_HW_R(crate::FieldReader<u8, u8>);
impl LNA_BM_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm_hw` writer - "]
pub struct LNA_BM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `lna_bm` reader - "]
pub struct LNA_BM_R(crate::FieldReader<u8, u8>);
impl LNA_BM_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm` writer - "]
pub struct LNA_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&self) -> LNA_LG_GSEL_R {
        LNA_LG_GSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&self) -> LNA_CAP_LG_R {
        LNA_CAP_LG_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&self) -> LNA_RFB_MATCH_R {
        LNA_RFB_MATCH_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&self) -> LNA_LOAD_CSW_HW_R {
        LNA_LOAD_CSW_HW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&self) -> LNA_LOAD_CSW_R {
        LNA_LOAD_CSW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&self) -> LNA_BM_HW_R {
        LNA_BM_HW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&self) -> LNA_BM_R {
        LNA_BM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&mut self) -> LNA_LG_GSEL_W {
        LNA_LG_GSEL_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&mut self) -> LNA_CAP_LG_W {
        LNA_CAP_LG_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&mut self) -> LNA_RFB_MATCH_W {
        LNA_RFB_MATCH_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&mut self) -> LNA_LOAD_CSW_HW_W {
        LNA_LOAD_CSW_HW_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&mut self) -> LNA_LOAD_CSW_W {
        LNA_LOAD_CSW_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&mut self) -> LNA_BM_HW_W {
        LNA_BM_HW_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&mut self) -> LNA_BM_W {
        LNA_BM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lna.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna](index.html) module"]
pub struct LNA_SPEC;
impl crate::RegisterSpec for LNA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lna::R](R) reader structure"]
impl crate::Readable for LNA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lna::W](W) writer structure"]
impl crate::Writable for LNA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lna to value 0"]
impl crate::Resettable for LNA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
