#[doc = "Register `TPLCR3` reader"]
pub struct R(crate::R<TPLCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPLCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPLCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPLCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPLCR3` writer"]
pub struct W(crate::W<TPLCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPLCR3_SPEC>;
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
impl From<crate::W<TPLCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPLCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tplcr` reader - "]
pub struct TPLCR_R(crate::FieldReader<u8, u8>);
impl TPLCR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPLCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPLCR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tplcr` writer - "]
pub struct TPLCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPLCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&self) -> TPLCR_R {
        TPLCR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&mut self) -> TPLCR_W {
        TPLCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TPLCR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplcr3](index.html) module"]
pub struct TPLCR3_SPEC;
impl crate::RegisterSpec for TPLCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tplcr3::R](R) reader structure"]
impl crate::Readable for TPLCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tplcr3::W](W) writer structure"]
impl crate::Writable for TPLCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPLCR3 to value 0"]
impl crate::Resettable for TPLCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
