#[doc = "Register `cks_config` reader"]
pub struct R(crate::R<CKS_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKS_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CKS_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<CKS_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cks_config` writer"]
pub struct W(crate::W<CKS_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKS_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<CKS_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<CKS_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_cks_byte_swap` reader - Endianness (0: little endian, 1: big endian)"]
pub struct CR_CKS_BYTE_SWAP_R(crate::FieldReader<bool, bool>);
impl CR_CKS_BYTE_SWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_CKS_BYTE_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_CKS_BYTE_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_cks_byte_swap` writer - Endianness (0: little endian, 1: big endian)"]
pub struct CR_CKS_BYTE_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_CKS_BYTE_SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `cr_cks_clr` reader - "]
pub struct CR_CKS_CLR_R(crate::FieldReader<bool, bool>);
impl CR_CKS_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_CKS_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_CKS_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_cks_clr` writer - "]
pub struct CR_CKS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_CKS_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Endianness (0: little endian, 1: big endian)"]
    #[inline(always)]
    pub fn cr_cks_byte_swap(&self) -> CR_CKS_BYTE_SWAP_R {
        CR_CKS_BYTE_SWAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_cks_clr(&self) -> CR_CKS_CLR_R {
        CR_CKS_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Endianness (0: little endian, 1: big endian)"]
    #[inline(always)]
    pub fn cr_cks_byte_swap(&mut self) -> CR_CKS_BYTE_SWAP_W {
        CR_CKS_BYTE_SWAP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_cks_clr(&mut self) -> CR_CKS_CLR_W {
        CR_CKS_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cks_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cks_config](index.html) module"]
pub struct CKS_CONFIG_SPEC;
impl crate::RegisterSpec for CKS_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cks_config::R](R) reader structure"]
impl crate::Readable for CKS_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cks_config::W](W) writer structure"]
impl crate::Writable for CKS_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cks_config to value 0"]
impl crate::Resettable for CKS_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
