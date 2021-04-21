#[doc = "Register `uart_bit_prd` reader"]
pub struct R(crate::R<UART_BIT_PRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_BIT_PRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_BIT_PRD_SPEC>> for R {
    fn from(reader: crate::R<UART_BIT_PRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_bit_prd` writer"]
pub struct W(crate::W<UART_BIT_PRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_BIT_PRD_SPEC>;
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
impl core::convert::From<crate::W<UART_BIT_PRD_SPEC>> for W {
    fn from(writer: crate::W<UART_BIT_PRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_bit_prd` reader - "]
pub struct CR_URX_BIT_PRD_R(crate::FieldReader<u16, u16>);
impl CR_URX_BIT_PRD_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_URX_BIT_PRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_BIT_PRD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_bit_prd` writer - "]
pub struct CR_URX_BIT_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_BIT_PRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `cr_utx_bit_prd` reader - "]
pub struct CR_UTX_BIT_PRD_R(crate::FieldReader<u16, u16>);
impl CR_UTX_BIT_PRD_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_UTX_BIT_PRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_BIT_PRD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_bit_prd` writer - "]
pub struct CR_UTX_BIT_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_BIT_PRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_bit_prd(&self) -> CR_URX_BIT_PRD_R {
        CR_URX_BIT_PRD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&self) -> CR_UTX_BIT_PRD_R {
        CR_UTX_BIT_PRD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_bit_prd(&mut self) -> CR_URX_BIT_PRD_W {
        CR_URX_BIT_PRD_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&mut self) -> CR_UTX_BIT_PRD_W {
        CR_UTX_BIT_PRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_bit_prd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_bit_prd](index.html) module"]
pub struct UART_BIT_PRD_SPEC;
impl crate::RegisterSpec for UART_BIT_PRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_bit_prd::R](R) reader structure"]
impl crate::Readable for UART_BIT_PRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_bit_prd::W](W) writer structure"]
impl crate::Writable for UART_BIT_PRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_bit_prd to value 0x00ff_00ff"]
impl crate::Resettable for UART_BIT_PRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_00ff
    }
}
