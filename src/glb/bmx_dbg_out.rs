#[doc = "Register `bmx_dbg_out` reader"]
pub struct R(crate::R<BMX_DBG_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_DBG_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMX_DBG_OUT_SPEC>> for R {
    fn from(reader: crate::R<BMX_DBG_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_dbg_out` writer"]
pub struct W(crate::W<BMX_DBG_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_DBG_OUT_SPEC>;
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
impl core::convert::From<crate::W<BMX_DBG_OUT_SPEC>> for W {
    fn from(writer: crate::W<BMX_DBG_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bmx_dbg_out` reader - "]
pub struct BMX_DBG_OUT_R(crate::FieldReader<u32, u32>);
impl BMX_DBG_OUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMX_DBG_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_DBG_OUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_dbg_out` writer - "]
pub struct BMX_DBG_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_DBG_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&self) -> BMX_DBG_OUT_R {
        BMX_DBG_OUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&mut self) -> BMX_DBG_OUT_W {
        BMX_DBG_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_dbg_out.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_dbg_out](index.html) module"]
pub struct BMX_DBG_OUT_SPEC;
impl crate::RegisterSpec for BMX_DBG_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_dbg_out::R](R) reader structure"]
impl crate::Readable for BMX_DBG_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_dbg_out::W](W) writer structure"]
impl crate::Writable for BMX_DBG_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets bmx_dbg_out to value 0"]
impl crate::Resettable for BMX_DBG_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
