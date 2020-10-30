#[doc = "Register `aon` reader"]
pub struct R(crate::R<AON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AON_SPEC>> for R {
    fn from(reader: crate::R<AON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon` writer"]
pub struct W(crate::W<AON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_SPEC>;
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
impl core::convert::From<crate::W<AON_SPEC>> for W {
    fn from(writer: crate::W<AON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sw_pu_ldo11_rt` reader - "]
pub struct SW_PU_LDO11_RT_R(crate::FieldReader<bool, bool>);
impl SW_PU_LDO11_RT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_PU_LDO11_RT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_PU_LDO11_RT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_pu_ldo11_rt` writer - "]
pub struct SW_PU_LDO11_RT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PU_LDO11_RT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ldo11_rt_pulldown_sel` reader - "]
pub struct LDO11_RT_PULLDOWN_SEL_R(crate::FieldReader<bool, bool>);
impl LDO11_RT_PULLDOWN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11_RT_PULLDOWN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11_RT_PULLDOWN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11_rt_pulldown_sel` writer - "]
pub struct LDO11_RT_PULLDOWN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11_RT_PULLDOWN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ldo11_rt_pulldown` reader - "]
pub struct LDO11_RT_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl LDO11_RT_PULLDOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11_RT_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11_RT_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11_rt_pulldown` writer - "]
pub struct LDO11_RT_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11_RT_PULLDOWN_W<'a> {
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
#[doc = "Field `pu_aon_dc_tbuf` reader - "]
pub struct PU_AON_DC_TBUF_R(crate::FieldReader<bool, bool>);
impl PU_AON_DC_TBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_AON_DC_TBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_AON_DC_TBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_aon_dc_tbuf` writer - "]
pub struct PU_AON_DC_TBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_AON_DC_TBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `aon_resv` reader - "]
pub struct AON_RESV_R(crate::FieldReader<u8, u8>);
impl AON_RESV_R {
    pub(crate) fn new(bits: u8) -> Self {
        AON_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_RESV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `aon_resv` writer - "]
pub struct AON_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&self) -> SW_PU_LDO11_RT_R {
        SW_PU_LDO11_RT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&self) -> LDO11_RT_PULLDOWN_SEL_R {
        LDO11_RT_PULLDOWN_SEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&self) -> LDO11_RT_PULLDOWN_R {
        LDO11_RT_PULLDOWN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&self) -> PU_AON_DC_TBUF_R {
        PU_AON_DC_TBUF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&self) -> AON_RESV_R {
        AON_RESV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&mut self) -> SW_PU_LDO11_RT_W {
        SW_PU_LDO11_RT_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&mut self) -> LDO11_RT_PULLDOWN_SEL_W {
        LDO11_RT_PULLDOWN_SEL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&mut self) -> LDO11_RT_PULLDOWN_W {
        LDO11_RT_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&mut self) -> PU_AON_DC_TBUF_W {
        PU_AON_DC_TBUF_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&mut self) -> AON_RESV_W {
        AON_RESV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon](index.html) module"]
pub struct AON_SPEC;
impl crate::RegisterSpec for AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon::R](R) reader structure"]
impl crate::Readable for AON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon::W](W) writer structure"]
impl crate::Writable for AON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets aon to value 0"]
impl crate::Resettable for AON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
