#[doc = "Register `tosdac_ctrl_hw2` reader"]
pub struct R(crate::R<TOSDAC_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOSDAC_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TOSDAC_CTRL_HW2_SPEC>> for R {
    fn from(reader: crate::R<TOSDAC_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tosdac_ctrl_hw2` writer"]
pub struct W(crate::W<TOSDAC_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOSDAC_CTRL_HW2_SPEC>;
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
impl core::convert::From<crate::W<TOSDAC_CTRL_HW2_SPEC>> for W {
    fn from(writer: crate::W<TOSDAC_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tbb_tosdac_q_gc3` reader - "]
pub struct TBB_TOSDAC_Q_GC3_R(crate::FieldReader<u8, u8>);
impl TBB_TOSDAC_Q_GC3_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_TOSDAC_Q_GC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_TOSDAC_Q_GC3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_tosdac_q_gc3` writer - "]
pub struct TBB_TOSDAC_Q_GC3_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_Q_GC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `tbb_tosdac_i_gc3` reader - "]
pub struct TBB_TOSDAC_I_GC3_R(crate::FieldReader<u8, u8>);
impl TBB_TOSDAC_I_GC3_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_TOSDAC_I_GC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_TOSDAC_I_GC3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_tosdac_i_gc3` writer - "]
pub struct TBB_TOSDAC_I_GC3_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_I_GC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `tbb_tosdac_q_gc2` reader - "]
pub struct TBB_TOSDAC_Q_GC2_R(crate::FieldReader<u8, u8>);
impl TBB_TOSDAC_Q_GC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_TOSDAC_Q_GC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_TOSDAC_Q_GC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_tosdac_q_gc2` writer - "]
pub struct TBB_TOSDAC_Q_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_Q_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `tbb_tosdac_i_gc2` reader - "]
pub struct TBB_TOSDAC_I_GC2_R(crate::FieldReader<u8, u8>);
impl TBB_TOSDAC_I_GC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBB_TOSDAC_I_GC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBB_TOSDAC_I_GC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbb_tosdac_i_gc2` writer - "]
pub struct TBB_TOSDAC_I_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_I_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc3(&self) -> TBB_TOSDAC_Q_GC3_R {
        TBB_TOSDAC_Q_GC3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc3(&self) -> TBB_TOSDAC_I_GC3_R {
        TBB_TOSDAC_I_GC3_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc2(&self) -> TBB_TOSDAC_Q_GC2_R {
        TBB_TOSDAC_Q_GC2_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc2(&self) -> TBB_TOSDAC_I_GC2_R {
        TBB_TOSDAC_I_GC2_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc3(&mut self) -> TBB_TOSDAC_Q_GC3_W {
        TBB_TOSDAC_Q_GC3_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc3(&mut self) -> TBB_TOSDAC_I_GC3_W {
        TBB_TOSDAC_I_GC3_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc2(&mut self) -> TBB_TOSDAC_Q_GC2_W {
        TBB_TOSDAC_Q_GC2_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc2(&mut self) -> TBB_TOSDAC_I_GC2_W {
        TBB_TOSDAC_I_GC2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tosdac_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tosdac_ctrl_hw2](index.html) module"]
pub struct TOSDAC_CTRL_HW2_SPEC;
impl crate::RegisterSpec for TOSDAC_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tosdac_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for TOSDAC_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tosdac_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for TOSDAC_CTRL_HW2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tosdac_ctrl_hw2 to value 0"]
impl crate::Resettable for TOSDAC_CTRL_HW2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
