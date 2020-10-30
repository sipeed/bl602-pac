#[doc = "Register `TCR3` reader"]
pub struct R(crate::R<TCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCR3_SPEC>> for R {
    fn from(reader: crate::R<TCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR3` writer"]
pub struct W(crate::W<TCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR3_SPEC>;
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
impl core::convert::From<crate::W<TCR3_SPEC>> for W {
    fn from(writer: crate::W<TCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcr3_counter` reader - "]
pub struct TCR3_COUNTER_R(crate::FieldReader<u32, u32>);
impl TCR3_COUNTER_R {
    pub(crate) fn new(bits: u32) -> Self {
        TCR3_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCR3_COUNTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tcr3_counter` writer - "]
pub struct TCR3_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR3_COUNTER_W<'a> {
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
    pub fn tcr3_counter(&self) -> TCR3_COUNTER_R {
        TCR3_COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_counter(&mut self) -> TCR3_COUNTER_W {
        TCR3_COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr3](index.html) module"]
pub struct TCR3_SPEC;
impl crate::RegisterSpec for TCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr3::R](R) reader structure"]
impl crate::Readable for TCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr3::W](W) writer structure"]
impl crate::Writable for TCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR3 to value 0"]
impl crate::Resettable for TCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
