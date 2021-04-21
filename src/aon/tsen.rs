#[doc = "Register `tsen` reader"]
pub struct R(crate::R<TSEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TSEN_SPEC>> for R {
    fn from(reader: crate::R<TSEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tsen` writer"]
pub struct W(crate::W<TSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEN_SPEC>;
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
impl core::convert::From<crate::W<TSEN_SPEC>> for W {
    fn from(writer: crate::W<TSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `xtal_rdy_int_sel_aon` reader - "]
pub struct XTAL_RDY_INT_SEL_AON_R(crate::FieldReader<u8, u8>);
impl XTAL_RDY_INT_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTAL_RDY_INT_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_RDY_INT_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal_rdy_int_sel_aon` writer - "]
pub struct XTAL_RDY_INT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_RDY_INT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `xtal_inn_cfg_en_aon` reader - "]
pub struct XTAL_INN_CFG_EN_AON_R(crate::FieldReader<bool, bool>);
impl XTAL_INN_CFG_EN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_INN_CFG_EN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_INN_CFG_EN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal_inn_cfg_en_aon` writer - "]
pub struct XTAL_INN_CFG_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_INN_CFG_EN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `xtal_rdy` reader - "]
pub struct XTAL_RDY_R(crate::FieldReader<bool, bool>);
impl XTAL_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tsen_refcode_rfcal` reader - "]
pub struct TSEN_REFCODE_RFCAL_R(crate::FieldReader<u16, u16>);
impl TSEN_REFCODE_RFCAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSEN_REFCODE_RFCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_REFCODE_RFCAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tsen_refcode_rfcal` writer - "]
pub struct TSEN_REFCODE_RFCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_REFCODE_RFCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `tsen_refcode_corner` reader - "]
pub struct TSEN_REFCODE_CORNER_R(crate::FieldReader<u16, u16>);
impl TSEN_REFCODE_CORNER_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSEN_REFCODE_CORNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_REFCODE_CORNER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tsen_refcode_corner` writer - "]
pub struct TSEN_REFCODE_CORNER_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_REFCODE_CORNER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&self) -> XTAL_RDY_INT_SEL_AON_R {
        XTAL_RDY_INT_SEL_AON_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&self) -> XTAL_INN_CFG_EN_AON_R {
        XTAL_INN_CFG_EN_AON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_rdy(&self) -> XTAL_RDY_R {
        XTAL_RDY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&self) -> TSEN_REFCODE_RFCAL_R {
        TSEN_REFCODE_RFCAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&self) -> TSEN_REFCODE_CORNER_R {
        TSEN_REFCODE_CORNER_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&mut self) -> XTAL_RDY_INT_SEL_AON_W {
        XTAL_RDY_INT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&mut self) -> XTAL_INN_CFG_EN_AON_W {
        XTAL_INN_CFG_EN_AON_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&mut self) -> TSEN_REFCODE_RFCAL_W {
        TSEN_REFCODE_RFCAL_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&mut self) -> TSEN_REFCODE_CORNER_W {
        TSEN_REFCODE_CORNER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tsen.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsen](index.html) module"]
pub struct TSEN_SPEC;
impl crate::RegisterSpec for TSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsen::R](R) reader structure"]
impl crate::Readable for TSEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsen::W](W) writer structure"]
impl crate::Writable for TSEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tsen to value 0x78ff_08ff"]
impl crate::Resettable for TSEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x78ff_08ff
    }
}
