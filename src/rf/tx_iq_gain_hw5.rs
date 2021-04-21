#[doc = "Register `tx_iq_gain_hw5` reader"]
pub struct R(crate::R<TX_IQ_GAIN_HW5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_IQ_GAIN_HW5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TX_IQ_GAIN_HW5_SPEC>> for R {
    fn from(reader: crate::R<TX_IQ_GAIN_HW5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tx_iq_gain_hw5` writer"]
pub struct W(crate::W<TX_IQ_GAIN_HW5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_IQ_GAIN_HW5_SPEC>;
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
impl core::convert::From<crate::W<TX_IQ_GAIN_HW5_SPEC>> for W {
    fn from(writer: crate::W<TX_IQ_GAIN_HW5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_iq_gain_comp_gc5` reader - "]
pub struct TX_IQ_GAIN_COMP_GC5_R(crate::FieldReader<u16, u16>);
impl TX_IQ_GAIN_COMP_GC5_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_IQ_GAIN_COMP_GC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IQ_GAIN_COMP_GC5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_iq_gain_comp_gc5` writer - "]
pub struct TX_IQ_GAIN_COMP_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQ_GAIN_COMP_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `tx_iq_phase_comp_gc5` reader - "]
pub struct TX_IQ_PHASE_COMP_GC5_R(crate::FieldReader<u16, u16>);
impl TX_IQ_PHASE_COMP_GC5_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_IQ_PHASE_COMP_GC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IQ_PHASE_COMP_GC5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_iq_phase_comp_gc5` writer - "]
pub struct TX_IQ_PHASE_COMP_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQ_PHASE_COMP_GC5_W<'a> {
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
    pub fn tx_iq_gain_comp_gc5(&self) -> TX_IQ_GAIN_COMP_GC5_R {
        TX_IQ_GAIN_COMP_GC5_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc5(&self) -> TX_IQ_PHASE_COMP_GC5_R {
        TX_IQ_PHASE_COMP_GC5_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc5(&mut self) -> TX_IQ_GAIN_COMP_GC5_W {
        TX_IQ_GAIN_COMP_GC5_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc5(&mut self) -> TX_IQ_PHASE_COMP_GC5_W {
        TX_IQ_PHASE_COMP_GC5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tx_iq_gain_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw5](index.html) module"]
pub struct TX_IQ_GAIN_HW5_SPEC;
impl crate::RegisterSpec for TX_IQ_GAIN_HW5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_iq_gain_hw5::R](R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw5::W](W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tx_iq_gain_hw5 to value 0"]
impl crate::Resettable for TX_IQ_GAIN_HW5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
