#[doc = "Register `rrf_gain_index1` reader"]
pub struct R(crate::R<RRF_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RRF_GAIN_INDEX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RRF_GAIN_INDEX1_SPEC>> for R {
    fn from(reader: crate::R<RRF_GAIN_INDEX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rrf_gain_index1` writer"]
pub struct W(crate::W<RRF_GAIN_INDEX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RRF_GAIN_INDEX1_SPEC>;
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
impl core::convert::From<crate::W<RRF_GAIN_INDEX1_SPEC>> for W {
    fn from(writer: crate::W<RRF_GAIN_INDEX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl5_gc_lna` reader - "]
pub struct GAIN_CTRL5_GC_LNA_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL5_GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL5_GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL5_GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl5_gc_lna` writer - "]
pub struct GAIN_CTRL5_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL5_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Field `gain_ctrl5_gc_rmxgm` reader - "]
pub struct GAIN_CTRL5_GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL5_GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL5_GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL5_GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl5_gc_rmxgm` writer - "]
pub struct GAIN_CTRL5_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL5_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `gain_ctrl4_gc_lna` reader - "]
pub struct GAIN_CTRL4_GC_LNA_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL4_GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL4_GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL4_GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl4_gc_lna` writer - "]
pub struct GAIN_CTRL4_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL4_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `gain_ctrl4_gc_rmxgm` reader - "]
pub struct GAIN_CTRL4_GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL4_GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL4_GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL4_GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl4_gc_rmxgm` writer - "]
pub struct GAIN_CTRL4_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL4_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `gain_ctrl3_gc_lna` reader - "]
pub struct GAIN_CTRL3_GC_LNA_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL3_GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL3_GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL3_GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl3_gc_lna` writer - "]
pub struct GAIN_CTRL3_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL3_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `gain_ctrl3_gc_rmxgm` reader - "]
pub struct GAIN_CTRL3_GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL3_GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL3_GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL3_GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl3_gc_rmxgm` writer - "]
pub struct GAIN_CTRL3_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL3_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `gain_ctrl2_gc_lna` reader - "]
pub struct GAIN_CTRL2_GC_LNA_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL2_GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL2_GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL2_GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl2_gc_lna` writer - "]
pub struct GAIN_CTRL2_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL2_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `gain_ctrl2_gc_rmxgm` reader - "]
pub struct GAIN_CTRL2_GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL2_GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL2_GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL2_GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl2_gc_rmxgm` writer - "]
pub struct GAIN_CTRL2_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL2_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `gain_ctrl1_gc_lna` reader - "]
pub struct GAIN_CTRL1_GC_LNA_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL1_GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL1_GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL1_GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl1_gc_lna` writer - "]
pub struct GAIN_CTRL1_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `gain_ctrl1_gc_rmxgm` reader - "]
pub struct GAIN_CTRL1_GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL1_GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL1_GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL1_GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl1_gc_rmxgm` writer - "]
pub struct GAIN_CTRL1_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `gain_ctrl0_gc_lna` reader - "]
pub struct GAIN_CTRL0_GC_LNA_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL0_GC_LNA_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL0_GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL0_GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl0_gc_lna` writer - "]
pub struct GAIN_CTRL0_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `gain_ctrl0_gc_rmxgm` reader - "]
pub struct GAIN_CTRL0_GC_RMXGM_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL0_GC_RMXGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL0_GC_RMXGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL0_GC_RMXGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl0_gc_rmxgm` writer - "]
pub struct GAIN_CTRL0_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&self) -> GAIN_CTRL5_GC_LNA_R {
        GAIN_CTRL5_GC_LNA_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&self) -> GAIN_CTRL5_GC_RMXGM_R {
        GAIN_CTRL5_GC_RMXGM_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&self) -> GAIN_CTRL4_GC_LNA_R {
        GAIN_CTRL4_GC_LNA_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&self) -> GAIN_CTRL4_GC_RMXGM_R {
        GAIN_CTRL4_GC_RMXGM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&self) -> GAIN_CTRL3_GC_LNA_R {
        GAIN_CTRL3_GC_LNA_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&self) -> GAIN_CTRL3_GC_RMXGM_R {
        GAIN_CTRL3_GC_RMXGM_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&self) -> GAIN_CTRL2_GC_LNA_R {
        GAIN_CTRL2_GC_LNA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&self) -> GAIN_CTRL2_GC_RMXGM_R {
        GAIN_CTRL2_GC_RMXGM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&self) -> GAIN_CTRL1_GC_LNA_R {
        GAIN_CTRL1_GC_LNA_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&self) -> GAIN_CTRL1_GC_RMXGM_R {
        GAIN_CTRL1_GC_RMXGM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&self) -> GAIN_CTRL0_GC_LNA_R {
        GAIN_CTRL0_GC_LNA_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&self) -> GAIN_CTRL0_GC_RMXGM_R {
        GAIN_CTRL0_GC_RMXGM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&mut self) -> GAIN_CTRL5_GC_LNA_W {
        GAIN_CTRL5_GC_LNA_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&mut self) -> GAIN_CTRL5_GC_RMXGM_W {
        GAIN_CTRL5_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&mut self) -> GAIN_CTRL4_GC_LNA_W {
        GAIN_CTRL4_GC_LNA_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&mut self) -> GAIN_CTRL4_GC_RMXGM_W {
        GAIN_CTRL4_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&mut self) -> GAIN_CTRL3_GC_LNA_W {
        GAIN_CTRL3_GC_LNA_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&mut self) -> GAIN_CTRL3_GC_RMXGM_W {
        GAIN_CTRL3_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&mut self) -> GAIN_CTRL2_GC_LNA_W {
        GAIN_CTRL2_GC_LNA_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&mut self) -> GAIN_CTRL2_GC_RMXGM_W {
        GAIN_CTRL2_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&mut self) -> GAIN_CTRL1_GC_LNA_W {
        GAIN_CTRL1_GC_LNA_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&mut self) -> GAIN_CTRL1_GC_RMXGM_W {
        GAIN_CTRL1_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&mut self) -> GAIN_CTRL0_GC_LNA_W {
        GAIN_CTRL0_GC_LNA_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&mut self) -> GAIN_CTRL0_GC_RMXGM_W {
        GAIN_CTRL0_GC_RMXGM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rrf_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rrf_gain_index1](index.html) module"]
pub struct RRF_GAIN_INDEX1_SPEC;
impl crate::RegisterSpec for RRF_GAIN_INDEX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rrf_gain_index1::R](R) reader structure"]
impl crate::Readable for RRF_GAIN_INDEX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rrf_gain_index1::W](W) writer structure"]
impl crate::Writable for RRF_GAIN_INDEX1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rrf_gain_index1 to value 0"]
impl crate::Resettable for RRF_GAIN_INDEX1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
