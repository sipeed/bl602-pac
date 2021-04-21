#[doc = "Register `ef_if_0_manual` reader"]
pub struct R(crate::R<EF_IF_0_MANUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_0_MANUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_IF_0_MANUAL_SPEC>> for R {
    fn from(reader: crate::R<EF_IF_0_MANUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_0_manual` writer"]
pub struct W(crate::W<EF_IF_0_MANUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_0_MANUAL_SPEC>;
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
impl core::convert::From<crate::W<EF_IF_0_MANUAL_SPEC>> for W {
    fn from(writer: crate::W<EF_IF_0_MANUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_prot_code_manual` reader - "]
pub struct EF_IF_PROT_CODE_MANUAL_R(crate::FieldReader<u8, u8>);
impl EF_IF_PROT_CODE_MANUAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_PROT_CODE_MANUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_PROT_CODE_MANUAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_prot_code_manual` writer - "]
pub struct EF_IF_PROT_CODE_MANUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PROT_CODE_MANUAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ef_if_0_q` reader - "]
pub struct EF_IF_0_Q_R(crate::FieldReader<u8, u8>);
impl EF_IF_0_Q_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_0_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_csb` reader - "]
pub struct EF_IF_CSB_R(crate::FieldReader<bool, bool>);
impl EF_IF_CSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_CSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_csb` writer - "]
pub struct EF_IF_CSB_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ef_if_load` reader - "]
pub struct EF_IF_LOAD_R(crate::FieldReader<bool, bool>);
impl EF_IF_LOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_load` writer - "]
pub struct EF_IF_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_LOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ef_if_pgenb` reader - "]
pub struct EF_IF_PGENB_R(crate::FieldReader<bool, bool>);
impl EF_IF_PGENB_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_PGENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_PGENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_pgenb` writer - "]
pub struct EF_IF_PGENB_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PGENB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ef_if_strobe` reader - "]
pub struct EF_IF_STROBE_R(crate::FieldReader<bool, bool>);
impl EF_IF_STROBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_STROBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_STROBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_strobe` writer - "]
pub struct EF_IF_STROBE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_STROBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `ef_if_ps` reader - "]
pub struct EF_IF_PS_R(crate::FieldReader<bool, bool>);
impl EF_IF_PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_PS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_ps` writer - "]
pub struct EF_IF_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ef_if_pd` reader - "]
pub struct EF_IF_PD_R(crate::FieldReader<bool, bool>);
impl EF_IF_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_pd` writer - "]
pub struct EF_IF_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ef_if_a` reader - "]
pub struct EF_IF_A_R(crate::FieldReader<u16, u16>);
impl EF_IF_A_R {
    pub(crate) fn new(bits: u16) -> Self {
        EF_IF_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_A_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_a` writer - "]
pub struct EF_IF_A_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&self) -> EF_IF_PROT_CODE_MANUAL_R {
        EF_IF_PROT_CODE_MANUAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ef_if_0_q(&self) -> EF_IF_0_Q_R {
        EF_IF_0_Q_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&self) -> EF_IF_CSB_R {
        EF_IF_CSB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&self) -> EF_IF_LOAD_R {
        EF_IF_LOAD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&self) -> EF_IF_PGENB_R {
        EF_IF_PGENB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&self) -> EF_IF_STROBE_R {
        EF_IF_STROBE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&self) -> EF_IF_PS_R {
        EF_IF_PS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&self) -> EF_IF_PD_R {
        EF_IF_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&self) -> EF_IF_A_R {
        EF_IF_A_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&mut self) -> EF_IF_PROT_CODE_MANUAL_W {
        EF_IF_PROT_CODE_MANUAL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&mut self) -> EF_IF_CSB_W {
        EF_IF_CSB_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&mut self) -> EF_IF_LOAD_W {
        EF_IF_LOAD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&mut self) -> EF_IF_PGENB_W {
        EF_IF_PGENB_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&mut self) -> EF_IF_STROBE_W {
        EF_IF_STROBE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&mut self) -> EF_IF_PS_W {
        EF_IF_PS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&mut self) -> EF_IF_PD_W {
        EF_IF_PD_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&mut self) -> EF_IF_A_W {
        EF_IF_A_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_0_manual.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_0_manual](index.html) module"]
pub struct EF_IF_0_MANUAL_SPEC;
impl crate::RegisterSpec for EF_IF_0_MANUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_0_manual::R](R) reader structure"]
impl crate::Readable for EF_IF_0_MANUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_0_manual::W](W) writer structure"]
impl crate::Writable for EF_IF_0_MANUAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_if_0_manual to value 0xe400"]
impl crate::Resettable for EF_IF_0_MANUAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe400
    }
}
