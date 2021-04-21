#[doc = "Register `trx_gain1` reader"]
pub struct R(crate::R<TRX_GAIN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRX_GAIN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRX_GAIN1_SPEC>> for R {
    fn from(reader: crate::R<TRX_GAIN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `trx_gain1` writer"]
pub struct W(crate::W<TRX_GAIN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRX_GAIN1_SPEC>;
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
impl core::convert::From<crate::W<TRX_GAIN1_SPEC>> for W {
    fn from(writer: crate::W<TRX_GAIN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gc_tbb_boost` reader - "]
pub struct GC_TBB_BOOST_R(crate::FieldReader<u8, u8>);
impl GC_TBB_BOOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_TBB_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_TBB_BOOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_tbb_boost` writer - "]
pub struct GC_TBB_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_TBB_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `gc_tbb` reader - "]
pub struct GC_TBB_R(crate::FieldReader<u8, u8>);
impl GC_TBB_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_TBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_TBB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_tbb` writer - "]
pub struct GC_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_TBB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `gc_tmx` reader - "]
pub struct GC_TMX_R(crate::FieldReader<u8, u8>);
impl GC_TMX_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_TMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_TMX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_tmx` writer - "]
pub struct GC_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_TMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `gc_rbb2` reader - "]
pub struct GC_RBB2_R(crate::FieldReader<u8, u8>);
impl GC_RBB2_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_rbb2` writer - "]
pub struct GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `gc_rbb1` reader - "]
pub struct GC_RBB1_R(crate::FieldReader<u8, u8>);
impl GC_RBB1_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_rbb1` writer - "]
pub struct GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `gc_rmxgm` reader - "]
pub struct GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_rmxgm` writer - "]
pub struct GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `gc_lna` reader - "]
pub struct GC_LNA_R(crate::FieldReader<u8, u8>);
impl GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_lna` writer - "]
pub struct GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost(&self) -> GC_TBB_BOOST_R {
        GC_TBB_BOOST_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb(&self) -> GC_TBB_R {
        GC_TBB_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx(&self) -> GC_TMX_R {
        GC_TMX_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2(&self) -> GC_RBB2_R {
        GC_RBB2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1(&self) -> GC_RBB1_R {
        GC_RBB1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm(&self) -> GC_RMXGM_R {
        GC_RMXGM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&self) -> GC_LNA_R {
        GC_LNA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost(&mut self) -> GC_TBB_BOOST_W {
        GC_TBB_BOOST_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb(&mut self) -> GC_TBB_W {
        GC_TBB_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx(&mut self) -> GC_TMX_W {
        GC_TMX_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2(&mut self) -> GC_RBB2_W {
        GC_RBB2_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1(&mut self) -> GC_RBB1_W {
        GC_RBB1_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm(&mut self) -> GC_RMXGM_W {
        GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&mut self) -> GC_LNA_W {
        GC_LNA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gain control1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain1](index.html) module"]
pub struct TRX_GAIN1_SPEC;
impl crate::RegisterSpec for TRX_GAIN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trx_gain1::R](R) reader structure"]
impl crate::Readable for TRX_GAIN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trx_gain1::W](W) writer structure"]
impl crate::Writable for TRX_GAIN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets trx_gain1 to value 0"]
impl crate::Resettable for TRX_GAIN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
