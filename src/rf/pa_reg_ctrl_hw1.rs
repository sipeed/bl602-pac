#[doc = "Register `pa_reg_ctrl_hw1` reader"]
pub struct R(crate::R<PA_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_REG_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PA_REG_CTRL_HW1_SPEC>> for R {
    fn from(reader: crate::R<PA_REG_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa_reg_ctrl_hw1` writer"]
pub struct W(crate::W<PA_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_REG_CTRL_HW1_SPEC>;
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
impl core::convert::From<crate::W<PA_REG_CTRL_HW1_SPEC>> for W {
    fn from(writer: crate::W<PA_REG_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_vbcas_11n` reader - "]
pub struct PA_VBCAS_11N_R(crate::FieldReader<u8, u8>);
impl PA_VBCAS_11N_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCAS_11N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCAS_11N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcas_11n` writer - "]
pub struct PA_VBCAS_11N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_11N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `pa_vbcore_11n` reader - "]
pub struct PA_VBCORE_11N_R(crate::FieldReader<u8, u8>);
impl PA_VBCORE_11N_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCORE_11N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCORE_11N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcore_11n` writer - "]
pub struct PA_VBCORE_11N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_11N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `pa_iet_11n` reader - "]
pub struct PA_IET_11N_R(crate::FieldReader<u8, u8>);
impl PA_IET_11N_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_IET_11N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IET_11N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_iet_11n` writer - "]
pub struct PA_IET_11N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_11N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&self) -> PA_VBCAS_11N_R {
        PA_VBCAS_11N_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&self) -> PA_VBCORE_11N_R {
        PA_VBCORE_11N_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&self) -> PA_IET_11N_R {
        PA_IET_11N_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&mut self) -> PA_VBCAS_11N_W {
        PA_VBCAS_11N_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&mut self) -> PA_VBCORE_11N_W {
        PA_VBCORE_11N_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&mut self) -> PA_IET_11N_W {
        PA_IET_11N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa_reg_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_ctrl_hw1](index.html) module"]
pub struct PA_REG_CTRL_HW1_SPEC;
impl crate::RegisterSpec for PA_REG_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_reg_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for PA_REG_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_reg_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for PA_REG_CTRL_HW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pa_reg_ctrl_hw1 to value 0"]
impl crate::Resettable for PA_REG_CTRL_HW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
