#[doc = "Register `se_pka_0_ctrl_1` reader"]
pub struct R(crate::R<SE_PKA_0_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_PKA_0_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_PKA_0_CTRL_1_SPEC>> for R {
    fn from(reader: crate::R<SE_PKA_0_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_pka_0_ctrl_1` writer"]
pub struct W(crate::W<SE_PKA_0_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_PKA_0_CTRL_1_SPEC>;
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
impl core::convert::From<crate::W<SE_PKA_0_CTRL_1_SPEC>> for W {
    fn from(writer: crate::W<SE_PKA_0_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_pka_0_hbypass` reader - "]
pub struct SE_PKA_0_HBYPASS_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_HBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_HBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_HBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_hbypass` writer - "]
pub struct SE_PKA_0_HBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_HBYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `se_pka_0_hburst` reader - "]
pub struct SE_PKA_0_HBURST_R(crate::FieldReader<u8, u8>);
impl SE_PKA_0_HBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_PKA_0_HBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_HBURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_hburst` writer - "]
pub struct SE_PKA_0_HBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_HBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&self) -> SE_PKA_0_HBYPASS_R {
        SE_PKA_0_HBYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&self) -> SE_PKA_0_HBURST_R {
        SE_PKA_0_HBURST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&mut self) -> SE_PKA_0_HBYPASS_W {
        SE_PKA_0_HBYPASS_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&mut self) -> SE_PKA_0_HBURST_W {
        SE_PKA_0_HBURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_pka_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_1](index.html) module"]
pub struct SE_PKA_0_CTRL_1_SPEC;
impl crate::RegisterSpec for SE_PKA_0_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_pka_0_ctrl_1::R](R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_1::W](W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_pka_0_ctrl_1 to value 0"]
impl crate::Resettable for SE_PKA_0_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
