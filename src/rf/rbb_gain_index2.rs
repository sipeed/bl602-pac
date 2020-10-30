#[doc = "Register `rbb_gain_index2` reader"]
pub struct R(crate::R<RBB_GAIN_INDEX2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_GAIN_INDEX2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBB_GAIN_INDEX2_SPEC>> for R {
    fn from(reader: crate::R<RBB_GAIN_INDEX2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_gain_index2` writer"]
pub struct W(crate::W<RBB_GAIN_INDEX2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_GAIN_INDEX2_SPEC>;
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
impl core::convert::From<crate::W<RBB_GAIN_INDEX2_SPEC>> for W {
    fn from(writer: crate::W<RBB_GAIN_INDEX2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl7_gc_rbb2` reader - "]
pub struct GAIN_CTRL7_GC_RBB2_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL7_GC_RBB2_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL7_GC_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL7_GC_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl7_gc_rbb2` writer - "]
pub struct GAIN_CTRL7_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL7_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `gain_ctrl7_gc_rbb1` reader - "]
pub struct GAIN_CTRL7_GC_RBB1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL7_GC_RBB1_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL7_GC_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL7_GC_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl7_gc_rbb1` writer - "]
pub struct GAIN_CTRL7_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL7_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `gain_ctrl6_gc_rbb2` reader - "]
pub struct GAIN_CTRL6_GC_RBB2_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL6_GC_RBB2_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL6_GC_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL6_GC_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl6_gc_rbb2` writer - "]
pub struct GAIN_CTRL6_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL6_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `gain_ctrl6_gc_rbb1` reader - "]
pub struct GAIN_CTRL6_GC_RBB1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL6_GC_RBB1_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL6_GC_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL6_GC_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl6_gc_rbb1` writer - "]
pub struct GAIN_CTRL6_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL6_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `gain_ctrl5_gc_rbb2` reader - "]
pub struct GAIN_CTRL5_GC_RBB2_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL5_GC_RBB2_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL5_GC_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL5_GC_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl5_gc_rbb2` writer - "]
pub struct GAIN_CTRL5_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL5_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `gain_ctrl5_gc_rbb1` reader - "]
pub struct GAIN_CTRL5_GC_RBB1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL5_GC_RBB1_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL5_GC_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL5_GC_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl5_gc_rbb1` writer - "]
pub struct GAIN_CTRL5_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL5_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `gain_ctrl4_gc_rbb2` reader - "]
pub struct GAIN_CTRL4_GC_RBB2_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL4_GC_RBB2_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL4_GC_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL4_GC_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl4_gc_rbb2` writer - "]
pub struct GAIN_CTRL4_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL4_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `gain_ctrl4_gc_rbb1` reader - "]
pub struct GAIN_CTRL4_GC_RBB1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL4_GC_RBB1_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL4_GC_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL4_GC_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl4_gc_rbb1` writer - "]
pub struct GAIN_CTRL4_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL4_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb2(&self) -> GAIN_CTRL7_GC_RBB2_R {
        GAIN_CTRL7_GC_RBB2_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb1(&self) -> GAIN_CTRL7_GC_RBB1_R {
        GAIN_CTRL7_GC_RBB1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb2(&self) -> GAIN_CTRL6_GC_RBB2_R {
        GAIN_CTRL6_GC_RBB2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb1(&self) -> GAIN_CTRL6_GC_RBB1_R {
        GAIN_CTRL6_GC_RBB1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb2(&self) -> GAIN_CTRL5_GC_RBB2_R {
        GAIN_CTRL5_GC_RBB2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb1(&self) -> GAIN_CTRL5_GC_RBB1_R {
        GAIN_CTRL5_GC_RBB1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb2(&self) -> GAIN_CTRL4_GC_RBB2_R {
        GAIN_CTRL4_GC_RBB2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb1(&self) -> GAIN_CTRL4_GC_RBB1_R {
        GAIN_CTRL4_GC_RBB1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb2(&mut self) -> GAIN_CTRL7_GC_RBB2_W {
        GAIN_CTRL7_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rbb1(&mut self) -> GAIN_CTRL7_GC_RBB1_W {
        GAIN_CTRL7_GC_RBB1_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb2(&mut self) -> GAIN_CTRL6_GC_RBB2_W {
        GAIN_CTRL6_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rbb1(&mut self) -> GAIN_CTRL6_GC_RBB1_W {
        GAIN_CTRL6_GC_RBB1_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb2(&mut self) -> GAIN_CTRL5_GC_RBB2_W {
        GAIN_CTRL5_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rbb1(&mut self) -> GAIN_CTRL5_GC_RBB1_W {
        GAIN_CTRL5_GC_RBB1_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb2(&mut self) -> GAIN_CTRL4_GC_RBB2_W {
        GAIN_CTRL4_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rbb1(&mut self) -> GAIN_CTRL4_GC_RBB1_W {
        GAIN_CTRL4_GC_RBB1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_gain_index2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index2](index.html) module"]
pub struct RBB_GAIN_INDEX2_SPEC;
impl crate::RegisterSpec for RBB_GAIN_INDEX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_gain_index2::R](R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index2::W](W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb_gain_index2 to value 0"]
impl crate::Resettable for RBB_GAIN_INDEX2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
