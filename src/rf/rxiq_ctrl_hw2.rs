#[doc = "Register `rxiq_ctrl_hw2` reader"]
pub struct R(crate::R<RXIQ_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIQ_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RXIQ_CTRL_HW2_SPEC>> for R {
    fn from(reader: crate::R<RXIQ_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxiq_ctrl_hw2` writer"]
pub struct W(crate::W<RXIQ_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIQ_CTRL_HW2_SPEC>;
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
impl core::convert::From<crate::W<RXIQ_CTRL_HW2_SPEC>> for W {
    fn from(writer: crate::W<RXIQ_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_iq_gain_comp_gc1` reader - "]
pub struct RX_IQ_GAIN_COMP_GC1_R(crate::FieldReader<u16, u16>);
impl RX_IQ_GAIN_COMP_GC1_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_IQ_GAIN_COMP_GC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_IQ_GAIN_COMP_GC1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_iq_gain_comp_gc1` writer - "]
pub struct RX_IQ_GAIN_COMP_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IQ_GAIN_COMP_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `rx_iq_phase_comp_gc1` reader - "]
pub struct RX_IQ_PHASE_COMP_GC1_R(crate::FieldReader<u16, u16>);
impl RX_IQ_PHASE_COMP_GC1_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_IQ_PHASE_COMP_GC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_IQ_PHASE_COMP_GC1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_iq_phase_comp_gc1` writer - "]
pub struct RX_IQ_PHASE_COMP_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IQ_PHASE_COMP_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc1(&self) -> RX_IQ_GAIN_COMP_GC1_R {
        RX_IQ_GAIN_COMP_GC1_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc1(&self) -> RX_IQ_PHASE_COMP_GC1_R {
        RX_IQ_PHASE_COMP_GC1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc1(&mut self) -> RX_IQ_GAIN_COMP_GC1_W {
        RX_IQ_GAIN_COMP_GC1_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc1(&mut self) -> RX_IQ_PHASE_COMP_GC1_W {
        RX_IQ_PHASE_COMP_GC1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rxiq_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxiq_ctrl_hw2](index.html) module"]
pub struct RXIQ_CTRL_HW2_SPEC;
impl crate::RegisterSpec for RXIQ_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxiq_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for RXIQ_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxiq_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for RXIQ_CTRL_HW2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rxiq_ctrl_hw2 to value 0"]
impl crate::Resettable for RXIQ_CTRL_HW2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
