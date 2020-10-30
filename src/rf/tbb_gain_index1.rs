#[doc = "Register `tbb_gain_index1` reader"]
pub struct R(crate::R<TBB_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBB_GAIN_INDEX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TBB_GAIN_INDEX1_SPEC>> for R {
    fn from(reader: crate::R<TBB_GAIN_INDEX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tbb_gain_index1` writer"]
pub struct W(crate::W<TBB_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBB_GAIN_INDEX1_SPEC>;
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
impl core::convert::From<crate::W<TBB_GAIN_INDEX1_SPEC>> for W {
    fn from(writer: crate::W<TBB_GAIN_INDEX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl1_gc_tbb_boost` reader - "]
pub struct GAIN_CTRL1_GC_TBB_BOOST_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL1_GC_TBB_BOOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL1_GC_TBB_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL1_GC_TBB_BOOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl1_gc_tbb_boost` writer - "]
pub struct GAIN_CTRL1_GC_TBB_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_TBB_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `gain_ctrl1_dac_bias_sel` reader - "]
pub struct GAIN_CTRL1_DAC_BIAS_SEL_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL1_DAC_BIAS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL1_DAC_BIAS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL1_DAC_BIAS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl1_dac_bias_sel` writer - "]
pub struct GAIN_CTRL1_DAC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_DAC_BIAS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `gain_ctrl1_gc_tmx` reader - "]
pub struct GAIN_CTRL1_GC_TMX_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL1_GC_TMX_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL1_GC_TMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL1_GC_TMX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl1_gc_tmx` writer - "]
pub struct GAIN_CTRL1_GC_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_TMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `gain_ctrl1_gc_tbb` reader - "]
pub struct GAIN_CTRL1_GC_TBB_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL1_GC_TBB_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL1_GC_TBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL1_GC_TBB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl1_gc_tbb` writer - "]
pub struct GAIN_CTRL1_GC_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_TBB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `gain_ctrl0_gc_tbb_boost` reader - "]
pub struct GAIN_CTRL0_GC_TBB_BOOST_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL0_GC_TBB_BOOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL0_GC_TBB_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL0_GC_TBB_BOOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl0_gc_tbb_boost` writer - "]
pub struct GAIN_CTRL0_GC_TBB_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_TBB_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `gain_ctrl0_dac_bias_sel` reader - "]
pub struct GAIN_CTRL0_DAC_BIAS_SEL_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL0_DAC_BIAS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL0_DAC_BIAS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL0_DAC_BIAS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl0_dac_bias_sel` writer - "]
pub struct GAIN_CTRL0_DAC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_DAC_BIAS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `gain_ctrl0_gc_tmx` reader - "]
pub struct GAIN_CTRL0_GC_TMX_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL0_GC_TMX_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL0_GC_TMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL0_GC_TMX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl0_gc_tmx` writer - "]
pub struct GAIN_CTRL0_GC_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_TMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `gain_ctrl0_gc_tbb` reader - "]
pub struct GAIN_CTRL0_GC_TBB_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL0_GC_TBB_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL0_GC_TBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL0_GC_TBB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl0_gc_tbb` writer - "]
pub struct GAIN_CTRL0_GC_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_TBB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb_boost(&self) -> GAIN_CTRL1_GC_TBB_BOOST_R {
        GAIN_CTRL1_GC_TBB_BOOST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl1_dac_bias_sel(&self) -> GAIN_CTRL1_DAC_BIAS_SEL_R {
        GAIN_CTRL1_DAC_BIAS_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tmx(&self) -> GAIN_CTRL1_GC_TMX_R {
        GAIN_CTRL1_GC_TMX_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb(&self) -> GAIN_CTRL1_GC_TBB_R {
        GAIN_CTRL1_GC_TBB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb_boost(&self) -> GAIN_CTRL0_GC_TBB_BOOST_R {
        GAIN_CTRL0_GC_TBB_BOOST_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl0_dac_bias_sel(&self) -> GAIN_CTRL0_DAC_BIAS_SEL_R {
        GAIN_CTRL0_DAC_BIAS_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tmx(&self) -> GAIN_CTRL0_GC_TMX_R {
        GAIN_CTRL0_GC_TMX_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb(&self) -> GAIN_CTRL0_GC_TBB_R {
        GAIN_CTRL0_GC_TBB_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb_boost(&mut self) -> GAIN_CTRL1_GC_TBB_BOOST_W {
        GAIN_CTRL1_GC_TBB_BOOST_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl1_dac_bias_sel(&mut self) -> GAIN_CTRL1_DAC_BIAS_SEL_W {
        GAIN_CTRL1_DAC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tmx(&mut self) -> GAIN_CTRL1_GC_TMX_W {
        GAIN_CTRL1_GC_TMX_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb(&mut self) -> GAIN_CTRL1_GC_TBB_W {
        GAIN_CTRL1_GC_TBB_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb_boost(&mut self) -> GAIN_CTRL0_GC_TBB_BOOST_W {
        GAIN_CTRL0_GC_TBB_BOOST_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl0_dac_bias_sel(&mut self) -> GAIN_CTRL0_DAC_BIAS_SEL_W {
        GAIN_CTRL0_DAC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tmx(&mut self) -> GAIN_CTRL0_GC_TMX_W {
        GAIN_CTRL0_GC_TMX_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb(&mut self) -> GAIN_CTRL0_GC_TBB_W {
        GAIN_CTRL0_GC_TBB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tbb_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb_gain_index1](index.html) module"]
pub struct TBB_GAIN_INDEX1_SPEC;
impl crate::RegisterSpec for TBB_GAIN_INDEX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbb_gain_index1::R](R) reader structure"]
impl crate::Readable for TBB_GAIN_INDEX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbb_gain_index1::W](W) writer structure"]
impl crate::Writable for TBB_GAIN_INDEX1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tbb_gain_index1 to value 0"]
impl crate::Resettable for TBB_GAIN_INDEX1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
