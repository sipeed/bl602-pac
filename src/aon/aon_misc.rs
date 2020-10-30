#[doc = "Register `aon_misc` reader"]
pub struct R(crate::R<AON_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AON_MISC_SPEC>> for R {
    fn from(reader: crate::R<AON_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon_misc` writer"]
pub struct W(crate::W<AON_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_MISC_SPEC>;
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
impl core::convert::From<crate::W<AON_MISC_SPEC>> for W {
    fn from(writer: crate::W<AON_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sw_wb_en_aon` reader - "]
pub struct SW_WB_EN_AON_R(crate::FieldReader<bool, bool>);
impl SW_WB_EN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_WB_EN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_WB_EN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_wb_en_aon` writer - "]
pub struct SW_WB_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WB_EN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `sw_soc_en_aon` reader - "]
pub struct SW_SOC_EN_AON_R(crate::FieldReader<bool, bool>);
impl SW_SOC_EN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SOC_EN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SOC_EN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_soc_en_aon` writer - "]
pub struct SW_SOC_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SOC_EN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&self) -> SW_WB_EN_AON_R {
        SW_WB_EN_AON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&self) -> SW_SOC_EN_AON_R {
        SW_SOC_EN_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&mut self) -> SW_WB_EN_AON_W {
        SW_WB_EN_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&mut self) -> SW_SOC_EN_AON_W {
        SW_SOC_EN_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon_misc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_misc](index.html) module"]
pub struct AON_MISC_SPEC;
impl crate::RegisterSpec for AON_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon_misc::R](R) reader structure"]
impl crate::Readable for AON_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon_misc::W](W) writer structure"]
impl crate::Writable for AON_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets aon_misc to value 0"]
impl crate::Resettable for AON_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
