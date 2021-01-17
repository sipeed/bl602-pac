#[doc = "Register `GPADC_32M_SRC_CTRL` reader"]
pub struct R(crate::R<GPADC_32M_SRC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_32M_SRC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_32M_SRC_CTRL_SPEC>> for R {
    fn from(reader: crate::R<GPADC_32M_SRC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPADC_32M_SRC_CTRL` writer"]
pub struct W(crate::W<GPADC_32M_SRC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_32M_SRC_CTRL_SPEC>;
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
impl core::convert::From<crate::W<GPADC_32M_SRC_CTRL_SPEC>> for W {
    fn from(writer: crate::W<GPADC_32M_SRC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_32m_div_en` reader - "]
pub struct GPADC_32M_DIV_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_32M_DIV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_32M_DIV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_32M_DIV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_32m_div_en` writer - "]
pub struct GPADC_32M_DIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_32M_DIV_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `gpadc_32m_clk_sel` reader - "]
pub struct GPADC_32M_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl GPADC_32M_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_32M_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_32M_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_32m_clk_sel` writer - "]
pub struct GPADC_32M_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_32M_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `gpadc_32m_clk_div` reader - "]
pub struct GPADC_32M_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl GPADC_32M_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_32M_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_32M_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_32m_clk_div` writer - "]
pub struct GPADC_32M_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_32M_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&self) -> GPADC_32M_DIV_EN_R {
        GPADC_32M_DIV_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&self) -> GPADC_32M_CLK_SEL_R {
        GPADC_32M_CLK_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&self) -> GPADC_32M_CLK_DIV_R {
        GPADC_32M_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&mut self) -> GPADC_32M_DIV_EN_W {
        GPADC_32M_DIV_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&mut self) -> GPADC_32M_CLK_SEL_W {
        GPADC_32M_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&mut self) -> GPADC_32M_CLK_DIV_W {
        GPADC_32M_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration for GPADC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_32m_src_ctrl](index.html) module"]
pub struct GPADC_32M_SRC_CTRL_SPEC;
impl crate::RegisterSpec for GPADC_32M_SRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_32m_src_ctrl::R](R) reader structure"]
impl crate::Readable for GPADC_32M_SRC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_32m_src_ctrl::W](W) writer structure"]
impl crate::Writable for GPADC_32M_SRC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPADC_32M_SRC_CTRL to value 0"]
impl crate::Resettable for GPADC_32M_SRC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
