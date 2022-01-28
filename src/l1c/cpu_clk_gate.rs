#[doc = "Register `cpu_clk_gate` reader"]
pub struct R(crate::R<CPU_CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_clk_gate` writer"]
pub struct W(crate::W<CPU_CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CLK_GATE_SPEC>;
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
impl From<crate::W<CPU_CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `force_e21_clock_on_2` reader - "]
pub struct FORCE_E21_CLOCK_ON_2_R(crate::FieldReader<bool, bool>);
impl FORCE_E21_CLOCK_ON_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_E21_CLOCK_ON_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_E21_CLOCK_ON_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_e21_clock_on_2` writer - "]
pub struct FORCE_E21_CLOCK_ON_2_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_E21_CLOCK_ON_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `force_e21_clock_on_1` reader - "]
pub struct FORCE_E21_CLOCK_ON_1_R(crate::FieldReader<bool, bool>);
impl FORCE_E21_CLOCK_ON_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_E21_CLOCK_ON_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_E21_CLOCK_ON_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_e21_clock_on_1` writer - "]
pub struct FORCE_E21_CLOCK_ON_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_E21_CLOCK_ON_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `force_e21_clock_on_0` reader - "]
pub struct FORCE_E21_CLOCK_ON_0_R(crate::FieldReader<bool, bool>);
impl FORCE_E21_CLOCK_ON_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_E21_CLOCK_ON_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_E21_CLOCK_ON_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_e21_clock_on_0` writer - "]
pub struct FORCE_E21_CLOCK_ON_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_E21_CLOCK_ON_0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&self) -> FORCE_E21_CLOCK_ON_2_R {
        FORCE_E21_CLOCK_ON_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&self) -> FORCE_E21_CLOCK_ON_1_R {
        FORCE_E21_CLOCK_ON_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&self) -> FORCE_E21_CLOCK_ON_0_R {
        FORCE_E21_CLOCK_ON_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&mut self) -> FORCE_E21_CLOCK_ON_2_W {
        FORCE_E21_CLOCK_ON_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&mut self) -> FORCE_E21_CLOCK_ON_1_W {
        FORCE_E21_CLOCK_ON_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&mut self) -> FORCE_E21_CLOCK_ON_0_W {
        FORCE_E21_CLOCK_ON_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_clk_gate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clk_gate](index.html) module"]
pub struct CPU_CLK_GATE_SPEC;
impl crate::RegisterSpec for CPU_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_clk_gate::R](R) reader structure"]
impl crate::Readable for CPU_CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_clk_gate::W](W) writer structure"]
impl crate::Writable for CPU_CLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cpu_clk_gate to value 0"]
impl crate::Resettable for CPU_CLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
