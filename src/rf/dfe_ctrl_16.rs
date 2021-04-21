#[doc = "Register `dfe_ctrl_16` reader"]
pub struct R(crate::R<DFE_CTRL_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_16_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_16` writer"]
pub struct W(crate::W<DFE_CTRL_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_16_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_16_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_tbb_ind_gc7` reader - "]
pub struct RF_TBB_IND_GC7_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC7_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc7` writer - "]
pub struct RF_TBB_IND_GC7_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc6` reader - "]
pub struct RF_TBB_IND_GC6_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC6_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc6` writer - "]
pub struct RF_TBB_IND_GC6_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc5` reader - "]
pub struct RF_TBB_IND_GC5_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC5_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc5` writer - "]
pub struct RF_TBB_IND_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc4` reader - "]
pub struct RF_TBB_IND_GC4_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc4` writer - "]
pub struct RF_TBB_IND_GC4_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc3` reader - "]
pub struct RF_TBB_IND_GC3_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc3` writer - "]
pub struct RF_TBB_IND_GC3_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc2` reader - "]
pub struct RF_TBB_IND_GC2_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc2` writer - "]
pub struct RF_TBB_IND_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc1` reader - "]
pub struct RF_TBB_IND_GC1_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc1` writer - "]
pub struct RF_TBB_IND_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc0` reader - "]
pub struct RF_TBB_IND_GC0_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc0` writer - "]
pub struct RF_TBB_IND_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&self) -> RF_TBB_IND_GC7_R {
        RF_TBB_IND_GC7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&self) -> RF_TBB_IND_GC6_R {
        RF_TBB_IND_GC6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&self) -> RF_TBB_IND_GC5_R {
        RF_TBB_IND_GC5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&self) -> RF_TBB_IND_GC4_R {
        RF_TBB_IND_GC4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&self) -> RF_TBB_IND_GC3_R {
        RF_TBB_IND_GC3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&self) -> RF_TBB_IND_GC2_R {
        RF_TBB_IND_GC2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&self) -> RF_TBB_IND_GC1_R {
        RF_TBB_IND_GC1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&self) -> RF_TBB_IND_GC0_R {
        RF_TBB_IND_GC0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&mut self) -> RF_TBB_IND_GC7_W {
        RF_TBB_IND_GC7_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&mut self) -> RF_TBB_IND_GC6_W {
        RF_TBB_IND_GC6_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&mut self) -> RF_TBB_IND_GC5_W {
        RF_TBB_IND_GC5_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&mut self) -> RF_TBB_IND_GC4_W {
        RF_TBB_IND_GC4_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&mut self) -> RF_TBB_IND_GC3_W {
        RF_TBB_IND_GC3_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&mut self) -> RF_TBB_IND_GC2_W {
        RF_TBB_IND_GC2_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&mut self) -> RF_TBB_IND_GC1_W {
        RF_TBB_IND_GC1_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&mut self) -> RF_TBB_IND_GC0_W {
        RF_TBB_IND_GC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_16.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_16](index.html) module"]
pub struct DFE_CTRL_16_SPEC;
impl crate::RegisterSpec for DFE_CTRL_16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_16::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_16::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_16 to value 0"]
impl crate::Resettable for DFE_CTRL_16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
