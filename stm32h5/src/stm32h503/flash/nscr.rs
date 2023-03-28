#[doc = "Register `NSCR` reader"]
pub struct R(crate::R<NSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCR` writer"]
pub struct W(crate::W<NSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCR_SPEC>;
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
impl From<crate::W<NSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `PG` reader - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
pub type PG_R = crate::BitReader<bool>;
#[doc = "Field `PG` writer - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `SER` reader - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
pub type SER_R = crate::BitReader<bool>;
#[doc = "Field `SER` writer - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
pub type SER_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `BER` reader - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
pub type BER_R = crate::BitReader<bool>;
#[doc = "Field `BER` writer - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
pub type BER_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `FW` reader - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
pub type FW_R = crate::BitReader<bool>;
#[doc = "Field `FW` writer - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
pub type FW_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `STRT` reader - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
pub type STRT_R = crate::BitReader<bool>;
#[doc = "Field `STRT` writer - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `SNB` reader - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
pub type SNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNB` writer - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
pub type SNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NSCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MER` reader - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
pub type MER_R = crate::BitReader<bool>;
#[doc = "Field `MER` writer - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
pub type MER_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `EOPIE` reader - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
pub type EOPIE_R = crate::BitReader<bool>;
#[doc = "Field `EOPIE` writer - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `WRPERRIE` reader - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
pub type WRPERRIE_R = crate::BitReader<bool>;
#[doc = "Field `WRPERRIE` writer - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
pub type WRPERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `PGSERRIE` reader - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
pub type PGSERRIE_R = crate::BitReader<bool>;
#[doc = "Field `PGSERRIE` writer - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
pub type PGSERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `STRBERRIE` reader - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
pub type STRBERRIE_R = crate::BitReader<bool>;
#[doc = "Field `STRBERRIE` writer - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
pub type STRBERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `INCERRIE` reader - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
pub type INCERRIE_R = crate::BitReader<bool>;
#[doc = "Field `INCERRIE` writer - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
pub type INCERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
pub type OPTCHANGEERRIE_R = crate::BitReader<bool>;
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
pub type OPTCHANGEERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
#[doc = "Field `BKSEL` reader - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
pub type BKSEL_R = crate::BitReader<bool>;
#[doc = "Field `BKSEL` writer - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
pub type BKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 15 - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
    #[inline(always)]
    pub fn bksel(&self) -> BKSEL_R {
        BKSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 1 - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<1> {
        PG_W::new(self)
    }
    #[doc = "Bit 2 - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<2> {
        SER_W::new(self)
    }
    #[doc = "Bit 3 - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
    #[inline(always)]
    #[must_use]
    pub fn ber(&mut self) -> BER_W<3> {
        BER_W::new(self)
    }
    #[doc = "Bit 4 - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
    #[inline(always)]
    #[must_use]
    pub fn fw(&mut self) -> FW_W<4> {
        FW_W::new(self)
    }
    #[doc = "Bit 5 - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<5> {
        STRT_W::new(self)
    }
    #[doc = "Bits 6:8 - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
    #[inline(always)]
    #[must_use]
    pub fn snb(&mut self) -> SNB_W<6> {
        SNB_W::new(self)
    }
    #[doc = "Bit 15 - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<15> {
        MER_W::new(self)
    }
    #[doc = "Bit 16 - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<16> {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 17 - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<17> {
        WRPERRIE_W::new(self)
    }
    #[doc = "Bit 18 - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<18> {
        PGSERRIE_W::new(self)
    }
    #[doc = "Bit 19 - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn strberrie(&mut self) -> STRBERRIE_W<19> {
        STRBERRIE_W::new(self)
    }
    #[doc = "Bit 20 - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn incerrie(&mut self) -> INCERRIE_W<20> {
        INCERRIE_W::new(self)
    }
    #[doc = "Bit 23 - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<23> {
        OPTCHANGEERRIE_W::new(self)
    }
    #[doc = "Bit 31 - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn bksel(&mut self) -> BKSEL_W<31> {
        BKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Non Secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nscr](index.html) module"]
pub struct NSCR_SPEC;
impl crate::RegisterSpec for NSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nscr::R](R) reader structure"]
impl crate::Readable for NSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nscr::W](W) writer structure"]
impl crate::Writable for NSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSCR to value 0x01"]
impl crate::Resettable for NSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
