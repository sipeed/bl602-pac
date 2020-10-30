#[doc = "Register `irrx_pw_config` reader"]
pub struct R(crate::R<IRRX_PW_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_PW_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRRX_PW_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<IRRX_PW_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_pw_config` writer"]
pub struct W(crate::W<IRRX_PW_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_PW_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<IRRX_PW_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<IRRX_PW_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irrx_end_th` reader - "]
pub struct CR_IRRX_END_TH_R(crate::FieldReader<u16, u16>);
impl CR_IRRX_END_TH_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_IRRX_END_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_END_TH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_end_th` writer - "]
pub struct CR_IRRX_END_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_END_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `cr_irrx_data_th` reader - "]
pub struct CR_IRRX_DATA_TH_R(crate::FieldReader<u16, u16>);
impl CR_IRRX_DATA_TH_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_IRRX_DATA_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_DATA_TH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_data_th` writer - "]
pub struct CR_IRRX_DATA_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_DATA_TH_W<'a> {
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
    pub fn cr_irrx_end_th(&self) -> CR_IRRX_END_TH_R {
        CR_IRRX_END_TH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&self) -> CR_IRRX_DATA_TH_R {
        CR_IRRX_DATA_TH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_irrx_end_th(&mut self) -> CR_IRRX_END_TH_W {
        CR_IRRX_END_TH_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&mut self) -> CR_IRRX_DATA_TH_W {
        CR_IRRX_DATA_TH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_pw_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_pw_config](index.html) module"]
pub struct IRRX_PW_CONFIG_SPEC;
impl crate::RegisterSpec for IRRX_PW_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_pw_config::R](R) reader structure"]
impl crate::Readable for IRRX_PW_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_pw_config::W](W) writer structure"]
impl crate::Writable for IRRX_PW_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irrx_pw_config to value 0"]
impl crate::Resettable for IRRX_PW_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
