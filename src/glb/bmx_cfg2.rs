#[doc = "Register `bmx_cfg2` reader"]
pub struct R(crate::R<BMX_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMX_CFG2_SPEC>> for R {
    fn from(reader: crate::R<BMX_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg2` writer"]
pub struct W(crate::W<BMX_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG2_SPEC>;
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
impl core::convert::From<crate::W<BMX_CFG2_SPEC>> for W {
    fn from(writer: crate::W<BMX_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bmx_dbg_sel` reader - "]
pub struct BMX_DBG_SEL_R(crate::FieldReader<u8, u8>);
impl BMX_DBG_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMX_DBG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_DBG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_dbg_sel` writer - "]
pub struct BMX_DBG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_DBG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `bmx_err_tz` reader - "]
pub struct BMX_ERR_TZ_R(crate::FieldReader<bool, bool>);
impl BMX_ERR_TZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMX_ERR_TZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_ERR_TZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_err_tz` writer - "]
pub struct BMX_ERR_TZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_TZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `bmx_err_dec` reader - "]
pub struct BMX_ERR_DEC_R(crate::FieldReader<bool, bool>);
impl BMX_ERR_DEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMX_ERR_DEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_ERR_DEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_err_dec` writer - "]
pub struct BMX_ERR_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `bmx_err_addr_dis` reader - "]
pub struct BMX_ERR_ADDR_DIS_R(crate::FieldReader<bool, bool>);
impl BMX_ERR_ADDR_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMX_ERR_ADDR_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_ERR_ADDR_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_err_addr_dis` writer - "]
pub struct BMX_ERR_ADDR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_ADDR_DIS_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&self) -> BMX_DBG_SEL_R {
        BMX_DBG_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bmx_err_tz(&self) -> BMX_ERR_TZ_R {
        BMX_ERR_TZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bmx_err_dec(&self) -> BMX_ERR_DEC_R {
        BMX_ERR_DEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&self) -> BMX_ERR_ADDR_DIS_R {
        BMX_ERR_ADDR_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&mut self) -> BMX_DBG_SEL_W {
        BMX_DBG_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bmx_err_tz(&mut self) -> BMX_ERR_TZ_W {
        BMX_ERR_TZ_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bmx_err_dec(&mut self) -> BMX_ERR_DEC_W {
        BMX_ERR_DEC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&mut self) -> BMX_ERR_ADDR_DIS_W {
        BMX_ERR_ADDR_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg2](index.html) module"]
pub struct BMX_CFG2_SPEC;
impl crate::RegisterSpec for BMX_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg2::R](R) reader structure"]
impl crate::Readable for BMX_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg2::W](W) writer structure"]
impl crate::Writable for BMX_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets bmx_cfg2 to value 0"]
impl crate::Resettable for BMX_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
