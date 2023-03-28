#[doc = "Register `SECOBKCFGR` reader"]
pub struct R(crate::R<SECOBKCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECOBKCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECOBKCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECOBKCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECOBKCFGR` writer"]
pub struct W(crate::W<SECOBKCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECOBKCFGR_SPEC>;
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
impl From<crate::W<SECOBKCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECOBKCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECOBKCFGR_SPEC, bool, O>;
#[doc = "Field `SWAP_SECT_REQ` reader - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
pub type SWAP_SECT_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SWAP_SECT_REQ` writer - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
pub type SWAP_SECT_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECOBKCFGR_SPEC, bool, O>;
#[doc = "Field `ALT_SECT` reader - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated"]
pub type ALT_SECT_R = crate::BitReader<bool>;
#[doc = "Field `ALT_SECT` writer - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated"]
pub type ALT_SECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECOBKCFGR_SPEC, bool, O>;
#[doc = "Field `ALT_SECT_ERASE` reader - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit."]
pub type ALT_SECT_ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ALT_SECT_ERASE` writer - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit."]
pub type ALT_SECT_ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECOBKCFGR_SPEC, bool, O>;
#[doc = "Field `SWAP_OFFSET` reader - key index (offset /16 bits) pointing for next swap. …"]
pub type SWAP_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SWAP_OFFSET` writer - key index (offset /16 bits) pointing for next swap. …"]
pub type SWAP_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECOBKCFGR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 0 - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
    #[inline(always)]
    pub fn swap_sect_req(&self) -> SWAP_SECT_REQ_R {
        SWAP_SECT_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated"]
    #[inline(always)]
    pub fn alt_sect(&self) -> ALT_SECT_R {
        ALT_SECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit."]
    #[inline(always)]
    pub fn alt_sect_erase(&self) -> ALT_SECT_ERASE_R {
        ALT_SECT_ERASE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:24 - key index (offset /16 bits) pointing for next swap. …"]
    #[inline(always)]
    pub fn swap_offset(&self) -> SWAP_OFFSET_R {
        SWAP_OFFSET_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - OBKCFGR lock option configuration bit This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 1 - OBK swap sector request bit When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one. The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL ≠ 0. Minimum value is 16 for OBK-HDPL = 1, 144 for OBK-HDPL = 2 and 192 for OBK-HDPL = 3."]
    #[inline(always)]
    #[must_use]
    pub fn swap_sect_req(&mut self) -> SWAP_SECT_REQ_W<1> {
        SWAP_SECT_REQ_W::new(self)
    }
    #[doc = "Bit 2 - alternate sector bit This bit must not change while filling the write buffer, otherwise an error is generated"]
    #[inline(always)]
    #[must_use]
    pub fn alt_sect(&mut self) -> ALT_SECT_W<2> {
        ALT_SECT_W::new(self)
    }
    #[doc = "Bit 3 - alternate sector erase bit When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit."]
    #[inline(always)]
    #[must_use]
    pub fn alt_sect_erase(&mut self) -> ALT_SECT_ERASE_W<3> {
        ALT_SECT_ERASE_W::new(self)
    }
    #[doc = "Bits 16:24 - key index (offset /16 bits) pointing for next swap. …"]
    #[inline(always)]
    #[must_use]
    pub fn swap_offset(&mut self) -> SWAP_OFFSET_W<16> {
        SWAP_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure OBK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secobkcfgr](index.html) module"]
pub struct SECOBKCFGR_SPEC;
impl crate::RegisterSpec for SECOBKCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secobkcfgr::R](R) reader structure"]
impl crate::Readable for SECOBKCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secobkcfgr::W](W) writer structure"]
impl crate::Writable for SECOBKCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECOBKCFGR to value 0x01ff_0000"]
impl crate::Resettable for SECOBKCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_0000;
}
