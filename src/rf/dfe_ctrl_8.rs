#[doc = "Register `dfe_ctrl_8` reader"]
pub struct R(crate::R<DFE_CTRL_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_8_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_8` writer"]
pub struct W(crate::W<DFE_CTRL_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_8_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_8_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pm_iqacc_i` reader - "]
pub struct RX_PM_IQACC_I_R(crate::FieldReader<u32, u32>);
impl RX_PM_IQACC_I_R {
    pub(crate) fn new(bits: u32) -> Self {
        RX_PM_IQACC_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_IQACC_I_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_iqacc_i` writer - "]
pub struct RX_PM_IQACC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_IQACC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_i(&self) -> RX_PM_IQACC_I_R {
        RX_PM_IQACC_I_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_i(&mut self) -> RX_PM_IQACC_I_W {
        RX_PM_IQACC_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_8](index.html) module"]
pub struct DFE_CTRL_8_SPEC;
impl crate::RegisterSpec for DFE_CTRL_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_8::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_8::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_8 to value 0"]
impl crate::Resettable for DFE_CTRL_8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
