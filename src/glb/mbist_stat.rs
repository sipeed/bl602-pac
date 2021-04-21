#[doc = "Register `MBIST_STAT` reader"]
pub struct R(crate::R<MBIST_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MBIST_STAT_SPEC>> for R {
    fn from(reader: crate::R<MBIST_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `wifi_mbist_fail` reader - "]
pub struct WIFI_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl WIFI_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ocram_mbist_fail` reader - "]
pub struct OCRAM_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl OCRAM_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCRAM_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCRAM_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tag_mbist_fail` reader - "]
pub struct TAG_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl TAG_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsram_mbist_fail` reader - "]
pub struct HSRAM_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl HSRAM_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSRAM_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAM_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irom_mbist_fail` reader - "]
pub struct IROM_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl IROM_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IROM_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IROM_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifi_mbist_done` reader - "]
pub struct WIFI_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl WIFI_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ocram_mbist_done` reader - "]
pub struct OCRAM_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl OCRAM_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCRAM_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCRAM_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tag_mbist_done` reader - "]
pub struct TAG_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl TAG_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsram_mbist_done` reader - "]
pub struct HSRAM_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl HSRAM_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSRAM_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAM_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irom_mbist_done` reader - "]
pub struct IROM_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl IROM_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IROM_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IROM_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&self) -> WIFI_MBIST_FAIL_R {
        WIFI_MBIST_FAIL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OCRAM_MBIST_FAIL_R {
        OCRAM_MBIST_FAIL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TAG_MBIST_FAIL_R {
        TAG_MBIST_FAIL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&self) -> HSRAM_MBIST_FAIL_R {
        HSRAM_MBIST_FAIL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IROM_MBIST_FAIL_R {
        IROM_MBIST_FAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&self) -> WIFI_MBIST_DONE_R {
        WIFI_MBIST_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OCRAM_MBIST_DONE_R {
        OCRAM_MBIST_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TAG_MBIST_DONE_R {
        TAG_MBIST_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&self) -> HSRAM_MBIST_DONE_R {
        HSRAM_MBIST_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IROM_MBIST_DONE_R {
        IROM_MBIST_DONE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "MBIST_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](index.html) module"]
pub struct MBIST_STAT_SPEC;
impl crate::RegisterSpec for MBIST_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_stat::R](R) reader structure"]
impl crate::Readable for MBIST_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MBIST_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
