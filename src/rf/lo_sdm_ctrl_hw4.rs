#[doc = "Register `lo_sdm_ctrl_hw4` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SDM_CTRL_HW4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SDM_CTRL_HW4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw4` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW4_SPEC>;
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
impl From<crate::W<LO_SDM_CTRL_HW4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SDM_CTRL_HW4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_tx` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_TX_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_tx` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2480` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2480_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2480_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2480_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2480_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2480` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2480_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2480_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2478` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2478_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2478_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2478_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2478_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2478` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2478_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2478_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2476` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2476_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2476_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2476_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2476_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2476` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2476_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2476_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2474` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2474_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2474_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2474_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2474_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2474` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2474_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2474_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2472` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2472_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2472_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2472_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2472_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2472` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2472_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2472_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2470` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2470_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2470_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2470_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2470_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2470` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2470_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2470_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2468` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2468_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2468_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2468_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2468_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2468` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2468_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2468_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2466` reader - "]
pub struct LO_SDM_DITHER_SEL_BLE_2466_R(crate::FieldReader<u8, u8>);
impl LO_SDM_DITHER_SEL_BLE_2466_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_DITHER_SEL_BLE_2466_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_DITHER_SEL_BLE_2466_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_dither_sel_ble_2466` writer - "]
pub struct LO_SDM_DITHER_SEL_BLE_2466_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2466_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&self) -> LO_SDM_DITHER_SEL_BLE_TX_R {
        LO_SDM_DITHER_SEL_BLE_TX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&self) -> LO_SDM_DITHER_SEL_BLE_2480_R {
        LO_SDM_DITHER_SEL_BLE_2480_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&self) -> LO_SDM_DITHER_SEL_BLE_2478_R {
        LO_SDM_DITHER_SEL_BLE_2478_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&self) -> LO_SDM_DITHER_SEL_BLE_2476_R {
        LO_SDM_DITHER_SEL_BLE_2476_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&self) -> LO_SDM_DITHER_SEL_BLE_2474_R {
        LO_SDM_DITHER_SEL_BLE_2474_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&self) -> LO_SDM_DITHER_SEL_BLE_2472_R {
        LO_SDM_DITHER_SEL_BLE_2472_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&self) -> LO_SDM_DITHER_SEL_BLE_2470_R {
        LO_SDM_DITHER_SEL_BLE_2470_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&self) -> LO_SDM_DITHER_SEL_BLE_2468_R {
        LO_SDM_DITHER_SEL_BLE_2468_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&self) -> LO_SDM_DITHER_SEL_BLE_2466_R {
        LO_SDM_DITHER_SEL_BLE_2466_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&mut self) -> LO_SDM_DITHER_SEL_BLE_TX_W {
        LO_SDM_DITHER_SEL_BLE_TX_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&mut self) -> LO_SDM_DITHER_SEL_BLE_2480_W {
        LO_SDM_DITHER_SEL_BLE_2480_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&mut self) -> LO_SDM_DITHER_SEL_BLE_2478_W {
        LO_SDM_DITHER_SEL_BLE_2478_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&mut self) -> LO_SDM_DITHER_SEL_BLE_2476_W {
        LO_SDM_DITHER_SEL_BLE_2476_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&mut self) -> LO_SDM_DITHER_SEL_BLE_2474_W {
        LO_SDM_DITHER_SEL_BLE_2474_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&mut self) -> LO_SDM_DITHER_SEL_BLE_2472_W {
        LO_SDM_DITHER_SEL_BLE_2472_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&mut self) -> LO_SDM_DITHER_SEL_BLE_2470_W {
        LO_SDM_DITHER_SEL_BLE_2470_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&mut self) -> LO_SDM_DITHER_SEL_BLE_2468_W {
        LO_SDM_DITHER_SEL_BLE_2468_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&mut self) -> LO_SDM_DITHER_SEL_BLE_2466_W {
        LO_SDM_DITHER_SEL_BLE_2466_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw4](index.html) module"]
pub struct LO_SDM_CTRL_HW4_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw4::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw4::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw4 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
