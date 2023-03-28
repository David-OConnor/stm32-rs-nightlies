#[doc = "Register `OPTSR_CUR` reader"]
pub struct R(crate::R<OPTSR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTSR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTSR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTSR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTSR_CUR` writer"]
pub struct W(crate::W<OPTSR_CUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTSR_CUR_SPEC>;
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
impl From<crate::W<OPTSR_CUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTSR_CUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPT_BUSY` reader - Option byte change ongoing flag"]
pub type OPT_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `OPT_BUSY` writer - Option byte change ongoing flag"]
pub type OPT_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `BOR_LEV` reader - Brownout level option status bit"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` writer - Brownout level option status bit"]
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTSR_CUR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IWDG1_HW` reader - IWDG1 control option status bit"]
pub type IWDG1_HW_R = crate::BitReader<bool>;
#[doc = "Field `IWDG1_HW` writer - IWDG1 control option status bit"]
pub type IWDG1_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `nRST_STOP_D1` reader - D1 DStop entry reset option status bit"]
pub type N_RST_STOP_D1_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP_D1` writer - D1 DStop entry reset option status bit"]
pub type N_RST_STOP_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `nRST_STBY_D1` reader - D1 DStandby entry reset option status bit"]
pub type N_RST_STBY_D1_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STBY_D1` writer - D1 DStandby entry reset option status bit"]
pub type N_RST_STBY_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `RDP` reader - Readout protection level option status byte"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - Readout protection level option status byte"]
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTSR_CUR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FZ_IWDG_STOP` reader - IWDG Stop mode freeze option status bit"]
pub type FZ_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `FZ_IWDG_STOP` writer - IWDG Stop mode freeze option status bit"]
pub type FZ_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `FZ_IWDG_SDBY` reader - IWDG Standby mode freeze option status bit"]
pub type FZ_IWDG_SDBY_R = crate::BitReader<bool>;
#[doc = "Field `FZ_IWDG_SDBY` writer - IWDG Standby mode freeze option status bit"]
pub type FZ_IWDG_SDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `ST_RAM_SIZE` reader - DTCM RAM size option status"]
pub type ST_RAM_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST_RAM_SIZE` writer - DTCM RAM size option status"]
pub type ST_RAM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTSR_CUR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SECURITY` reader - Security enable option status bit"]
pub type SECURITY_R = crate::BitReader<bool>;
#[doc = "Field `SECURITY` writer - Security enable option status bit"]
pub type SECURITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `RSS1` reader - User option bit 1"]
pub type RSS1_R = crate::BitReader<bool>;
#[doc = "Field `RSS1` writer - User option bit 1"]
pub type RSS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `PERSO_OK` reader - Device personalization status bit"]
pub type PERSO_OK_R = crate::BitReader<bool>;
#[doc = "Field `PERSO_OK` writer - Device personalization status bit"]
pub type PERSO_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `IO_HSLV` reader - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
pub type IO_HSLV_R = crate::BitReader<bool>;
#[doc = "Field `IO_HSLV` writer - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
pub type IO_HSLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag"]
pub type OPTCHANGEERR_R = crate::BitReader<bool>;
#[doc = "Field `OPTCHANGEERR` writer - Option byte change error flag"]
pub type OPTCHANGEERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option status bit"]
pub type SWAP_BANK_OPT_R = crate::BitReader<bool>;
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option status bit"]
pub type SWAP_BANK_OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&self) -> OPT_BUSY_R {
        OPT_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - IWDG1 control option status bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&self) -> IWDG1_HW_R {
        IWDG1_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - D1 DStop entry reset option status bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&self) -> N_RST_STOP_D1_R {
        N_RST_STOP_D1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D1 DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&self) -> N_RST_STBY_D1_R {
        N_RST_STBY_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FZ_IWDG_STOP_R {
        FZ_IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FZ_IWDG_SDBY_R {
        FZ_IWDG_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - DTCM RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - User option bit 1"]
    #[inline(always)]
    pub fn rss1(&self) -> RSS1_R {
        RSS1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Device personalization status bit"]
    #[inline(always)]
    pub fn perso_ok(&self) -> PERSO_OK_R {
        PERSO_OK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    #[must_use]
    pub fn opt_busy(&mut self) -> OPT_BUSY_W<0> {
        OPT_BUSY_W::new(self)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<2> {
        BOR_LEV_W::new(self)
    }
    #[doc = "Bit 4 - IWDG1 control option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1_hw(&mut self) -> IWDG1_HW_W<4> {
        IWDG1_HW_W::new(self)
    }
    #[doc = "Bit 6 - D1 DStop entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop_d1(&mut self) -> N_RST_STOP_D1_W<6> {
        N_RST_STOP_D1_W::new(self)
    }
    #[doc = "Bit 7 - D1 DStandby entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stby_d1(&mut self) -> N_RST_STBY_D1_W<7> {
        N_RST_STBY_D1_W::new(self)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<8> {
        RDP_W::new(self)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg_stop(&mut self) -> FZ_IWDG_STOP_W<17> {
        FZ_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg_sdby(&mut self) -> FZ_IWDG_SDBY_W<18> {
        FZ_IWDG_SDBY_W::new(self)
    }
    #[doc = "Bits 19:20 - DTCM RAM size option status"]
    #[inline(always)]
    #[must_use]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<19> {
        ST_RAM_SIZE_W::new(self)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<21> {
        SECURITY_W::new(self)
    }
    #[doc = "Bit 26 - User option bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rss1(&mut self) -> RSS1_W<26> {
        RSS1_W::new(self)
    }
    #[doc = "Bit 28 - Device personalization status bit"]
    #[inline(always)]
    #[must_use]
    pub fn perso_ok(&mut self) -> PERSO_OK_W<28> {
        PERSO_OK_W::new(self)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    #[must_use]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<29> {
        IO_HSLV_W::new(self)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    #[must_use]
    pub fn optchangeerr(&mut self) -> OPTCHANGEERR_W<30> {
        OPTCHANGEERR_W::new(self)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<31> {
        SWAP_BANK_OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_cur](index.html) module"]
pub struct OPTSR_CUR_SPEC;
impl crate::RegisterSpec for OPTSR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optsr_cur::R](R) reader structure"]
impl crate::Readable for OPTSR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optsr_cur::W](W) writer structure"]
impl crate::Writable for OPTSR_CUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTSR_CUR to value 0"]
impl crate::Resettable for OPTSR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
