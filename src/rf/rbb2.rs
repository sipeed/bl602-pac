#[doc = "Register `rbb2` reader"]
pub struct R(crate::R<RBB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBB2_SPEC>> for R {
    fn from(reader: crate::R<RBB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb2` writer"]
pub struct W(crate::W<RBB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB2_SPEC>;
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
impl core::convert::From<crate::W<RBB2_SPEC>> for W {
    fn from(writer: crate::W<RBB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_cap1_fc_i` reader - "]
pub struct RBB_CAP1_FC_I_R(crate::FieldReader<u8, u8>);
impl RBB_CAP1_FC_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_CAP1_FC_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_CAP1_FC_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_cap1_fc_i` writer - "]
pub struct RBB_CAP1_FC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP1_FC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `rbb_cap1_fc_q` reader - "]
pub struct RBB_CAP1_FC_Q_R(crate::FieldReader<u8, u8>);
impl RBB_CAP1_FC_Q_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_CAP1_FC_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_CAP1_FC_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_cap1_fc_q` writer - "]
pub struct RBB_CAP1_FC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP1_FC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `rbb_cap2_fc_i` reader - "]
pub struct RBB_CAP2_FC_I_R(crate::FieldReader<u8, u8>);
impl RBB_CAP2_FC_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_CAP2_FC_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_CAP2_FC_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_cap2_fc_i` writer - "]
pub struct RBB_CAP2_FC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP2_FC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `rbb_cap2_fc_q` reader - "]
pub struct RBB_CAP2_FC_Q_R(crate::FieldReader<u8, u8>);
impl RBB_CAP2_FC_Q_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_CAP2_FC_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_CAP2_FC_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_cap2_fc_q` writer - "]
pub struct RBB_CAP2_FC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP2_FC_Q_W<'a> {
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
    pub fn rbb_cap1_fc_i(&self) -> RBB_CAP1_FC_I_R {
        RBB_CAP1_FC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q(&self) -> RBB_CAP1_FC_Q_R {
        RBB_CAP1_FC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i(&self) -> RBB_CAP2_FC_I_R {
        RBB_CAP2_FC_I_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q(&self) -> RBB_CAP2_FC_Q_R {
        RBB_CAP2_FC_Q_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rbb_cap1_fc_i(&mut self) -> RBB_CAP1_FC_I_W {
        RBB_CAP1_FC_I_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q(&mut self) -> RBB_CAP1_FC_Q_W {
        RBB_CAP1_FC_Q_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i(&mut self) -> RBB_CAP2_FC_I_W {
        RBB_CAP2_FC_I_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q(&mut self) -> RBB_CAP2_FC_Q_W {
        RBB_CAP2_FC_Q_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb2](index.html) module"]
pub struct RBB2_SPEC;
impl crate::RegisterSpec for RBB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb2::R](R) reader structure"]
impl crate::Readable for RBB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb2::W](W) writer structure"]
impl crate::Writable for RBB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb2 to value 0"]
impl crate::Resettable for RBB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
