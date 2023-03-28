#[doc = "Register `OTFDEC_R4CFGR` reader"]
pub struct R(crate::R<OTFDEC_R4CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFDEC_R4CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFDEC_R4CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFDEC_R4CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTFDEC_R4CFGR` writer"]
pub struct W(crate::W<OTFDEC_R4CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_R4CFGR_SPEC>;
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
impl From<crate::W<OTFDEC_R4CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_R4CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_EN` reader - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
pub type REG_EN_R = crate::BitReader<bool>;
#[doc = "Field `REG_EN` writer - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
pub type REG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_R4CFGR_SPEC, bool, O>;
#[doc = "Field `CONFIGLOCK` reader - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
pub type CONFIGLOCK_R = crate::BitReader<bool>;
#[doc = "Field `CONFIGLOCK` writer - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
pub type CONFIGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_R4CFGR_SPEC, bool, O>;
#[doc = "Field `KEYLOCK` reader - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
pub type KEYLOCK_R = crate::BitReader<bool>;
#[doc = "Field `KEYLOCK` writer - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
pub type KEYLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_R4CFGR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTFDEC_R4CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEYCRC` reader - region key 8-bit CRC When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new computation starts as soon as a new valid sequence is initiated, and KEYCRC is read as zero until a valid sequence is completed. When KEYLOCK = 1, KEYCRC remains unchanged until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Source code is available in . This field is read only. Note: CRC information is updated only after the last bit of the key has been written."]
pub type KEYCRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGx_VERSION` reader - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
pub type REGX_VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REGx_VERSION` writer - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
pub type REGX_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTFDEC_R4CFGR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - region key 8-bit CRC When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new computation starts as soon as a new valid sequence is initiated, and KEYCRC is read as zero until a valid sequence is completed. When KEYLOCK = 1, KEYCRC remains unchanged until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Source code is available in . This field is read only. Note: CRC information is updated only after the last bit of the key has been written."]
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
    #[inline(always)]
    pub fn regx_version(&self) -> REGX_VERSION_R {
        REGX_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn reg_en(&mut self) -> REG_EN_W<0> {
        REG_EN_W::new(self)
    }
    #[doc = "Bit 1 - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<1> {
        CONFIGLOCK_W::new(self)
    }
    #[doc = "Bit 2 - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
    #[inline(always)]
    #[must_use]
    pub fn keylock(&mut self) -> KEYLOCK_W<2> {
        KEYLOCK_W::new(self)
    }
    #[doc = "Bits 4:5 - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:31 - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
    #[inline(always)]
    #[must_use]
    pub fn regx_version(&mut self) -> REGX_VERSION_W<16> {
        REGX_VERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_r4cfgr](index.html) module"]
pub struct OTFDEC_R4CFGR_SPEC;
impl crate::RegisterSpec for OTFDEC_R4CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfdec_r4cfgr::R](R) reader structure"]
impl crate::Readable for OTFDEC_R4CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otfdec_r4cfgr::W](W) writer structure"]
impl crate::Writable for OTFDEC_R4CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_R4CFGR to value 0"]
impl crate::Resettable for OTFDEC_R4CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
