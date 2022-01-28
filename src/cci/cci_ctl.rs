#[doc = "Register `cci_ctl` reader"]
pub struct R(crate::R<CCI_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCI_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCI_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ahb_state` reader - "]
pub struct AHB_STATE_R(crate::FieldReader<u8, u8>);
impl AHB_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHB_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_read_flag` reader - "]
pub struct CCI_READ_FLAG_R(crate::FieldReader<bool, bool>);
impl CCI_READ_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_READ_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_READ_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_write_flag` reader - "]
pub struct CCI_WRITE_FLAG_R(crate::FieldReader<bool, bool>);
impl CCI_WRITE_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_WRITE_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_WRITE_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AHB_STATE_R {
        AHB_STATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CCI_READ_FLAG_R {
        CCI_READ_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CCI_WRITE_FLAG_R {
        CCI_WRITE_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "cci_ctl.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_ctl](index.html) module"]
pub struct CCI_CTL_SPEC;
impl crate::RegisterSpec for CCI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_ctl::R](R) reader structure"]
impl crate::Readable for CCI_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cci_ctl to value 0"]
impl crate::Resettable for CCI_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
