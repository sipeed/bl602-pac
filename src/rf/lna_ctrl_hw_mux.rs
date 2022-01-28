#[doc = "Register `lna_ctrl_hw_mux` reader"]
pub struct R(crate::R<LNA_CTRL_HW_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LNA_CTRL_HW_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LNA_CTRL_HW_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LNA_CTRL_HW_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lna_ctrl_hw_mux` writer"]
pub struct W(crate::W<LNA_CTRL_HW_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LNA_CTRL_HW_MUX_SPEC>;
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
impl From<crate::W<LNA_CTRL_HW_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LNA_CTRL_HW_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lna_load_csw_lg` reader - "]
pub struct LNA_LOAD_CSW_LG_R(crate::FieldReader<u8, u8>);
impl LNA_LOAD_CSW_LG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LOAD_CSW_LG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LOAD_CSW_LG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_load_csw_lg` writer - "]
pub struct LNA_LOAD_CSW_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `lna_load_csw_hg` reader - "]
pub struct LNA_LOAD_CSW_HG_R(crate::FieldReader<u8, u8>);
impl LNA_LOAD_CSW_HG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LOAD_CSW_HG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LOAD_CSW_HG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_load_csw_hg` writer - "]
pub struct LNA_LOAD_CSW_HG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_HG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `lna_bm_lg` reader - "]
pub struct LNA_BM_LG_R(crate::FieldReader<u8, u8>);
impl LNA_BM_LG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_LG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_LG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm_lg` writer - "]
pub struct LNA_BM_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `lna_bm_hg` reader - "]
pub struct LNA_BM_HG_R(crate::FieldReader<u8, u8>);
impl LNA_BM_HG_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_HG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_HG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm_hg` writer - "]
pub struct LNA_BM_HG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_HG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_lg(&self) -> LNA_LOAD_CSW_LG_R {
        LNA_LOAD_CSW_LG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw_hg(&self) -> LNA_LOAD_CSW_HG_R {
        LNA_LOAD_CSW_HG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_lg(&self) -> LNA_BM_LG_R {
        LNA_BM_LG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm_hg(&self) -> LNA_BM_HG_R {
        LNA_BM_HG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_lg(&mut self) -> LNA_LOAD_CSW_LG_W {
        LNA_LOAD_CSW_LG_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw_hg(&mut self) -> LNA_LOAD_CSW_HG_W {
        LNA_LOAD_CSW_HG_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_lg(&mut self) -> LNA_BM_LG_W {
        LNA_BM_LG_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm_hg(&mut self) -> LNA_BM_HG_W {
        LNA_BM_HG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lna_ctrl_hw_mux.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna_ctrl_hw_mux](index.html) module"]
pub struct LNA_CTRL_HW_MUX_SPEC;
impl crate::RegisterSpec for LNA_CTRL_HW_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lna_ctrl_hw_mux::R](R) reader structure"]
impl crate::Readable for LNA_CTRL_HW_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lna_ctrl_hw_mux::W](W) writer structure"]
impl crate::Writable for LNA_CTRL_HW_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lna_ctrl_hw_mux to value 0"]
impl crate::Resettable for LNA_CTRL_HW_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
