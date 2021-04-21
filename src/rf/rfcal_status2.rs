#[doc = "Register `rfcal_status2` reader"]
pub struct R(crate::R<RFCAL_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RFCAL_STATUS2_SPEC>> for R {
    fn from(reader: crate::R<RFCAL_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_status2` writer"]
pub struct W(crate::W<RFCAL_STATUS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_STATUS2_SPEC>;
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
impl core::convert::From<crate::W<RFCAL_STATUS2_SPEC>> for W {
    fn from(writer: crate::W<RFCAL_STATUS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dl_rfcal_table_status` reader - "]
pub struct DL_RFCAL_TABLE_STATUS_R(crate::FieldReader<u8, u8>);
impl DL_RFCAL_TABLE_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DL_RFCAL_TABLE_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL_RFCAL_TABLE_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dl_rfcal_table_status` writer - "]
pub struct DL_RFCAL_TABLE_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&self) -> DL_RFCAL_TABLE_STATUS_R {
        DL_RFCAL_TABLE_STATUS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&mut self) -> DL_RFCAL_TABLE_STATUS_W {
        DL_RFCAL_TABLE_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfcal_status2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_status2](index.html) module"]
pub struct RFCAL_STATUS2_SPEC;
impl crate::RegisterSpec for RFCAL_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_status2::R](R) reader structure"]
impl crate::Readable for RFCAL_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_status2::W](W) writer structure"]
impl crate::Writable for RFCAL_STATUS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfcal_status2 to value 0"]
impl crate::Resettable for RFCAL_STATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
