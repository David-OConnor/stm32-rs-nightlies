#[doc = "Register `HASH_CR` reader"]
pub struct R(crate::R<HASH_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CR` writer"]
pub struct W(crate::W<HASH_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CR_SPEC>;
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
impl From<crate::W<HASH_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `DMAE` reader - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
pub type DMAE_R = crate::BitReader<bool>;
#[doc = "Field `DMAE` writer - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `DATATYPE` reader - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATYPE` writer - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `NBW` reader - Number of words already pushed Refer to NBWP\\[3:0\\]
bitfield of HASH_SR for a description of NBW\\[3:0\\]
bitfield. This bit is read-only."]
pub type NBW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINNE` reader - DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only."]
pub type DINNE_R = crate::BitReader<bool>;
#[doc = "Field `MDMAT` reader - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
pub type MDMAT_R = crate::BitReader<bool>;
#[doc = "Field `MDMAT` writer - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
pub type MDMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `LKEY` reader - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
pub type LKEY_R = crate::BitReader<bool>;
#[doc = "Field `LKEY` writer - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
pub type LKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `ALGO` reader - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
pub type ALGO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALGO` writer - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
pub type ALGO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 2 - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words already pushed Refer to NBWP\\[3:0\\]
bitfield of HASH_SR for a description of NBW\\[3:0\\]
bitfield. This bit is read-only."]
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only."]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
    #[inline(always)]
    pub fn algo(&self) -> ALGO_R {
        ALGO_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<2> {
        INIT_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<3> {
        DMAE_W::new(self)
    }
    #[doc = "Bits 4:5 - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<4> {
        DATATYPE_W::new(self)
    }
    #[doc = "Bit 6 - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<6> {
        MODE_W::new(self)
    }
    #[doc = "Bit 13 - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
    #[inline(always)]
    #[must_use]
    pub fn mdmat(&mut self) -> MDMAT_W<13> {
        MDMAT_W::new(self)
    }
    #[doc = "Bit 16 - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LKEY_W<16> {
        LKEY_W::new(self)
    }
    #[doc = "Bits 17:20 - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
    #[inline(always)]
    #[must_use]
    pub fn algo(&mut self) -> ALGO_W<17> {
        ALGO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_cr](index.html) module"]
pub struct HASH_CR_SPEC;
impl crate::RegisterSpec for HASH_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_cr::R](R) reader structure"]
impl crate::Readable for HASH_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_cr::W](W) writer structure"]
impl crate::Writable for HASH_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_CR to value 0"]
impl crate::Resettable for HASH_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
