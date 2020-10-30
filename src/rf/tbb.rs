#[doc = "Register `tbb` reader"]
pub struct R(crate::R<TBB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TBB_SPEC>> for R {
    fn from(reader: crate::R<TBB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tbb` writer"]
pub struct W(crate::W<TBB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBB_SPEC>;
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
impl core::convert::From<crate::W<TBB_SPEC>> for W {
    fn from(writer: crate::W<TBB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tbb_tosdac_i` reader - "]
pub struct TBB_TOSDAC_I_R(crate::FieldReader<u8, u8>);
impl TBB_TOSDAC_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_TOSDAC_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_TOSDAC_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_tosdac_i` writer - "]
pub struct TBB_TOSDAC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `tbb_tosdac_q` reader - "]
pub struct TBB_TOSDAC_Q_R(crate::FieldReader<u8, u8>);
impl TBB_TOSDAC_Q_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_TOSDAC_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_TOSDAC_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_tosdac_q` writer - "]
pub struct TBB_TOSDAC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `tbb_atest_out_en` reader - "]
pub struct TBB_ATEST_OUT_EN_R(crate::FieldReader<bool, bool>);
impl TBB_ATEST_OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBB_ATEST_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_ATEST_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_atest_out_en` writer - "]
pub struct TBB_ATEST_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_ATEST_OUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `tbb_iq_bias_short` reader - "]
pub struct TBB_IQ_BIAS_SHORT_R(crate::FieldReader<bool, bool>);
impl TBB_IQ_BIAS_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBB_IQ_BIAS_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_IQ_BIAS_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_iq_bias_short` writer - "]
pub struct TBB_IQ_BIAS_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_IQ_BIAS_SHORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `tbb_cflt` reader - "]
pub struct TBB_CFLT_R(crate::FieldReader<u8, u8>);
impl TBB_CFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_CFLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_CFLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_cflt` writer - "]
pub struct TBB_CFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_CFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `tbb_vcm` reader - "]
pub struct TBB_VCM_R(crate::FieldReader<u8, u8>);
impl TBB_VCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_VCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_VCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_vcm` writer - "]
pub struct TBB_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `tbb_bm_cg` reader - "]
pub struct TBB_BM_CG_R(crate::FieldReader<u8, u8>);
impl TBB_BM_CG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_BM_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_BM_CG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_bm_cg` writer - "]
pub struct TBB_BM_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_BM_CG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `tbb_bm_sf` reader - "]
pub struct TBB_BM_SF_R(crate::FieldReader<u8, u8>);
impl TBB_BM_SF_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_BM_SF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_BM_SF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_bm_sf` writer - "]
pub struct TBB_BM_SF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_BM_SF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&self) -> TBB_TOSDAC_I_R {
        TBB_TOSDAC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&self) -> TBB_TOSDAC_Q_R {
        TBB_TOSDAC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&self) -> TBB_ATEST_OUT_EN_R {
        TBB_ATEST_OUT_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&self) -> TBB_IQ_BIAS_SHORT_R {
        TBB_IQ_BIAS_SHORT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&self) -> TBB_CFLT_R {
        TBB_CFLT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&self) -> TBB_VCM_R {
        TBB_VCM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&self) -> TBB_BM_CG_R {
        TBB_BM_CG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&self) -> TBB_BM_SF_R {
        TBB_BM_SF_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&mut self) -> TBB_TOSDAC_I_W {
        TBB_TOSDAC_I_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&mut self) -> TBB_TOSDAC_Q_W {
        TBB_TOSDAC_Q_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&mut self) -> TBB_ATEST_OUT_EN_W {
        TBB_ATEST_OUT_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&mut self) -> TBB_IQ_BIAS_SHORT_W {
        TBB_IQ_BIAS_SHORT_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&mut self) -> TBB_CFLT_W {
        TBB_CFLT_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&mut self) -> TBB_VCM_W {
        TBB_VCM_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&mut self) -> TBB_BM_CG_W {
        TBB_BM_CG_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&mut self) -> TBB_BM_SF_W {
        TBB_BM_SF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tbb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb](index.html) module"]
pub struct TBB_SPEC;
impl crate::RegisterSpec for TBB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbb::R](R) reader structure"]
impl crate::Readable for TBB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbb::W](W) writer structure"]
impl crate::Writable for TBB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tbb to value 0"]
impl crate::Resettable for TBB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
