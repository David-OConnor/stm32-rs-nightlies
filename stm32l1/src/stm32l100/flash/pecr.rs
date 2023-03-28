#[doc = "Register `PECR` reader"]
pub struct R(crate::R<PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PECR` writer"]
pub struct W(crate::W<PECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PECR_SPEC>;
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
impl From<crate::W<PECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PELOCK` reader - FLASH_PECR and data EEPROM lock"]
pub type PELOCK_R = crate::BitReader<bool>;
#[doc = "Field `PELOCK` writer - FLASH_PECR and data EEPROM lock"]
pub type PELOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `PRGLOCK` reader - Program memory lock"]
pub type PRGLOCK_R = crate::BitReader<bool>;
#[doc = "Field `PRGLOCK` writer - Program memory lock"]
pub type PRGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `OPTLOCK` reader - Option bytes block lock"]
pub type OPTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OPTLOCK` writer - Option bytes block lock"]
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `PROG` reader - Program memory selection"]
pub type PROG_R = crate::BitReader<bool>;
#[doc = "Field `PROG` writer - Program memory selection"]
pub type PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `DATA` reader - Data EEPROM selection"]
pub type DATA_R = crate::BitReader<bool>;
#[doc = "Field `DATA` writer - Data EEPROM selection"]
pub type DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `FTDW` reader - Fixed time data write for Byte, Half Word and Word programming"]
pub type FTDW_R = crate::BitReader<bool>;
#[doc = "Field `FTDW` writer - Fixed time data write for Byte, Half Word and Word programming"]
pub type FTDW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `ERASE` reader - Page or Double Word erase mode"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - Page or Double Word erase mode"]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `FPRG` reader - Half Page/Double Word programming mode"]
pub type FPRG_R = crate::BitReader<bool>;
#[doc = "Field `FPRG` writer - Half Page/Double Word programming mode"]
pub type FPRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `PARALLELBANK` reader - Parallel bank mode"]
pub type PARALLELBANK_R = crate::BitReader<bool>;
#[doc = "Field `PARALLELBANK` writer - Parallel bank mode"]
pub type PARALLELBANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `EOPIE` reader - End of programming interrupt enable"]
pub type EOPIE_R = crate::BitReader<bool>;
#[doc = "Field `EOPIE` writer - End of programming interrupt enable"]
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
#[doc = "Field `OBL_LAUNCH` reader - Launch the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader<bool>;
#[doc = "Field `OBL_LAUNCH` writer - Launch the option byte loading"]
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&self) -> PELOCK_R {
        PELOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&self) -> PRGLOCK_R {
        PRGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn ftdw(&self) -> FTDW_R {
        FTDW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&self) -> FPRG_R {
        FPRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&self) -> PARALLELBANK_R {
        PARALLELBANK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    #[must_use]
    pub fn pelock(&mut self) -> PELOCK_W<0> {
        PELOCK_W::new(self)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    #[must_use]
    pub fn prglock(&mut self) -> PRGLOCK_W<1> {
        PRGLOCK_W::new(self)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<2> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<3> {
        PROG_W::new(self)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<4> {
        DATA_W::new(self)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    #[must_use]
    pub fn ftdw(&mut self) -> FTDW_W<8> {
        FTDW_W::new(self)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<9> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    #[must_use]
    pub fn fprg(&mut self) -> FPRG_W<10> {
        FPRG_W::new(self)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    #[must_use]
    pub fn parallelbank(&mut self) -> PARALLELBANK_W<15> {
        PARALLELBANK_W::new(self)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<16> {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<17> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<18> {
        OBL_LAUNCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program/erase control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](index.html) module"]
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pecr::R](R) reader structure"]
impl crate::Readable for PECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pecr::W](W) writer structure"]
impl crate::Writable for PECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PECR to value 0x07"]
impl crate::Resettable for PECR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
