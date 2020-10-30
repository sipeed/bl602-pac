#[doc = "Register `vco3` reader"]
pub struct R(crate::R<VCO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VCO3_SPEC>> for R {
    fn from(reader: crate::R<VCO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco3` writer"]
pub struct W(crate::W<VCO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO3_SPEC>;
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
impl core::convert::From<crate::W<VCO3_SPEC>> for W {
    fn from(writer: crate::W<VCO3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_cnt_op` reader - "]
pub struct FCAL_CNT_OP_R(crate::FieldReader<u16, u16>);
impl FCAL_CNT_OP_R {
    pub(crate) fn new(bits: u16) -> Self {
        FCAL_CNT_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_CNT_OP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_cnt_op` writer - "]
pub struct FCAL_CNT_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CNT_OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `fcal_div` reader - "]
pub struct FCAL_DIV_R(crate::FieldReader<u16, u16>);
impl FCAL_DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        FCAL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_div` writer - "]
pub struct FCAL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn fcal_cnt_op(&self) -> FCAL_CNT_OP_R {
        FCAL_CNT_OP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&self) -> FCAL_DIV_R {
        FCAL_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn fcal_cnt_op(&mut self) -> FCAL_CNT_OP_W {
        FCAL_CNT_OP_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&mut self) -> FCAL_DIV_W {
        FCAL_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco3](index.html) module"]
pub struct VCO3_SPEC;
impl crate::RegisterSpec for VCO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco3::R](R) reader structure"]
impl crate::Readable for VCO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco3::W](W) writer structure"]
impl crate::Writable for VCO3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vco3 to value 0"]
impl crate::Resettable for VCO3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
