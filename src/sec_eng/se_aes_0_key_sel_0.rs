#[doc = "Register `se_aes_0_key_sel_0` reader"]
pub struct R(crate::R<SE_AES_0_KEY_SEL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_KEY_SEL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_AES_0_KEY_SEL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_AES_0_KEY_SEL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_key_sel_0` writer"]
pub struct W(crate::W<SE_AES_0_KEY_SEL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_KEY_SEL_0_SPEC>;
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
impl From<crate::W<SE_AES_0_KEY_SEL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_AES_0_KEY_SEL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_0_key_sel_0` reader - "]
pub struct SE_AES_0_KEY_SEL_0_R(crate::FieldReader<u8, u8>);
impl SE_AES_0_KEY_SEL_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_AES_0_KEY_SEL_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_KEY_SEL_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_0_key_sel_0` writer - "]
pub struct SE_AES_0_KEY_SEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_KEY_SEL_0_W<'a> {
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
    pub fn se_aes_0_key_sel_0(&self) -> SE_AES_0_KEY_SEL_0_R {
        SE_AES_0_KEY_SEL_0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_0(&mut self) -> SE_AES_0_KEY_SEL_0_W {
        SE_AES_0_KEY_SEL_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_key_sel_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_sel_0](index.html) module"]
pub struct SE_AES_0_KEY_SEL_0_SPEC;
impl crate::RegisterSpec for SE_AES_0_KEY_SEL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_key_sel_0::R](R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_SEL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_sel_0::W](W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_SEL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_aes_0_key_sel_0 to value 0"]
impl crate::Resettable for SE_AES_0_KEY_SEL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
