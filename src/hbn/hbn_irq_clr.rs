#[doc = "Register `HBN_IRQ_CLR` writer"]
pub struct W(crate::W<HBN_IRQ_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_IRQ_CLR_SPEC>;
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
impl core::convert::From<crate::W<HBN_IRQ_CLR_SPEC>> for W {
    fn from(writer: crate::W<HBN_IRQ_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq_clr` writer - "]
pub struct IRQ_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IRQ_CLR_W {
        IRQ_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_IRQ_CLR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_clr](index.html) module"]
pub struct HBN_IRQ_CLR_SPEC;
impl crate::RegisterSpec for HBN_IRQ_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hbn_irq_clr::W](W) writer structure"]
impl crate::Writable for HBN_IRQ_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_IRQ_CLR to value 0"]
impl crate::Resettable for HBN_IRQ_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
