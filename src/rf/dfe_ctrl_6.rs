#[doc = "Register `dfe_ctrl_6` reader"]
pub struct R(crate::R<DFE_CTRL_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_6_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_6` writer"]
pub struct W(crate::W<DFE_CTRL_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_6_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_6_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pm_in_sel` reader - "]
pub struct RX_PM_IN_SEL_R(crate::FieldReader<u8, u8>);
impl RX_PM_IN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_PM_IN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_IN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_in_sel` writer - "]
pub struct RX_PM_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_IN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `rx_pm_en` reader - "]
pub struct RX_PM_EN_R(crate::FieldReader<bool, bool>);
impl RX_PM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_PM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_en` writer - "]
pub struct RX_PM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `rx_pm_done` reader - "]
pub struct RX_PM_DONE_R(crate::FieldReader<bool, bool>);
impl RX_PM_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_PM_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_done` writer - "]
pub struct RX_PM_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `rx_pm_freqshift_en` reader - "]
pub struct RX_PM_FREQSHIFT_EN_R(crate::FieldReader<bool, bool>);
impl RX_PM_FREQSHIFT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_PM_FREQSHIFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_FREQSHIFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_freqshift_en` writer - "]
pub struct RX_PM_FREQSHIFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_FREQSHIFT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `rx_pm_freqshift_cw` reader - "]
pub struct RX_PM_FREQSHIFT_CW_R(crate::FieldReader<u32, u32>);
impl RX_PM_FREQSHIFT_CW_R {
    pub(crate) fn new(bits: u32) -> Self {
        RX_PM_FREQSHIFT_CW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_FREQSHIFT_CW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_freqshift_cw` writer - "]
pub struct RX_PM_FREQSHIFT_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_FREQSHIFT_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&self) -> RX_PM_IN_SEL_R {
        RX_PM_IN_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&self) -> RX_PM_EN_R {
        RX_PM_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&self) -> RX_PM_DONE_R {
        RX_PM_DONE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&self) -> RX_PM_FREQSHIFT_EN_R {
        RX_PM_FREQSHIFT_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&self) -> RX_PM_FREQSHIFT_CW_R {
        RX_PM_FREQSHIFT_CW_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&mut self) -> RX_PM_IN_SEL_W {
        RX_PM_IN_SEL_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&mut self) -> RX_PM_EN_W {
        RX_PM_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&mut self) -> RX_PM_DONE_W {
        RX_PM_DONE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&mut self) -> RX_PM_FREQSHIFT_EN_W {
        RX_PM_FREQSHIFT_EN_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&mut self) -> RX_PM_FREQSHIFT_CW_W {
        RX_PM_FREQSHIFT_CW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_6](index.html) module"]
pub struct DFE_CTRL_6_SPEC;
impl crate::RegisterSpec for DFE_CTRL_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_6::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_6::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_6 to value 0"]
impl crate::Resettable for DFE_CTRL_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
