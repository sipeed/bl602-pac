#[doc = "Register `adda_reg_ctrl_hw` reader"]
pub struct R(crate::R<ADDA_REG_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDA_REG_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDA_REG_CTRL_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDA_REG_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adda_reg_ctrl_hw` writer"]
pub struct W(crate::W<ADDA_REG_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDA_REG_CTRL_HW_SPEC>;
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
impl From<crate::W<ADDA_REG_CTRL_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDA_REG_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adda_ldo_dvdd_sel_tx` reader - "]
pub struct ADDA_LDO_DVDD_SEL_TX_R(crate::FieldReader<u8, u8>);
impl ADDA_LDO_DVDD_SEL_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDA_LDO_DVDD_SEL_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDA_LDO_DVDD_SEL_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adda_ldo_dvdd_sel_tx` writer - "]
pub struct ADDA_LDO_DVDD_SEL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `adda_ldo_dvdd_sel_rx` reader - "]
pub struct ADDA_LDO_DVDD_SEL_RX_R(crate::FieldReader<u8, u8>);
impl ADDA_LDO_DVDD_SEL_RX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDA_LDO_DVDD_SEL_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDA_LDO_DVDD_SEL_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adda_ldo_dvdd_sel_rx` writer - "]
pub struct ADDA_LDO_DVDD_SEL_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_tx(&self) -> ADDA_LDO_DVDD_SEL_TX_R {
        ADDA_LDO_DVDD_SEL_TX_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_rx(&self) -> ADDA_LDO_DVDD_SEL_RX_R {
        ADDA_LDO_DVDD_SEL_RX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_tx(&mut self) -> ADDA_LDO_DVDD_SEL_TX_W {
        ADDA_LDO_DVDD_SEL_TX_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_rx(&mut self) -> ADDA_LDO_DVDD_SEL_RX_W {
        ADDA_LDO_DVDD_SEL_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adda_reg_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda_reg_ctrl_hw](index.html) module"]
pub struct ADDA_REG_CTRL_HW_SPEC;
impl crate::RegisterSpec for ADDA_REG_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adda_reg_ctrl_hw::R](R) reader structure"]
impl crate::Readable for ADDA_REG_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adda_reg_ctrl_hw::W](W) writer structure"]
impl crate::Writable for ADDA_REG_CTRL_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adda_reg_ctrl_hw to value 0"]
impl crate::Resettable for ADDA_REG_CTRL_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
