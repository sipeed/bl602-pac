#[doc = "Register `HBN_BOR_CFG` reader"]
pub struct R(crate::R<HBN_BOR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_BOR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HBN_BOR_CFG_SPEC>> for R {
    fn from(reader: crate::R<HBN_BOR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_BOR_CFG` writer"]
pub struct W(crate::W<HBN_BOR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_BOR_CFG_SPEC>;
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
impl core::convert::From<crate::W<HBN_BOR_CFG_SPEC>> for W {
    fn from(writer: crate::W<HBN_BOR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `r_bor_out` reader - "]
pub struct R_BOR_OUT_R(crate::FieldReader<bool, bool>);
impl R_BOR_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        R_BOR_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_BOR_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_bor` reader - "]
pub struct PU_BOR_R(crate::FieldReader<bool, bool>);
impl PU_BOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_BOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_BOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_bor` writer - "]
pub struct PU_BOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_BOR_W<'a> {
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
#[doc = "Field `bor_vth` reader - "]
pub struct BOR_VTH_R(crate::FieldReader<bool, bool>);
impl BOR_VTH_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOR_VTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOR_VTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bor_vth` writer - "]
pub struct BOR_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_VTH_W<'a> {
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
#[doc = "Field `bor_sel` reader - "]
pub struct BOR_SEL_R(crate::FieldReader<bool, bool>);
impl BOR_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bor_sel` writer - "]
pub struct BOR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_SEL_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_bor_out(&self) -> R_BOR_OUT_R {
        R_BOR_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&self) -> PU_BOR_R {
        PU_BOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&self) -> BOR_VTH_R {
        BOR_VTH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&self) -> BOR_SEL_R {
        BOR_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&mut self) -> PU_BOR_W {
        PU_BOR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&mut self) -> BOR_VTH_W {
        BOR_VTH_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&mut self) -> BOR_SEL_W {
        BOR_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_BOR_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_bor_cfg](index.html) module"]
pub struct HBN_BOR_CFG_SPEC;
impl crate::RegisterSpec for HBN_BOR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_bor_cfg::R](R) reader structure"]
impl crate::Readable for HBN_BOR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_bor_cfg::W](W) writer structure"]
impl crate::Writable for HBN_BOR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_BOR_CFG to value 0x02"]
impl crate::Resettable for HBN_BOR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
