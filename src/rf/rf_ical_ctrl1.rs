#[doc = "Register `rf_ical_ctrl1` reader"]
pub struct R(crate::R<RF_ICAL_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ICAL_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ICAL_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ICAL_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ical_ctrl1` writer"]
pub struct W(crate::W<RF_ICAL_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ICAL_CTRL1_SPEC>;
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
impl From<crate::W<RF_ICAL_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ICAL_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ical_r_os_i` reader - "]
pub struct RF_ICAL_R_OS_I_R(crate::FieldReader<u16, u16>);
impl RF_ICAL_R_OS_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_ICAL_R_OS_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_R_OS_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_r_os_i` writer - "]
pub struct RF_ICAL_R_OS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_OS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `rf_ical_r_os_q` reader - "]
pub struct RF_ICAL_R_OS_Q_R(crate::FieldReader<u16, u16>);
impl RF_ICAL_R_OS_Q_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_ICAL_R_OS_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_R_OS_Q_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_r_os_q` writer - "]
pub struct RF_ICAL_R_OS_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_OS_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `rf_ical_r_avg_n` reader - "]
pub struct RF_ICAL_R_AVG_N_R(crate::FieldReader<u8, u8>);
impl RF_ICAL_R_AVG_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_ICAL_R_AVG_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_R_AVG_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_r_avg_n` writer - "]
pub struct RF_ICAL_R_AVG_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_AVG_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&self) -> RF_ICAL_R_OS_I_R {
        RF_ICAL_R_OS_I_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&self) -> RF_ICAL_R_OS_Q_R {
        RF_ICAL_R_OS_Q_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&self) -> RF_ICAL_R_AVG_N_R {
        RF_ICAL_R_AVG_N_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&mut self) -> RF_ICAL_R_OS_I_W {
        RF_ICAL_R_OS_I_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&mut self) -> RF_ICAL_R_OS_Q_W {
        RF_ICAL_R_OS_Q_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&mut self) -> RF_ICAL_R_AVG_N_W {
        RF_ICAL_R_AVG_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ical_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl1](index.html) module"]
pub struct RF_ICAL_CTRL1_SPEC;
impl crate::RegisterSpec for RF_ICAL_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ical_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_ical_ctrl1 to value 0"]
impl crate::Resettable for RF_ICAL_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
