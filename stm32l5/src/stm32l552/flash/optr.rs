#[doc = "Register `OPTR` reader"]
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTR` writer"]
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDP` reader - Read protection level"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nRST_SHDW` reader - nRST_SHDW"]
pub type N_RST_SHDW_R = crate::BitReader<bool>;
#[doc = "Field `nRST_SHDW` writer - nRST_SHDW"]
pub type N_RST_SHDW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `IWDG_SW` reader - Independent watchdog selection"]
pub type IWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_SW` writer - Independent watchdog selection"]
pub type IWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `SWAP_BANK` reader - SWAP_BANK"]
pub type SWAP_BANK_R = crate::BitReader<bool>;
#[doc = "Field `SWAP_BANK` writer - SWAP_BANK"]
pub type SWAP_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `DB256K` reader - DB256K"]
pub type DB256K_R = crate::BitReader<bool>;
#[doc = "Field `DB256K` writer - DB256K"]
pub type DB256K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `DBANK` reader - DBANK"]
pub type DBANK_R = crate::BitReader<bool>;
#[doc = "Field `DBANK` writer - DBANK"]
pub type DBANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `SRAM2_PE` reader - SRAM2 parity check enable"]
pub type SRAM2_PE_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2_PE` writer - SRAM2 parity check enable"]
pub type SRAM2_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 Erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2_RST` writer - SRAM2 Erase when system reset"]
pub type SRAM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nSWBOOT0` reader - nSWBOOT0"]
pub type N_SWBOOT0_R = crate::BitReader<bool>;
#[doc = "Field `nSWBOOT0` writer - nSWBOOT0"]
pub type N_SWBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nBOOT0` reader - nBOOT0"]
pub type N_BOOT0_R = crate::BitReader<bool>;
#[doc = "Field `nBOOT0` writer - nBOOT0"]
pub type N_BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `PA15_PUPEN` reader - PA15_PUPEN"]
pub type PA15_PUPEN_R = crate::BitReader<bool>;
#[doc = "Field `PA15_PUPEN` writer - PA15_PUPEN"]
pub type PA15_PUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `TZEN` reader - TZEN"]
pub type TZEN_R = crate::BitReader<bool>;
#[doc = "Field `TZEN` writer - TZEN"]
pub type TZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - nRST_SHDW"]
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SWAP_BANK"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DB256K"]
    #[inline(always)]
    pub fn db256k(&self) -> DB256K_R {
        DB256K_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DBANK"]
    #[inline(always)]
    pub fn dbank(&self) -> DBANK_R {
        DBANK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - nSWBOOT0"]
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - nBOOT0"]
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PA15_PUPEN"]
    #[inline(always)]
    pub fn pa15_pupen(&self) -> PA15_PUPEN_R {
        PA15_PUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - TZEN"]
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<8> {
        BOR_LEV_W::new(self)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<12> {
        N_RST_STOP_W::new(self)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<13> {
        N_RST_STDBY_W::new(self)
    }
    #[doc = "Bit 14 - nRST_SHDW"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<14> {
        N_RST_SHDW_W::new(self)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<16> {
        IWDG_SW_W::new(self)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<17> {
        IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<18> {
        IWDG_STDBY_W::new(self)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<19> {
        WWDG_SW_W::new(self)
    }
    #[doc = "Bit 20 - SWAP_BANK"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<20> {
        SWAP_BANK_W::new(self)
    }
    #[doc = "Bit 21 - DB256K"]
    #[inline(always)]
    #[must_use]
    pub fn db256k(&mut self) -> DB256K_W<21> {
        DB256K_W::new(self)
    }
    #[doc = "Bit 22 - DBANK"]
    #[inline(always)]
    #[must_use]
    pub fn dbank(&mut self) -> DBANK_W<22> {
        DBANK_W::new(self)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<24> {
        SRAM2_PE_W::new(self)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<25> {
        SRAM2_RST_W::new(self)
    }
    #[doc = "Bit 26 - nSWBOOT0"]
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<26> {
        N_SWBOOT0_W::new(self)
    }
    #[doc = "Bit 27 - nBOOT0"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<27> {
        N_BOOT0_W::new(self)
    }
    #[doc = "Bit 28 - PA15_PUPEN"]
    #[inline(always)]
    #[must_use]
    pub fn pa15_pupen(&mut self) -> PA15_PUPEN_W<28> {
        PA15_PUPEN_W::new(self)
    }
    #[doc = "Bit 31 - TZEN"]
    #[inline(always)]
    #[must_use]
    pub fn tzen(&mut self) -> TZEN_W<31> {
        TZEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](index.html) module"]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optr::R](R) reader structure"]
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optr::W](W) writer structure"]
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTR to value 0"]
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
