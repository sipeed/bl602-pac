#[doc = "Register `TMR2_1` reader"]
pub struct R(crate::R<TMR2_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR2_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR2_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR2_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR2_1` writer"]
pub struct W(crate::W<TMR2_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR2_1_SPEC>;
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
impl From<crate::W<TMR2_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR2_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmr` reader - "]
pub struct TMR_R(crate::FieldReader<u32, u32>);
impl TMR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr` writer - "]
pub struct TMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TMR_W {
        TMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR2_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2_1](index.html) module"]
pub struct TMR2_1_SPEC;
impl crate::RegisterSpec for TMR2_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr2_1::R](R) reader structure"]
impl crate::Readable for TMR2_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr2_1::W](W) writer structure"]
impl crate::Writable for TMR2_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR2_1 to value 0xffff_ffff"]
impl crate::Resettable for TMR2_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
