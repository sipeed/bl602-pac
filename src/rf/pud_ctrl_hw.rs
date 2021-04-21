#[doc = "Register `pud_ctrl_hw` reader"]
pub struct R(crate::R<PUD_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUD_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PUD_CTRL_HW_SPEC>> for R {
    fn from(reader: crate::R<PUD_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pud_ctrl_hw` writer"]
pub struct W(crate::W<PUD_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUD_CTRL_HW_SPEC>;
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
impl core::convert::From<crate::W<PUD_CTRL_HW_SPEC>> for W {
    fn from(writer: crate::W<PUD_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pud_vco_hw` reader - "]
pub struct PUD_VCO_HW_R(crate::FieldReader<bool, bool>);
impl PUD_VCO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUD_VCO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUD_VCO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pud_vco_hw` writer - "]
pub struct PUD_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_VCO_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&self) -> PUD_VCO_HW_R {
        PUD_VCO_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&mut self) -> PUD_VCO_HW_W {
        PUD_VCO_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pud_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pud_ctrl_hw](index.html) module"]
pub struct PUD_CTRL_HW_SPEC;
impl crate::RegisterSpec for PUD_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pud_ctrl_hw::R](R) reader structure"]
impl crate::Readable for PUD_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pud_ctrl_hw::W](W) writer structure"]
impl crate::Writable for PUD_CTRL_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pud_ctrl_hw to value 0"]
impl crate::Resettable for PUD_CTRL_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
