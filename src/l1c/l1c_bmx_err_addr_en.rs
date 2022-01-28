#[doc = "Register `l1c_bmx_err_addr_en` reader"]
pub struct R(crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1C_BMX_ERR_ADDR_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l1c_bmx_err_addr_en` writer"]
pub struct W(crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>;
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
impl From<crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1C_BMX_ERR_ADDR_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l1c_hsel_option` reader - "]
pub struct L1C_HSEL_OPTION_R(crate::FieldReader<u8, u8>);
impl L1C_HSEL_OPTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        L1C_HSEL_OPTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_HSEL_OPTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_hsel_option` writer - "]
pub struct L1C_HSEL_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_HSEL_OPTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `l1c_bmx_err_tz` reader - "]
pub struct L1C_BMX_ERR_TZ_R(crate::FieldReader<bool, bool>);
impl L1C_BMX_ERR_TZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_BMX_ERR_TZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_ERR_TZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_err_dec` reader - "]
pub struct L1C_BMX_ERR_DEC_R(crate::FieldReader<bool, bool>);
impl L1C_BMX_ERR_DEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_BMX_ERR_DEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_ERR_DEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_err_addr_dis` reader - "]
pub struct L1C_BMX_ERR_ADDR_DIS_R(crate::FieldReader<bool, bool>);
impl L1C_BMX_ERR_ADDR_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_BMX_ERR_ADDR_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_ERR_ADDR_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_err_addr_dis` writer - "]
pub struct L1C_BMX_ERR_ADDR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_ADDR_DIS_W<'a> {
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
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&self) -> L1C_HSEL_OPTION_R {
        L1C_HSEL_OPTION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn l1c_bmx_err_tz(&self) -> L1C_BMX_ERR_TZ_R {
        L1C_BMX_ERR_TZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn l1c_bmx_err_dec(&self) -> L1C_BMX_ERR_DEC_R {
        L1C_BMX_ERR_DEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&self) -> L1C_BMX_ERR_ADDR_DIS_R {
        L1C_BMX_ERR_ADDR_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&mut self) -> L1C_HSEL_OPTION_W {
        L1C_HSEL_OPTION_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&mut self) -> L1C_BMX_ERR_ADDR_DIS_W {
        L1C_BMX_ERR_ADDR_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "l1c_bmx_err_addr_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_bmx_err_addr_en](index.html) module"]
pub struct L1C_BMX_ERR_ADDR_EN_SPEC;
impl crate::RegisterSpec for L1C_BMX_ERR_ADDR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_bmx_err_addr_en::R](R) reader structure"]
impl crate::Readable for L1C_BMX_ERR_ADDR_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1c_bmx_err_addr_en::W](W) writer structure"]
impl crate::Writable for L1C_BMX_ERR_ADDR_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets l1c_bmx_err_addr_en to value 0"]
impl crate::Resettable for L1C_BMX_ERR_ADDR_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
