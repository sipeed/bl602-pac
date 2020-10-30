#[doc = "Register `se_trng_0_ctrl_2` reader"]
pub struct R(crate::R<SE_TRNG_0_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_TRNG_0_CTRL_2_SPEC>> for R {
    fn from(reader: crate::R<SE_TRNG_0_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_ctrl_2` writer"]
pub struct W(crate::W<SE_TRNG_0_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_CTRL_2_SPEC>;
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
impl core::convert::From<crate::W<SE_TRNG_0_CTRL_2_SPEC>> for W {
    fn from(writer: crate::W<SE_TRNG_0_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_reseed_n_msb` reader - "]
pub struct SE_TRNG_0_RESEED_N_MSB_R(crate::FieldReader<u16, u16>);
impl SE_TRNG_0_RESEED_N_MSB_R {
    pub(crate) fn new(bits: u16) -> Self {
        SE_TRNG_0_RESEED_N_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_RESEED_N_MSB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_reseed_n_msb` writer - "]
pub struct SE_TRNG_0_RESEED_N_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_RESEED_N_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_msb(&self) -> SE_TRNG_0_RESEED_N_MSB_R {
        SE_TRNG_0_RESEED_N_MSB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_msb(&mut self) -> SE_TRNG_0_RESEED_N_MSB_W {
        SE_TRNG_0_RESEED_N_MSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_2](index.html) module"]
pub struct SE_TRNG_0_CTRL_2_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_ctrl_2::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_2::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_2 to value 0"]
impl crate::Resettable for SE_TRNG_0_CTRL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
