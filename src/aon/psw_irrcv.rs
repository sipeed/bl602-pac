#[doc = "Register `psw_irrcv` reader"]
pub struct R(crate::R<PSW_IRRCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSW_IRRCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSW_IRRCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSW_IRRCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psw_irrcv` writer"]
pub struct W(crate::W<PSW_IRRCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSW_IRRCV_SPEC>;
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
impl From<crate::W<PSW_IRRCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSW_IRRCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ir_psw_aon` reader - "]
pub struct PU_IR_PSW_AON_R(crate::FieldReader<bool, bool>);
impl PU_IR_PSW_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_IR_PSW_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_IR_PSW_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_ir_psw_aon` writer - "]
pub struct PU_IR_PSW_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_IR_PSW_AON_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&self) -> PU_IR_PSW_AON_R {
        PU_IR_PSW_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&mut self) -> PU_IR_PSW_AON_W {
        PU_IR_PSW_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psw_irrcv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psw_irrcv](index.html) module"]
pub struct PSW_IRRCV_SPEC;
impl crate::RegisterSpec for PSW_IRRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psw_irrcv::R](R) reader structure"]
impl crate::Readable for PSW_IRRCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psw_irrcv::W](W) writer structure"]
impl crate::Writable for PSW_IRRCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets psw_irrcv to value 0"]
impl crate::Resettable for PSW_IRRCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
