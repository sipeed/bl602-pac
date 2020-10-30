#[doc = "Register `lo_sdm_ctrl_hw2` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LO_SDM_CTRL_HW2_SPEC>> for R {
    fn from(reader: crate::R<LO_SDM_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw2` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW2_SPEC>;
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
impl core::convert::From<crate::W<LO_SDM_CTRL_HW2_SPEC>> for W {
    fn from(writer: crate::W<LO_SDM_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2432` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2432_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2432_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2432_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2432_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2432` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2432_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2432_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2430` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2430_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2430_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2430_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2430_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2430` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2430_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2430_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2428` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2428_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2428_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2428_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2428_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2428` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2428_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2428_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2426` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2426_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2426_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2426_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2426_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2426` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2426_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2426_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2424` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2424_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2424_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2424_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2424_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2424` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2424_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2424_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2422` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2422_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2422_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2422_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2422_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2422` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2422_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2422_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2420` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2420_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2420_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2420_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2420_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2420` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2420_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2420_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2418` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2418_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2418_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2418_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2418_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2418` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2418_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2418_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2416` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2416_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2416_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2416_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2416_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2416` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2416_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2416_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2414` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2414_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2414_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2414_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2414_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2414` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2414_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2414_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2412` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2412_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2412_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2412_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2412_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2412` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2412_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2412_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2410` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2410_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2410_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2410_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2410_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2410` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2410_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2410_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2408` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2408_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2408_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2408_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2408_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2408` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2408_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2408_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2406` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2406_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2406_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2406_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2406_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2406` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2406_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2406_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2404` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2404_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2404_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2404_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2404_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2404` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2404_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2404_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2402` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2402_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2402_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2402_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2402_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2402` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2402_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2402_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2432(&self) -> LO_SDM_DITHER_SEL_BLE_2432_R {
        LO_SDM_DITHER_SEL_BLE_2432_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2430(&self) -> LO_SDM_DITHER_SEL_BLE_2430_R {
        LO_SDM_DITHER_SEL_BLE_2430_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2428(&self) -> LO_SDM_DITHER_SEL_BLE_2428_R {
        LO_SDM_DITHER_SEL_BLE_2428_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2426(&self) -> LO_SDM_DITHER_SEL_BLE_2426_R {
        LO_SDM_DITHER_SEL_BLE_2426_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2424(&self) -> LO_SDM_DITHER_SEL_BLE_2424_R {
        LO_SDM_DITHER_SEL_BLE_2424_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2422(&self) -> LO_SDM_DITHER_SEL_BLE_2422_R {
        LO_SDM_DITHER_SEL_BLE_2422_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2420(&self) -> LO_SDM_DITHER_SEL_BLE_2420_R {
        LO_SDM_DITHER_SEL_BLE_2420_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2418(&self) -> LO_SDM_DITHER_SEL_BLE_2418_R {
        LO_SDM_DITHER_SEL_BLE_2418_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2416(&self) -> LO_SDM_DITHER_SEL_BLE_2416_R {
        LO_SDM_DITHER_SEL_BLE_2416_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2414(&self) -> LO_SDM_DITHER_SEL_BLE_2414_R {
        LO_SDM_DITHER_SEL_BLE_2414_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2412(&self) -> LO_SDM_DITHER_SEL_BLE_2412_R {
        LO_SDM_DITHER_SEL_BLE_2412_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2410(&self) -> LO_SDM_DITHER_SEL_BLE_2410_R {
        LO_SDM_DITHER_SEL_BLE_2410_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2408(&self) -> LO_SDM_DITHER_SEL_BLE_2408_R {
        LO_SDM_DITHER_SEL_BLE_2408_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2406(&self) -> LO_SDM_DITHER_SEL_BLE_2406_R {
        LO_SDM_DITHER_SEL_BLE_2406_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2404(&self) -> LO_SDM_DITHER_SEL_BLE_2404_R {
        LO_SDM_DITHER_SEL_BLE_2404_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2402(&self) -> LO_SDM_DITHER_SEL_BLE_2402_R {
        LO_SDM_DITHER_SEL_BLE_2402_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2432(&mut self) -> LO_SDM_DITHER_SEL_BLE_2432_W {
        LO_SDM_DITHER_SEL_BLE_2432_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2430(&mut self) -> LO_SDM_DITHER_SEL_BLE_2430_W {
        LO_SDM_DITHER_SEL_BLE_2430_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2428(&mut self) -> LO_SDM_DITHER_SEL_BLE_2428_W {
        LO_SDM_DITHER_SEL_BLE_2428_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2426(&mut self) -> LO_SDM_DITHER_SEL_BLE_2426_W {
        LO_SDM_DITHER_SEL_BLE_2426_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2424(&mut self) -> LO_SDM_DITHER_SEL_BLE_2424_W {
        LO_SDM_DITHER_SEL_BLE_2424_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2422(&mut self) -> LO_SDM_DITHER_SEL_BLE_2422_W {
        LO_SDM_DITHER_SEL_BLE_2422_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2420(&mut self) -> LO_SDM_DITHER_SEL_BLE_2420_W {
        LO_SDM_DITHER_SEL_BLE_2420_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2418(&mut self) -> LO_SDM_DITHER_SEL_BLE_2418_W {
        LO_SDM_DITHER_SEL_BLE_2418_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2416(&mut self) -> LO_SDM_DITHER_SEL_BLE_2416_W {
        LO_SDM_DITHER_SEL_BLE_2416_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2414(&mut self) -> LO_SDM_DITHER_SEL_BLE_2414_W {
        LO_SDM_DITHER_SEL_BLE_2414_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2412(&mut self) -> LO_SDM_DITHER_SEL_BLE_2412_W {
        LO_SDM_DITHER_SEL_BLE_2412_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2410(&mut self) -> LO_SDM_DITHER_SEL_BLE_2410_W {
        LO_SDM_DITHER_SEL_BLE_2410_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2408(&mut self) -> LO_SDM_DITHER_SEL_BLE_2408_W {
        LO_SDM_DITHER_SEL_BLE_2408_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2406(&mut self) -> LO_SDM_DITHER_SEL_BLE_2406_W {
        LO_SDM_DITHER_SEL_BLE_2406_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2404(&mut self) -> LO_SDM_DITHER_SEL_BLE_2404_W {
        LO_SDM_DITHER_SEL_BLE_2404_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2402(&mut self) -> LO_SDM_DITHER_SEL_BLE_2402_W {
        LO_SDM_DITHER_SEL_BLE_2402_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw2](index.html) module"]
pub struct LO_SDM_CTRL_HW2_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw2 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
