#[doc = "Register `CPU_CLK_CFG` reader"]
pub struct R(crate::R<CPU_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPU_CLK_CFG_SPEC>> for R {
    fn from(reader: crate::R<CPU_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CLK_CFG` writer"]
pub struct W(crate::W<CPU_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CLK_CFG_SPEC>;
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
impl core::convert::From<crate::W<CPU_CLK_CFG_SPEC>> for W {
    fn from(writer: crate::W<CPU_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `debug_ndreset_gate` reader - "]
pub struct DEBUG_NDRESET_GATE_R(crate::FieldReader<bool, bool>);
impl DEBUG_NDRESET_GATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBUG_NDRESET_GATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_NDRESET_GATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `debug_ndreset_gate` writer - "]
pub struct DEBUG_NDRESET_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_NDRESET_GATE_W<'a> {
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
#[doc = "Field `cpu_rtc_sel` reader - "]
pub struct CPU_RTC_SEL_R(crate::FieldReader<bool, bool>);
impl CPU_RTC_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_RTC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_RTC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu_rtc_sel` writer - "]
pub struct CPU_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RTC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `cpu_rtc_en` reader - "]
pub struct CPU_RTC_EN_R(crate::FieldReader<bool, bool>);
impl CPU_RTC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_RTC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_RTC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu_rtc_en` writer - "]
pub struct CPU_RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RTC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cpu_rtc_div` reader - "]
pub struct CPU_RTC_DIV_R(crate::FieldReader<u32, u32>);
impl CPU_RTC_DIV_R {
    pub(crate) fn new(bits: u32) -> Self {
        CPU_RTC_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_RTC_DIV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu_rtc_div` writer - "]
pub struct CPU_RTC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RTC_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DEBUG_NDRESET_GATE_R {
        DEBUG_NDRESET_GATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&self) -> CPU_RTC_SEL_R {
        CPU_RTC_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&self) -> CPU_RTC_EN_R {
        CPU_RTC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&self) -> CPU_RTC_DIV_R {
        CPU_RTC_DIV_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&mut self) -> DEBUG_NDRESET_GATE_W {
        DEBUG_NDRESET_GATE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&mut self) -> CPU_RTC_SEL_W {
        CPU_RTC_SEL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&mut self) -> CPU_RTC_EN_W {
        CPU_RTC_EN_W { w: self }
    }
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&mut self) -> CPU_RTC_DIV_W {
        CPU_RTC_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU_CLK_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clk_cfg](index.html) module"]
pub struct CPU_CLK_CFG_SPEC;
impl crate::RegisterSpec for CPU_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_clk_cfg::R](R) reader structure"]
impl crate::Readable for CPU_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_clk_cfg::W](W) writer structure"]
impl crate::Writable for CPU_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_CLK_CFG to value 0"]
impl crate::Resettable for CPU_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
