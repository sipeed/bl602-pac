#[doc = "Register `dfe_ctrl_1` reader"]
pub struct R(crate::R<DFE_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_1_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_1` writer"]
pub struct W(crate::W<DFE_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_1_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_1_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_dac_iq_swap` reader - "]
pub struct TX_DAC_IQ_SWAP_R(crate::FieldReader<bool, bool>);
impl TX_DAC_IQ_SWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DAC_IQ_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DAC_IQ_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dac_iq_swap` writer - "]
pub struct TX_DAC_IQ_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_IQ_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `tx_dac_dat_format` reader - "]
pub struct TX_DAC_DAT_FORMAT_R(crate::FieldReader<bool, bool>);
impl TX_DAC_DAT_FORMAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DAC_DAT_FORMAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DAC_DAT_FORMAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dac_dat_format` writer - "]
pub struct TX_DAC_DAT_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_DAT_FORMAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `tx_dac_os_q` reader - "]
pub struct TX_DAC_OS_Q_R(crate::FieldReader<u16, u16>);
impl TX_DAC_OS_Q_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_DAC_OS_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DAC_OS_Q_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dac_os_q` writer - "]
pub struct TX_DAC_OS_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_OS_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `tx_dac_os_i` reader - "]
pub struct TX_DAC_OS_I_R(crate::FieldReader<u16, u16>);
impl TX_DAC_OS_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_DAC_OS_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DAC_OS_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dac_os_i` writer - "]
pub struct TX_DAC_OS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_OS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dac_iq_swap(&self) -> TX_DAC_IQ_SWAP_R {
        TX_DAC_IQ_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_dac_dat_format(&self) -> TX_DAC_DAT_FORMAT_R {
        TX_DAC_DAT_FORMAT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tx_dac_os_q(&self) -> TX_DAC_OS_Q_R {
        TX_DAC_OS_Q_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_dac_os_i(&self) -> TX_DAC_OS_I_R {
        TX_DAC_OS_I_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dac_iq_swap(&mut self) -> TX_DAC_IQ_SWAP_W {
        TX_DAC_IQ_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_dac_dat_format(&mut self) -> TX_DAC_DAT_FORMAT_W {
        TX_DAC_DAT_FORMAT_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tx_dac_os_q(&mut self) -> TX_DAC_OS_Q_W {
        TX_DAC_OS_Q_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_dac_os_i(&mut self) -> TX_DAC_OS_I_W {
        TX_DAC_OS_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_1](index.html) module"]
pub struct DFE_CTRL_1_SPEC;
impl crate::RegisterSpec for DFE_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_1::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_1::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_1 to value 0"]
impl crate::Resettable for DFE_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
