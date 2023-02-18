#[doc = "Register `se_aes_0_key_sel_1` reader"]
pub struct R(crate::R<SE_AES_0_KEY_SEL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_KEY_SEL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_AES_0_KEY_SEL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_AES_0_KEY_SEL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_key_sel_1` writer"]
pub struct W(crate::W<SE_AES_0_KEY_SEL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_KEY_SEL_1_SPEC>;
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
impl From<crate::W<SE_AES_0_KEY_SEL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_AES_0_KEY_SEL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_0_key_sel_1` reader - "]
pub type SE_AES_0_KEY_SEL_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `se_aes_0_key_sel_1` writer - "]
pub type SE_AES_0_KEY_SEL_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_AES_0_KEY_SEL_1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_1(&self) -> SE_AES_0_KEY_SEL_1_R {
        SE_AES_0_KEY_SEL_1_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn se_aes_0_key_sel_1(&mut self) -> SE_AES_0_KEY_SEL_1_W<0> {
        SE_AES_0_KEY_SEL_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_key_sel_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_sel_1](index.html) module"]
pub struct SE_AES_0_KEY_SEL_1_SPEC;
impl crate::RegisterSpec for SE_AES_0_KEY_SEL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_key_sel_1::R](R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_SEL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_sel_1::W](W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_SEL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_aes_0_key_sel_1 to value 0"]
impl crate::Resettable for SE_AES_0_KEY_SEL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
